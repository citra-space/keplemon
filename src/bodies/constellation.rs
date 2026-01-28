use super::Satellite;
use crate::bodies::Observatory;
use crate::catalogs::TLECatalog;
use crate::configs;
#[cfg(feature = "cuda")]
use crate::elements::TLE;
use crate::elements::{CartesianState, Ephemeris, OrbitPlotData};
#[cfg(feature = "cuda")]
use crate::enums::KeplerianType;
use crate::estimation::{CollectionAssociationReport, Observation, ObservationCollection};
use crate::events::{
    CandidateAnalysis, CloseApproachReport, CrossTagEvidence, CrossTagReport, CrossTagResult, HorizonAccessReport,
    ManeuverEvent, ManeuverReport, ProximityReport,
};
use crate::propagation::{BatchPropagator, PropagationBackend};
use crate::time::{Epoch, TimeSpan};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Constellation {
    pub name: Option<String>,
    satellites: HashMap<String, Satellite>,
}

impl From<TLECatalog> for Constellation {
    fn from(catalog: TLECatalog) -> Self {
        let mut constellation = Constellation::new();
        for satellite_id in catalog.keys() {
            if let Some(tle) = catalog.get(satellite_id.clone()) {
                let sat = Satellite::from(tle);
                constellation.add(satellite_id, sat);
            }
        }
        constellation.name = catalog.name;
        constellation
    }
}

impl Constellation {
    pub fn get_satellites(&self) -> &HashMap<String, Satellite> {
        &self.satellites
    }

    pub fn new() -> Self {
        Constellation {
            name: None,
            satellites: HashMap::new(),
        }
    }

    pub fn get_states_at_epoch(&self, epoch: Epoch) -> HashMap<String, Option<CartesianState>> {
        self.satellites
            .iter()
            .map(|(satellite_id, sat)| {
                let state = sat.get_state_at_epoch(epoch);
                (satellite_id.clone(), state)
            })
            .collect()
    }

    pub fn get_plot_data(&self, start: Epoch, end: Epoch, step: TimeSpan) -> HashMap<String, OrbitPlotData> {
        self.satellites
            .iter()
            .filter_map(|(satellite_id, sat)| {
                sat.get_plot_data(start, end, step)
                    .map(|plot_data| (satellite_id.clone(), plot_data))
            })
            .collect()
    }

    pub fn step_to_epoch(&mut self, epoch: Epoch) -> Constellation {
        let sat_map = self
            .satellites
            .iter_mut()
            .filter_map(|(sat_id, sat)| match sat.step_to_epoch(epoch) {
                Ok(_) => Some((sat_id.clone(), sat.clone())),
                Err(_) => None,
            })
            .collect();
        let mut new_constellation = Constellation::new();
        new_constellation.satellites = sat_map;
        new_constellation.name = self.name.clone();
        new_constellation
    }

    pub fn get_horizon_access_report(
        &mut self,
        site: &Observatory,
        start: Epoch,
        end: Epoch,
        min_el: f64,
        min_duration: TimeSpan,
    ) -> HorizonAccessReport {
        let ephem_dt = min_duration * 0.5;
        // get TEME states for site
        let site_ephem = site.get_ephemeris(start, end, ephem_dt);

        // get TEME states for all satellites
        let sat_ephem_list: Vec<Ephemeris> = self.get_ephemeris_list(start, end, ephem_dt);

        // create empty report
        let mut report = HorizonAccessReport::new(start, end, min_el, min_duration);

        // parallelize the access report generation
        let num = sat_ephem_list.len();
        let accesses = (0..num)
            .into_par_iter()
            .filter_map(|i| {
                let sat_ephem = &sat_ephem_list[i];
                sat_ephem.get_horizon_accesses(&site_ephem, min_el, min_duration)
            })
            .collect::<Vec<_>>();

        report.set_accesses(accesses.into_iter().flatten().collect());
        report
    }

