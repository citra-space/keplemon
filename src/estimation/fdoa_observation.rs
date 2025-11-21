use uuid::Uuid;

use super::ObservationType;
use crate::bodies::{Satellite, Sensor};
use crate::elements::{CartesianState, CartesianVector};
use crate::time::Epoch;
use pyo3::prelude::*;

// Speed of light in km/s
const SPEED_OF_LIGHT: f64 = 299792.458;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct FDOAObservation {
    id: String,
    sensor_1: Sensor,
    sensor_2: Sensor,
    epoch: Epoch,
    frequency_difference: f64,  // Hz, positive means sensor_2 observes higher frequency
    observer_1_teme_state: CartesianState,  // position and velocity
    observer_2_teme_state: CartesianState,  // position and velocity
    transmit_frequency: f64,  // Hz
    observed_satellite_id: Option<i32>,
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
        let w_vec = match self.sensor_1.get_fdoa_noise() {
            Some(noise) => vec![1.0 / noise.powi(2)],
            None => vec![0.0],  // No weight if noise not specified
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
                satellite.get_id(),
                self.epoch.to_iso()
            )),
        }
    }
}

#[pymethods]
impl FDOAObservation {
    #[new]
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

    #[getter]
    pub fn sensor_1(&self) -> Sensor {
        self.sensor_1.clone()
    }

    #[getter]
    pub fn sensor_2(&self) -> Sensor {
        self.sensor_2.clone()
    }

    #[getter]
    pub fn epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[getter]
    pub fn frequency_difference(&self) -> f64 {
        self.frequency_difference
    }

    #[getter]
    pub fn observer_1_teme_state(&self) -> CartesianState {
        self.observer_1_teme_state
    }

    #[getter]
    pub fn observer_2_teme_state(&self) -> CartesianState {
        self.observer_2_teme_state
    }

    #[getter]
    pub fn transmit_frequency(&self) -> f64 {
        self.transmit_frequency
    }

    #[getter]
    pub fn observed_satellite_id(&self) -> Option<i32> {
        self.observed_satellite_id
    }

    #[setter]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[setter]
    pub fn set_frequency_difference(&mut self, frequency_difference: f64) {
        self.frequency_difference = frequency_difference;
    }

    #[setter]
    pub fn set_observed_satellite_id(&mut self, observed_satellite_id: i32) {
        self.observed_satellite_id = Some(observed_satellite_id);
    }

    pub fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        self.compute_measurement_and_weight_vector()
    }

    pub fn get_predicted_vector(&self, satellite: &Satellite) -> PyResult<Vec<f64>> {
        self.compute_predicted_vector(satellite)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))
    }
}

impl ObservationType for FDOAObservation {
    fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        self.compute_measurement_and_weight_vector()
    }

    fn get_predicted_vector(&self, satellite: &Satellite) -> Result<Vec<f64>, String> {
        self.compute_predicted_vector(satellite)
    }

    fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    fn get_satellite_id(&self) -> Option<i32> {
        self.observed_satellite_id
    }
}
