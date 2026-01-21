use uuid::Uuid;

use super::{ObservationType, ObservationResidual};
use crate::bodies::{Satellite, Sensor};
use crate::elements::CartesianVector;
use crate::time::Epoch;

// Speed of light in km/s
const SPEED_OF_LIGHT: f64 = 299792.458;

#[derive(Debug, Clone, PartialEq)]
pub struct TDOAObservation {
    pub id: String,
    pub sensor_1: Sensor,
    pub sensor_2: Sensor,
    pub epoch: Epoch,
    pub time_difference: f64,  // seconds, positive means sensor_2 received later
    pub observer_1_teme_position: CartesianVector,
    pub observer_2_teme_position: CartesianVector,
    pub observed_satellite_id: Option<i32>,
}

impl TDOAObservation {
    /// Compute the predicted TDOA: (distance_2 - distance_1) / c
    pub(crate) fn compute_predicted_tdoa(
        sat_position: CartesianVector,
        sensor1_position: CartesianVector,
        sensor2_position: CartesianVector,
    ) -> f64 {
        let distance_1 = (sat_position - sensor1_position).get_magnitude();
        let distance_2 = (sat_position - sensor2_position).get_magnitude();
        (distance_2 - distance_1) / SPEED_OF_LIGHT
    }

    pub(crate) fn compute_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        let m_vec = vec![self.time_difference];

        // Combined noise for differential measurement: σ_combined² = σ_1² + σ_2²
        // Weight = 1/σ_combined² = 1/(σ_1² + σ_2²)
        let w_vec = match (self.sensor_1.tdoa_noise, self.sensor_2.tdoa_noise) {
            (Some(noise1), Some(noise2)) => {
                let variance_combined = noise1.powi(2) + noise2.powi(2);
                vec![1.0 / variance_combined]
            },
            (Some(noise1), None) => vec![1.0 / noise1.powi(2)],  // Fall back to sensor 1 only
            (None, Some(noise2)) => vec![1.0 / noise2.powi(2)],  // Fall back to sensor 2 only
            (None, None) => vec![0.0],  // No weight if neither sensor has noise specified
        };
        (m_vec, w_vec)
    }

    pub(crate) fn compute_predicted_vector(&self, satellite: &Satellite) -> Result<Vec<f64>, String> {
        match satellite.get_state_at_epoch(self.epoch) {
            Some(satellite_state) => {
                let predicted_tdoa = Self::compute_predicted_tdoa(
                    satellite_state.position,
                    self.observer_1_teme_position,
                    self.observer_2_teme_position,
                );
                Ok(vec![predicted_tdoa])
            }
            None => Err(format!(
                "Error propagating satellite {} to {}",
                satellite.id,
                self.epoch.to_iso()
            )),
        }
    }
}

impl TDOAObservation {
    pub fn new(
        sensor_1: Sensor,
        sensor_2: Sensor,
        epoch: Epoch,
        time_difference: f64,
        observer_1_teme_position: CartesianVector,
        observer_2_teme_position: CartesianVector,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sensor_1,
            sensor_2,
            epoch,
            time_difference,
            observer_1_teme_position,
            observer_2_teme_position,
            observed_satellite_id: None,
        }
    }
}

impl ObservationType for TDOAObservation {
    fn id(&self) -> &str {
        &self.id
    }

    fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        self.compute_measurement_and_weight_vector()
    }

    fn fill_predicted_vector(&self, satellite: &Satellite, out: &mut Vec<f64>) -> Result<(), String> {
        let vec = self.compute_predicted_vector(satellite)?;
        out.clear();
        out.extend_from_slice(&vec);
        Ok(())
    }

    fn get_predicted_vector(&self, satellite: &Satellite) -> Result<Vec<f64>, String> {
        self.compute_predicted_vector(satellite)
    }

    fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    fn get_satellite_id(&self) -> Option<String> {
        self.observed_satellite_id.map(|id| id.to_string())
    }

    fn get_residual(&self, satellite: &Satellite) -> Option<ObservationResidual> {
        // Compute predicted TDOA
        match self.compute_predicted_vector(satellite) {
            Ok(predicted_vec) => {
                let predicted_tdoa = predicted_vec[0];
                // Calculate residual: observed - predicted (in seconds)
                let time_residual = self.time_difference - predicted_tdoa;
                // Convert time difference to range difference (km)
                let range_residual = time_residual * SPEED_OF_LIGHT;
                // Return residual with only range component populated
                Some(ObservationResidual::with_range_only(range_residual))
            }
            Err(_) => None,
        }
    }
}