    pub fn get_ca_report_vs_one(
        &mut self,
        sat: &mut Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: f64,
    ) -> CloseApproachReport {
        match sat.get_ephemeris(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES)) {
            Some(ephemeris) => {
                let mut candidates = Constellation::new();
                candidates.satellites = self
                    .satellites
                    .iter_mut()
                    .filter_map(|(id, other_sat)| {
                        if sat.get_apoapsis()? < other_sat.get_periapsis()? - distance_threshold
                            || other_sat.get_apoapsis()? < sat.get_periapsis()? - distance_threshold
                            || sat.get_periapsis()? > other_sat.get_apoapsis()? + distance_threshold
                            || other_sat.get_periapsis()? > sat.get_apoapsis()? + distance_threshold
                            || sat.id == other_sat.id
                        {
                            return None;
                        }
                        Some((id.clone(), other_sat.clone()))
                    })
                    .collect();

                let candidate_ephem = candidates.get_ephemeris_list(
                    start,
                    end,
                    TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES),
                );
                let close_approaches = candidate_ephem
                    .par_iter()
                    .filter_map(|other_ephem| ephemeris.get_close_approach(other_ephem, distance_threshold))
                    .collect();
                let mut report = CloseApproachReport::new(start, end, distance_threshold);
                report.set_close_approaches(close_approaches);
                report
            }
            None => CloseApproachReport::new(start, end, distance_threshold),
        }
    }

    pub fn get_ca_report_vs_many(&mut self, start: Epoch, end: Epoch, distance_threshold: f64) -> CloseApproachReport {
        let mut report = CloseApproachReport::new(start, end, distance_threshold);
        let ephem_list: Vec<Ephemeris> =
            self.get_ephemeris_list(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES));
        let num = ephem_list.len();
        let close_approaches = (0..num)
            .into_par_iter()
            .flat_map(|i| {
                let pri_ephem = &ephem_list[i];
                let pri_sat = &self.satellites.get(&pri_ephem.get_satellite_id()).unwrap();
                (i + 1..num)
                    .into_par_iter()
                    .filter_map(|j| {
                        let sec_ephem = &ephem_list[j];
                        let sec_sat = &self.satellites.get(&sec_ephem.get_satellite_id()).unwrap();
                        if pri_sat.get_apoapsis()? < sec_sat.get_periapsis()? - distance_threshold
                            || sec_sat.get_apoapsis()? < pri_sat.get_periapsis()? - distance_threshold
                            || pri_sat.get_periapsis()? > sec_sat.get_apoapsis()? + distance_threshold
                            || sec_sat.get_periapsis()? > pri_sat.get_apoapsis()? + distance_threshold
                        {
                            return None;
                        }
                        pri_ephem.get_close_approach(sec_ephem, distance_threshold)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        report.set_close_approaches(close_approaches);
        report
    }

    pub fn get_proximity_report_vs_one(
        &mut self,
        sat: &mut Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: f64,
    ) -> ProximityReport {
        match sat.get_ephemeris(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES)) {
            Some(ephemeris) => {
                let candidate_ephem =
                    self.get_ephemeris_list(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES));
                let events = candidate_ephem
                    .par_iter()
                    .filter_map(|other_ephem| ephemeris.get_proximity_event(other_ephem, distance_threshold))
                    .collect();
                let mut report = ProximityReport::new(start, end, distance_threshold);
                report.set_events(events);
                report
            }
            None => ProximityReport::new(start, end, distance_threshold),
        }
    }

    pub fn get_proximity_report_vs_many(
        &mut self,
        start: Epoch,
        end: Epoch,
        distance_threshold: f64,
    ) -> ProximityReport {
        let mut report = ProximityReport::new(start, end, distance_threshold);
        let ephem_list: Vec<Ephemeris> =
            self.get_ephemeris_list(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES));
        let num = ephem_list.len();
        let events = (0..num)
            .into_par_iter()
            .flat_map(|i| {
                let pri_ephem = &ephem_list[i];
                (i + 1..num)
                    .into_par_iter()
                    .filter_map(|j| {
                        let sec_ephem = &ephem_list[j];
                        pri_ephem.get_proximity_event(sec_ephem, distance_threshold)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        report.set_events(events);
        report
    }

    pub fn detect_cross_tags(
        &mut self,
        uct: &mut Satellite,
        observations: &Vec<Observation>,
        start: Epoch,
        end: Epoch,
        proximity_threshold: Option<f64>,
        confidence_threshold: Option<f64>,
    ) -> CrossTagReport {
        let prox_threshold = proximity_threshold.unwrap_or(10.0);
        let conf_threshold = confidence_threshold.unwrap_or(0.75);

        // Step 1: Get proximity candidates
        let proximity_report = self.get_proximity_report_vs_one(uct, start, end, prox_threshold);

        if proximity_report.get_events().is_empty() {
            return CrossTagReport::new(
                uct.id.clone(),
                CrossTagResult::NoProximityFound,
                None,
                0.0,
                Vec::new(),
                "No proximity events found".to_string(),
                0,
                0,
                0,
                0,
                Vec::new(),
            );
        }

        // Step 2: Get unique candidate satellite IDs from proximity events
        let mut candidate_ids: Vec<String> = proximity_report
            .get_events()
            .iter()
            .map(|event| event.get_secondary_id())
            .collect();
        candidate_ids.sort();
        candidate_ids.dedup();

        // Step 3: Analyze each candidate separately
        let mut all_candidates: Vec<CandidateAnalysis> = Vec::new();
        let all_events = proximity_report.get_events();

        for candidate_id in candidate_ids.iter() {
            // Collect observations during this candidate's proximity windows
            let mut candidate_observations: Vec<Observation> = Vec::new();

            // Process events for this specific candidate
            for event in all_events.iter() {
                // Skip events that don't match this candidate
                if event.get_secondary_id() != *candidate_id {
                    continue;
                }

                let event_start = event.get_start_epoch();
                let event_end = event.get_end_epoch();

                for obs in observations {
                    let obs_epoch = obs.get_epoch();
                    if obs_epoch >= event_start && obs_epoch <= event_end {
                        candidate_observations.push(obs.clone());
                    }
                }
            }

            if candidate_observations.is_empty() {
                continue;
            }

            // Group observations by sensor/time
            let collections = ObservationCollection::get_list(candidate_observations.clone());

            // Analyze each collection for this candidate
            let mut evidence_list: Vec<CrossTagEvidence> = Vec::new();
            let mut cross_tag_votes = 0;
            let mut real_uct_votes = 0;

            for collection in collections.iter() {
                // Check observations against the approved constellation
                let report = collection.get_association_report(self);

                // Find associations for THIS candidate only
                let mut candidate_associations = Vec::new();
                for assoc in report.get_associations() {
                    if assoc.get_satellite_id() == *candidate_id {
                        candidate_associations.push(assoc.clone());
                    }
                }

                // Check visibility of UCT
                let uct_visible = collection.get_visibility(uct);

                let orphan_observations = report.get_orphan_observations();
                let orphan_count = orphan_observations.len();
                let candidate_matched = !candidate_associations.is_empty();

                // Determine conclusion
                let conclusion = if orphan_count > 0 && uct_visible {
                    // Orphans exist AND UCT was visible
                    // Both objects were likely present
                    real_uct_votes += 1;
                    "REAL_UCT".to_string()
                } else if orphan_count == 0 && candidate_matched {
                    // No orphans, candidate matched all observations
                    // UCT is likely just the candidate satellite
                    cross_tag_votes += 1;
                    "CROSS_TAG".to_string()
                } else {
                    "INCONCLUSIVE".to_string()
                };

                let evidence = CrossTagEvidence::new(
                    collection.get_epoch(),
                    collection.get_observations()[0].get_sensor().id.clone(),
                    orphan_count,
                    candidate_matched,
                    uct_visible,
                    conclusion,
                    candidate_associations,
                    orphan_observations.clone(),
                );
                evidence_list.push(evidence);
            }

            // Calculate result and confidence for this candidate
            let total_votes = cross_tag_votes + real_uct_votes;
            let inconclusive_votes = evidence_list.len() - total_votes;

            if total_votes > 0 {
                // Determine result: Any real_uct vote means they're different objects
                let result = if real_uct_votes > 0 {
                    CrossTagResult::RealUCT
                } else {
                    CrossTagResult::CrossTag
                };

                // Calculate confidence: fraction of conclusive votes supporting this result
                let confidence = match result {
                    CrossTagResult::RealUCT => real_uct_votes as f64 / total_votes as f64,
                    CrossTagResult::CrossTag => cross_tag_votes as f64 / total_votes as f64,
                    _ => 0.0,
                };

                all_candidates.push(CandidateAnalysis::new(
                    candidate_id.clone(),
                    result,
                    confidence,
                    real_uct_votes,
                    cross_tag_votes,
                    inconclusive_votes,
                    evidence_list.len(),
                    evidence_list,
                ));
            }
        }

        // Step 4: Sort candidates by confidence (descending) and select best
        all_candidates.sort_by(|a, b| b.get_confidence().partial_cmp(&a.get_confidence()).unwrap());

        if all_candidates.is_empty() {
            return CrossTagReport::new(
                uct.id.clone(),
                CrossTagResult::NoObservationsDuringProximity,
                None,
                0.0,
                Vec::new(),
                "No observations during proximity windows or insufficient evidence".to_string(),
                0,
                0,
                0,
                0,
                Vec::new(),
            );
        }

        // Best candidate is the first one (highest confidence)
        let best = &all_candidates[0];
        let result = best.get_result();
        let confidence = best.get_confidence();
        let real_uct_votes = best.get_real_uct_votes();
        let cross_tag_votes = best.get_cross_tag_votes();
        let inconclusive_votes = best.get_inconclusive_votes();
        let total_collections = best.get_total_collections_analyzed();
        let evidence = best.get_evidence();

        // Apply confidence threshold - filter out low-confidence results
        if confidence < conf_threshold {
            return CrossTagReport::new(
                uct.id.clone(),
                CrossTagResult::InsufficientEvidence,
                Some(best.get_candidate_id()),
                confidence,
                evidence,
                format!(
                    "Confidence {:.1}% below threshold {:.1}% ({} of {} conclusive votes) vs {}",
                    confidence * 100.0,
                    conf_threshold * 100.0,
                    match result {
                        CrossTagResult::RealUCT => real_uct_votes,
                        CrossTagResult::CrossTag => cross_tag_votes,
                        _ => 0,
                    },
                    real_uct_votes + cross_tag_votes,
                    best.get_candidate_id()
                ),
                total_collections,
                real_uct_votes,
                cross_tag_votes,
                inconclusive_votes,
                all_candidates,
            );
        }

        let reason = match result {
            CrossTagResult::RealUCT => format!(
                "Real UCT with {:.1}% confidence ({} of {} conclusive votes) vs {}",
                confidence * 100.0,
                real_uct_votes,
                real_uct_votes + cross_tag_votes,
                best.get_candidate_id()
            ),
            CrossTagResult::CrossTag => format!(
                "Cross-tag detected with {:.1}% confidence ({} of {} conclusive votes) - likely same as {}",
                confidence * 100.0,
                cross_tag_votes,
                real_uct_votes + cross_tag_votes,
                best.get_candidate_id()
            ),
            _ => "Unknown".to_string(),
        };

        CrossTagReport::new(
            uct.id.clone(),
            result,
            Some(best.get_candidate_id()),
            confidence,
            evidence,
            reason,
            total_collections,
            real_uct_votes,
            cross_tag_votes,
            inconclusive_votes,
            all_candidates,
        )
    }

    pub fn get_maneuver_events(
        &mut self,
        future_sats: &mut Constellation,
        start: Epoch,
        end: Epoch,
        distance_threshold: f64,
        velocity_threshold: f64,
    ) -> ManeuverReport {
        let mut report = ManeuverReport::new(start, end, distance_threshold, velocity_threshold);

        let current_ephem_map: HashMap<String, Ephemeris> = self
            .satellites
            .par_iter_mut()
            .filter_map(|(id, sat)| {
                sat.get_ephemeris(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES))
                    .map(|e| (id.clone(), e))
            })
            .collect();

        let future_ephem_map: HashMap<String, Ephemeris> = future_sats
            .satellites
            .par_iter_mut()
            .filter_map(|(id, sat)| {
                sat.get_ephemeris(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES))
                    .map(|e| (id.clone(), e))
            })
            .collect();

        let maneuvers: Vec<ManeuverEvent> = current_ephem_map
            .par_iter()
            .filter_map(|(id, current_ephem)| {
                let future_ephem = future_ephem_map.get(id)?;
                current_ephem.get_maneuver_event(future_ephem, distance_threshold, velocity_threshold)
            })
            .collect();

        report.set_maneuvers(maneuvers);
        report
    }

    pub fn get_ephemeris(
        &mut self,
        start_epoch: Epoch,
        end_epoch: Epoch,
        step_size: TimeSpan,
    ) -> HashMap<String, Option<Ephemeris>> {
        self.satellites
            .par_iter_mut()
            .map(|(satellite_id, sat)| {
                let ephemeris = sat.get_ephemeris(start_epoch, end_epoch, step_size);
                (satellite_id.clone(), ephemeris)
            })
            .collect()
    }

    pub fn get_ephemeris_list(&mut self, start: Epoch, end: Epoch, step: TimeSpan) -> Vec<Ephemeris> {
        self.satellites
            .iter_mut()
            .filter_map(|(_, sat)| sat.get_ephemeris(start, end, step))
            .collect()
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.satellites.keys().cloned().collect()
    }

    fn __setitem__(&mut self, satellite_id: String, state: Satellite) {
        self.satellites.insert(satellite_id, state);
    }

    pub fn add(&mut self, satellite_id: String, sat: Satellite) {
        self.satellites.insert(satellite_id, sat);
    }

    pub fn get(&self, satellite_id: String) -> Option<Satellite> {
        self.satellites.get(&satellite_id).cloned()
    }

    pub fn remove(&mut self, satellite_id: String) {
        self.satellites.remove(&satellite_id);
    }

    pub fn clear(&mut self) {
        self.satellites.clear();
    }

    pub fn get_count(&self) -> usize {
        self.satellites.len()
    }

    pub fn cache_ephemeris(&mut self, start: Epoch, end: Epoch, step: TimeSpan) {
        self.satellites.par_iter_mut().for_each(|(_, sat)| {
            let _ = sat.get_ephemeris(start, end, step);
        });
    }

    pub fn get_association_reports(
        &self,
        collections: &Vec<ObservationCollection>,
    ) -> Vec<CollectionAssociationReport> {
        collections
            .par_iter()
            .map(|collection| collection.get_association_report(self))
            .collect()
    }
}

impl Constellation {
    /// Get states at multiple epochs using batch propagation (GPU-accelerated when available)
    ///
    /// This method automatically selects GPU or CPU backend based on problem size.
    /// For large constellations and many time points, GPU acceleration can provide
    /// significant speedups over serial CPU propagation.
    ///
    /// # Arguments
    /// * `epochs` - Vector of epochs to propagate to
    /// * `backend` - Optional backend selection (defaults to Auto)
    ///
    /// # Returns
    /// HashMap mapping satellite ID to vector of states (one per epoch)
    pub fn get_states_at_epochs(
        &self,
        epochs: &[Epoch],
        backend: Option<PropagationBackend>,
    ) -> HashMap<String, Vec<Option<CartesianState>>> {
        let backend = backend.unwrap_or(PropagationBackend::Auto);
        let propagator = BatchPropagator::new().set_backend(backend);

        let n_sats = self.satellites.len();
        let n_times = epochs.len();

        let _selected = propagator.select_backend(n_sats, n_times);

        // Try batch GPU propagation if selected
        #[cfg(feature = "cuda")]
        if matches!(_selected, crate::propagation::SelectedBackend::Gpu) {
            log::info!(
                "Using GPU batch propagation for {} satellites × {} epochs",
                n_sats,
                n_times
            );

            // Collect TLEs from satellites
            let tles: Vec<TLE> = self.satellites.values().map(|sat| TLE::from(sat.clone())).collect();

            // If we successfully got TLEs for all satellites, check if all are GP-only
            if tles.len() == n_sats {
                // Check if all TLEs are GP-only (MeanKozaiGP or MeanBrouwerGP)
                // GPU propagator only supports type 0 (SGP) and type 2 (SGP4) TLEs
                let all_gp_only = tles.iter().all(|tle| {
                    matches!(
                        tle.get_type(),
                        KeplerianType::MeanKozaiGP | KeplerianType::MeanBrouwerGP
                    )
                });

                if !all_gp_only {
                    log::warn!("Not all TLEs are GP-only (SGP/SGP4), falling back to CPU propagation");
                } else if let Ok(batch_results) = propagator.propagate_batch(&tles, epochs) {
                    // Map results back to satellite IDs
                    let sat_ids: Vec<String> = self.satellites.keys().cloned().collect();
                    return sat_ids
                        .iter()
                        .zip(batch_results.into_iter())
                        .map(|(sat_id, states)| (sat_id.clone(), states.into_iter().map(Some).collect()))
                        .collect();
                } else {
                    log::warn!("GPU batch propagation failed, falling back to CPU");
                }
            } else {
                log::warn!(
                    "Not all satellites have TLEs ({}/{}), falling back to serial propagation",
                    tles.len(),
                    n_sats
                );
            }
        }

        // CPU path: propagate each satellite independently
        self.satellites
            .iter()
            .map(|(sat_id, sat)| {
                let states: Vec<Option<CartesianState>> =
                    epochs.iter().map(|&epoch| sat.get_state_at_epoch(epoch)).collect();
                (sat_id.clone(), states)
            })
            .collect()
    }

    /// Get ephemeris for all satellites using batch propagation
    ///
    /// This is a convenience wrapper around get_states_at_epochs that generates
    /// the epoch list automatically.
    pub fn get_batch_ephemeris(
        &self,
        start: Epoch,
        end: Epoch,
        step: TimeSpan,
        backend: Option<PropagationBackend>,
    ) -> HashMap<String, Vec<Option<CartesianState>>> {
        let epochs: Vec<Epoch> = std::iter::successors(Some(start), |&current| {
            let next = current + step;
            (next <= end).then_some(next)
        })
        .collect();

        self.get_states_at_epochs(&epochs, backend)
    }

    /// Check if GPU acceleration is available
    pub fn is_gpu_available() -> bool {
        BatchPropagator::new().is_gpu_available()
    }
}

#[cfg(test)]
mod tests {
    use super::{Constellation, Observatory};
    use crate::catalogs::TLECatalog;
    use crate::enums::TimeSystem;
    use crate::time::{Epoch, TimeSpan};
    use approx::assert_abs_diff_eq;
    use std::collections::HashMap;
    use std::path::Path;
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn load_catalog(path: &str) -> TLECatalog {
        TLECatalog::from_tle_file(path).expect("failed to load TLE catalog")
    }

    #[test]
    fn test_from_tle_catalog() {
        let _guard = TEST_LOCK.lock().expect("test lock poisoned");
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
        let celestrak_3le_path = base.join("2025-04-15-celestrak.3le");
        let space_track_3le_path = base.join("2025-04-15-space-track.3le");
        let celestrak_tle_path = base.join("2025-04-15-celestrak.tle");
        let space_track_tle_path = base.join("2025-04-15-space-track.tle");

        let celestrak_3le_sats = Constellation::from(load_catalog(celestrak_3le_path.to_str().unwrap()));
        let space_track_3le_sats = Constellation::from(load_catalog(space_track_3le_path.to_str().unwrap()));
        let celestrak_tle_sats = Constellation::from(load_catalog(celestrak_tle_path.to_str().unwrap()));
        let space_track_tle_sats = Constellation::from(load_catalog(space_track_tle_path.to_str().unwrap()));

        assert_eq!(space_track_3le_sats.get_satellites().len(), 27_485);
        assert_eq!(celestrak_3le_sats.get_satellites().len(), 11_304);
        assert_eq!(space_track_tle_sats.get_satellites().len(), 27_485);
        assert_eq!(celestrak_tle_sats.get_satellites().len(), 11_305);

        assert_eq!(
            space_track_3le_sats.name.as_deref(),
            Some(space_track_3le_path.to_str().unwrap())
        );
        assert_eq!(
            celestrak_3le_sats.name.as_deref(),
            Some(celestrak_3le_path.to_str().unwrap())
        );
        assert_eq!(
            space_track_tle_sats.name.as_deref(),
            Some(space_track_tle_path.to_str().unwrap())
        );
        assert_eq!(
            celestrak_tle_sats.name.as_deref(),
            Some(celestrak_tle_path.to_str().unwrap())
        );
    }

    #[test]
    fn test_get_horizon_access_report() {
        let _guard = TEST_LOCK.lock().expect("test lock poisoned");
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/2025-04-15-celestrak.3le");
        let mut sats = Constellation::from(load_catalog(path.to_str().unwrap()));
        let observatory = Observatory::new(1.0, 2.0, 0.003);

        let start = Epoch::from_iso("2025-04-15T00:00:00.000000Z", TimeSystem::UTC);
        let end = start + TimeSpan::from_minutes(60.0);
        let report = sats.get_horizon_access_report(&observatory, start, end, 10.0, TimeSpan::from_minutes(10.0));

        assert_eq!(report.get_accesses().len(), 365);
    }

    #[test]
    fn test_get_ca_report_vs_many() {
        let _guard = TEST_LOCK.lock().expect("test lock poisoned");
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/2025-04-15-ca.3le");
        let mut sats = Constellation::from(load_catalog(path.to_str().unwrap()));
        let start = Epoch::from_iso("2025-04-15T00:00:00.000000Z", TimeSystem::UTC);
        let end = start + TimeSpan::from_minutes(5.0);
        let report = sats.get_ca_report_vs_many(start, end, 1.0);

        let mut expected: HashMap<String, HashMap<String, f64>> = HashMap::new();
        expected.insert(
            "TANDEM-X".to_string(),
            HashMap::from([("TERRASAR-X".to_string(), 0.049)]),
        );
        expected.insert(
            "SHIJIAN-6 05A (SJ-6 05A)".to_string(),
            HashMap::from([("STARLINK-5893".to_string(), 0.672)]),
        );
        expected.insert(
            "STARLINK-4043".to_string(),
            HashMap::from([("QB50P2".to_string(), 0.902)]),
        );
        expected.insert(
            "TERRASAR-X".to_string(),
            HashMap::from([("TANDEM-X".to_string(), 0.049)]),
        );
        expected.insert(
            "STARLINK-5893".to_string(),
            HashMap::from([("SHIJIAN-6 05A (SJ-6 05A)".to_string(), 0.672)]),
        );
        expected.insert(
            "QB50P2".to_string(),
            HashMap::from([("STARLINK-4043".to_string(), 0.902)]),
        );

        let close_approaches = report.get_close_approaches();
        assert_eq!(close_approaches.len(), 3);
        for ca in close_approaches {
            let primary_id = ca.get_primary_id();
            let secondary_id = ca.get_secondary_id();
            let primary_name = sats
                .get(primary_id)
                .and_then(|sat| sat.name)
                .expect("missing primary name");
            let secondary_name = sats
                .get(secondary_id)
                .and_then(|sat| sat.name)
                .expect("missing secondary name");
            let distance = ca.get_distance();
            assert!(expected.contains_key(&primary_name));
            let secondary_map = expected.get(&primary_name).expect("missing secondary map");
            let expected_distance = secondary_map.get(&secondary_name).expect("missing expected distance");
            assert_abs_diff_eq!(distance, *expected_distance, epsilon = 1e-3);
        }
    }
}
