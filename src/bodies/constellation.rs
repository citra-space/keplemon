use super::Satellite;
use crate::bodies::Observatory;
use crate::catalogs::TLECatalog;
use crate::configs;
use crate::elements::{CartesianState, Ephemeris, OrbitPlotData};
use crate::events::{CloseApproachReport, HorizonAccessReport};
use crate::time::{Epoch, TimeSpan};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
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
            .par_iter()
            .map(|(satellite_id, sat)| {
                let state = sat.get_state_at_epoch(epoch);
                (satellite_id.clone(), state)
            })
            .collect()
    }

    pub fn get_plot_data(&self, start: Epoch, end: Epoch, step: TimeSpan) -> HashMap<String, OrbitPlotData> {
        self.satellites
            .par_iter()
            .filter_map(|(satellite_id, sat)| {
                sat.get_plot_data(start, end, step)
                    .map(|plot_data| (satellite_id.clone(), plot_data))
            })
            .collect()
    }

    pub fn step_to_epoch(&mut self, epoch: Epoch) -> Constellation {
        let sat_map = self
            .satellites
            .par_iter_mut()
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
        // get TEME states for site
        let site_ephem = site.get_ephemeris(start, end, min_duration);

        // get TEME states for all satellites
        let sat_ephem_list: Vec<Ephemeris> = self
            .satellites
            .par_iter_mut()
            .filter_map(|(_, sat)| sat.get_ephemeris(start, end, min_duration))
            .collect();

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
                let close_approaches = self
                    .satellites
                    .par_iter_mut()
                    .filter_map(|(_, other_sat)| {
                        if sat.get_apoapsis()? < other_sat.get_periapsis()? - distance_threshold
                            || other_sat.get_apoapsis()? < sat.get_periapsis()? - distance_threshold
                            || sat.get_periapsis()? > other_sat.get_apoapsis()? + distance_threshold
                            || other_sat.get_periapsis()? > sat.get_apoapsis()? + distance_threshold
                        {
                            return None;
                        }
                        match other_sat.get_ephemeris(
                            start,
                            end,
                            TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES),
                        ) {
                            Some(other_ephemeris) => ephemeris.get_close_approach(&other_ephemeris, distance_threshold),

                            None => None,
                        }
                    })
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
        let ephem_list: Vec<Ephemeris> = self
            .satellites
            .par_iter_mut()
            .filter_map(|(_, sat)| {
                sat.get_ephemeris(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES))
            })
            .collect();
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
}

#[cfg(test)]
mod tests {
    use super::Constellation;
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
