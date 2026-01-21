use uuid::Uuid;

use super::{ObservationType, ObservationResidual};
use crate::bodies::{Satellite, Sensor};
use crate::elements::{CartesianState, CartesianVector};
use crate::time::Epoch;

// Speed of light in km/s
const SPEED_OF_LIGHT: f64 = 299792.458;

#[derive(Debug, Clone, PartialEq)]
pub struct FDOAObservation {
    pub id: String,
    pub sensor_1: Sensor,
    pub sensor_2: Sensor,
    pub epoch: Epoch,
    pub frequency_difference: f64,  // Hz, positive means sensor_2 observes higher frequency
    pub observer_1_teme_state: CartesianState,  // position and velocity
    pub observer_2_teme_state: CartesianState,  // position and velocity
    pub transmit_frequency: f64,  // Hz
    pub observed_satellite_id: Option<i32>,
}

impl FDOAObservation {
    /// Compute the predicted FDOA based on Doppler difference
    /// FDOA = (f0/c) * [Doppler_2 - Doppler_1]
    /// where Doppler_i = (r_sat - r_sensor_i) · v_rel_i / ||r_sat - r_sensor_i||
    pub(crate) fn compute_predicted_fdoa(
        sat_position: CartesianVector,
        sat_velocity: CartesianVector,
        sensor1_position: CartesianVector,
        sensor1_velocity: CartesianVector,
        sensor2_position: CartesianVector,
        sensor2_velocity: CartesianVector,
        transmit_frequency: f64,
    ) -> f64 {
        // Vector from sensor to satellite
        let vec_sat_1 = sat_position - sensor1_position;
        let vec_sat_2 = sat_position - sensor2_position;

        let dist_1 = vec_sat_1.get_magnitude();
        let dist_2 = vec_sat_2.get_magnitude();

        if dist_1 < 1e-6 || dist_2 < 1e-6 {
            return 0.0;
        }

        // Relative velocities
        let v_rel_1 = sat_velocity - sensor1_velocity;
        let v_rel_2 = sat_velocity - sensor2_velocity;

        // Unit vectors from sensors to satellite
        let unit_sat_1 = vec_sat_1 * (1.0 / dist_1);
        let unit_sat_2 = vec_sat_2 * (1.0 / dist_2);

        // Doppler components: (r_sat - r_sensor) · v_rel / ||r_sat - r_sensor||
        let doppler_1 = unit_sat_1.dot(&v_rel_1);
        let doppler_2 = unit_sat_2.dot(&v_rel_2);

        // FDOA = (f0/c) * (doppler_2 - doppler_1)
        (transmit_frequency / SPEED_OF_LIGHT) * (doppler_2 - doppler_1)
    }

    pub(crate) fn compute_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        let m_vec = vec![self.frequency_difference];

        // Combined noise for differential measurement: σ_combined² = σ_1² + σ_2²
        // Weight = 1/σ_combined² = 1/(σ_1² + σ_2²)
        let w_vec = match (self.sensor_1.fdoa_noise, self.sensor_2.fdoa_noise) {
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
                let predicted_fdoa = Self::compute_predicted_fdoa(
                    satellite_state.position,
                    satellite_state.velocity,
                    self.observer_1_teme_state.position,
                    self.observer_1_teme_state.velocity,
                    self.observer_2_teme_state.position,
                    self.observer_2_teme_state.velocity,
                    self.transmit_frequency,
                );
                Ok(vec![predicted_fdoa])
            }
            None => Err(format!(
                "Error propagating satellite {} to {}",
                satellite.id,
                self.epoch.to_iso()
            )),
        }
    }
}

impl FDOAObservation {
    pub fn new(
        sensor_1: Sensor,
        sensor_2: Sensor,
        epoch: Epoch,
        frequency_difference: f64,
        observer_1_teme_state: CartesianState,
        observer_2_teme_state: CartesianState,
        transmit_frequency: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sensor_1,
            sensor_2,
            epoch,
            frequency_difference,
            observer_1_teme_state,
            observer_2_teme_state,
            transmit_frequency,
            observed_satellite_id: None,
        }
    }
}

impl ObservationType for FDOAObservation {
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
        // Compute predicted FDOA
        match self.compute_predicted_vector(satellite) {
            Ok(predicted_vec) => {
                let predicted_fdoa = predicted_vec[0];
                // Calculate residual: observed - predicted (in Hz)
                let frequency_residual = self.frequency_difference - predicted_fdoa;
                // Convert frequency difference to range rate difference (km/s)
                // FDOA = (f0/c) * doppler, so: doppler = FDOA * c / f0
                // range_rate = doppler, so: range_rate_residual = frequency_residual * c / f0
                let range_rate_residual = frequency_residual * SPEED_OF_LIGHT / self.transmit_frequency;
                // Return residual with only radial_velocity component populated
                Some(ObservationResidual::with_radial_velocity_only(range_rate_residual))
            }
            Err(_) => None,
        }
    }
}
