use super::{Observation, ObservationAssociation};
use crate::bodies::Satellite;
use crate::elements::CartesianVector;
use crate::time::Epoch;
use log;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ObservationCollection {
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
            let sensor_to_sat = sat_state.position - self.sensor_position;
            let angle_from_bore = sensor_to_sat.angle(&self.sensor_direction);
            let max_angle = (self.field_of_view / 2.0).max(1.0);
            angle_from_bore.to_degrees() <= max_angle
        } else {
            false
        }
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
        let mut groups: HashMap<(Epoch, CartesianVector), Vec<Observation>> = HashMap::new();

        for observation in obs {
            let key = (observation.get_epoch(), observation.get_observer_position());
            groups.entry(key).or_default().push(observation);
        }

        groups.into_values().filter_map(|group| Self::new(group).ok()).collect()
    }
}
