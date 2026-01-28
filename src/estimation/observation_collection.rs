use super::{CollectionAssociationReport, Observation, ObservationAssociation};
use crate::bodies::{Constellation, Satellite};
use crate::elements::CartesianVector;
use crate::enums::AssociationConfidence;
use crate::time::Epoch;
use log;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ObservationCollection {
    id: String,
    epoch: Epoch,
    sensor_position: CartesianVector,
    sensor_direction: CartesianVector,
    observations: Vec<Observation>,
    field_of_view: f64,
}

impl ObservationCollection {
    pub fn new(obs: Vec<Observation>) -> Result<Self, String> {
        if obs.is_empty() {
            return Err("No observations provided".to_string());
        }

        let reference_position = obs[0].get_observer_position();
        let reference_epoch = obs[0].get_epoch();

        let mut unit_vectors: Vec<[f64; 3]> = Vec::with_capacity(obs.len());

        for observation in &obs {
            if observation.get_observer_position() != reference_position {
                return Err("Observer positions do not match".to_string());
            }
            if observation.get_epoch() != reference_epoch {
                return Err("Observation epochs do not match".to_string());
            }

            let ra = observation.get_right_ascension().to_radians();
            let dec = observation.get_declination().to_radians();
            unit_vectors.push([dec.cos() * ra.cos(), dec.cos() * ra.sin(), dec.sin()]);
        }

        let n = unit_vectors.len() as f64;
        let avg_x: f64 = unit_vectors.iter().map(|v| v[0]).sum::<f64>() / n;
        let avg_y: f64 = unit_vectors.iter().map(|v| v[1]).sum::<f64>() / n;
        let avg_z: f64 = unit_vectors.iter().map(|v| v[2]).sum::<f64>() / n;

        let sensor_direction = CartesianVector::new(avg_x, avg_y, avg_z);

        let mut max_angular_distance: f64 = 0.0;
        for uv in &unit_vectors {
            let dot = (uv[0] * sensor_direction.get_x()
                + uv[1] * sensor_direction.get_y()
                + uv[2] * sensor_direction.get_z())
            .clamp(-1.0, 1.0);
            let angular_distance = dot.acos().to_degrees();
            max_angular_distance = max_angular_distance.max(angular_distance);
        }

        let field_of_view = max_angular_distance * 2.0;

        log::debug!(
            "Created ObservationCollection with {} observations at {} in a {} deg field-of-view",
            obs.len(),
            reference_epoch.to_iso(),
            field_of_view
        );

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            epoch: reference_epoch,
            sensor_position: reference_position,
            sensor_direction,
            observations: obs,
            field_of_view,
        })
    }

    pub fn get_sensor_position(&self) -> CartesianVector {
        self.sensor_position
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_sensor_direction(&self) -> CartesianVector {
        self.sensor_direction
    }

    pub fn get_field_of_view(&self) -> f64 {
        self.field_of_view
    }

    pub fn get_observations(&self) -> &Vec<Observation> {
        &self.observations
    }

    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn get_visibility(&self, satellite: &Satellite) -> bool {
        if let Some(sat_state) = satellite.get_state_at_epoch(self.epoch) {
            self.is_state_visible(&sat_state)
        } else {
            false
        }
    }

    pub fn get_visibility_interpolated(&self, satellite: &Satellite) -> bool {
        if let Some(sat_state) = satellite.interpolate_state_at_epoch(self.epoch) {
            self.is_state_visible(&sat_state)
        } else {
            false
        }
    }

    fn is_state_visible(&self, sat_state: &crate::elements::CartesianState) -> bool {
        let sensor_to_sat = sat_state.position - self.sensor_position;
        let angle_from_bore = sensor_to_sat.angle(&self.sensor_direction);
        let max_angle = (self.field_of_view / 2.0).max(1.0);
        angle_from_bore.to_degrees() <= max_angle
    }

    pub fn get_association(&self, satellite: &Satellite) -> Option<ObservationAssociation> {
        if !self.get_visibility(satellite) {
            return None;
        }
        let mut best_association: Option<ObservationAssociation> = None;

        for ob in &self.observations {
            if let Some(association) = ob.get_association(satellite) {
                match &best_association {
                    Some(best) => {
                        if association.get_residual().get_range() < best.get_residual().get_range() {
                            best_association = Some(association);
                        }
                    }
                    None => {
                        best_association = Some(association);
                    }
                }
            }
        }
        best_association
    }

    pub fn get_list(obs: Vec<Observation>) -> Vec<Self> {
        let mut groups: HashMap<(Epoch, String), Vec<Observation>> = HashMap::new();

        for observation in obs {
            let key = (observation.get_epoch(), observation.get_sensor().id.clone());
            groups.entry(key).or_default().push(observation);
        }

        groups.into_values().filter_map(|group| Self::new(group).ok()).collect()
    }

    pub fn get_association_report(&self, satellites: &Constellation) -> CollectionAssociationReport {
        let mut associations: Vec<ObservationAssociation> = Vec::new();
        let mut moving_satellite_ids: HashSet<String> = HashSet::new();
        let mut associated_satellite_ids: HashSet<String> = HashSet::new();
        let mut associated_observation_ids: HashSet<String> = HashSet::new();

        // Pre-filter satellites to only those visible in the field of view
        let visible_satellites: HashMap<String, &Satellite> = satellites
            .get_satellites()
            .iter()
            .filter(|(_, sat)| self.get_visibility_interpolated(sat))
            .map(|(id, sat)| (id.clone(), sat))
            .collect();

        // Pre-compute all candidate associations with their states
        // Structure: (observation_id, satellite_id, association)
        let mut all_candidates: Vec<(String, String, ObservationAssociation)> = Vec::new();

        for observation in &self.observations {
            // First check observed_satellite_id if present
            if let Some(ref observed_id) = observation.observed_satellite_id
                && let Some(satellite) = visible_satellites.get(observed_id)
                && let Some(state) = satellite.interpolate_state_at_epoch(self.epoch)
                && let Some(association) = observation.get_association_from_state(observed_id, &state)
            {
                all_candidates.push((observation.id.clone(), observed_id.clone(), association));
            }

            // Then check all other visible satellites
            for (sat_id, satellite) in &visible_satellites {
                if observation.observed_satellite_id.as_ref() == Some(sat_id) {
                    continue;
                }

                if let Some(state) = satellite.interpolate_state_at_epoch(self.epoch)
                    && let Some(association) = observation.get_association_from_state(sat_id, &state)
                {
                    all_candidates.push((observation.id.clone(), sat_id.clone(), association));
                }
            }
        }

        // Process in order of confidence: High, then Medium, then Low
        // Within each confidence level, sort by residual range (best first)
        for target_confidence in [
            AssociationConfidence::High,
            AssociationConfidence::Medium,
            AssociationConfidence::Low,
        ] {
            // Filter candidates for this confidence level
            let mut level_candidates: Vec<&(String, String, ObservationAssociation)> = all_candidates
                .iter()
                .filter(|(obs_id, sat_id, assoc)| {
                    !associated_observation_ids.contains(obs_id)
                        && !associated_satellite_ids.contains(sat_id)
                        && assoc.get_confidence() == target_confidence
                })
                .collect();

            // Sort by residual range (smallest first = best match)
            level_candidates.sort_by(|a, b| {
                a.2.get_residual()
                    .get_range()
                    .partial_cmp(&b.2.get_residual().get_range())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            // Greedily assign best matches
            for (obs_id, sat_id, association) in level_candidates {
                if associated_observation_ids.contains(obs_id) || associated_satellite_ids.contains(sat_id) {
                    continue;
                }

                associated_observation_ids.insert(obs_id.clone());
                associated_satellite_ids.insert(sat_id.clone());

                // Only add to moving_satellite_ids for low/medium confidence
                if target_confidence != AssociationConfidence::High {
                    moving_satellite_ids.insert(sat_id.clone());
                }

                associations.push(association.clone());
            }
        }

        // Collect orphan observations (those without any association)
        let orphan_observations: Vec<Observation> = self
            .observations
            .iter()
            .filter(|obs| !associated_observation_ids.contains(&obs.id))
            .cloned()
            .collect();

        let moving_satellite_ids: Vec<String> = moving_satellite_ids.into_iter().collect();
        CollectionAssociationReport::new(self.id.clone(), orphan_observations, associations, moving_satellite_ids)
    }
}
