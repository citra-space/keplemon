use uuid::Uuid;

use super::ObservationType;
use crate::bodies::{Satellite, Sensor};
use crate::elements::CartesianVector;
use crate::time::Epoch;
use pyo3::prelude::*;

// Speed of light in km/s
const SPEED_OF_LIGHT: f64 = 299792.458;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct TDOAObservation {
    id: String,
    sensor_1: Sensor,
    sensor_2: Sensor,
    epoch: Epoch,
    time_difference: f64,  // seconds, positive means sensor_2 received later
    observer_1_teme_position: CartesianVector,
    observer_2_teme_position: CartesianVector,
    observed_satellite_id: Option<i32>,
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
        let w_vec = match self.sensor_1.get_tdoa_noise() {
            Some(noise) => vec![1.0 / noise.powi(2)],
            None => vec![0.0],  // No weight if noise not specified
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
                satellite.get_id(),
                self.epoch.to_iso()
            )),
        }
    }
}

#[pymethods]
impl TDOAObservation {
    #[new]
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
    pub fn time_difference(&self) -> f64 {
        self.time_difference
    }

    #[getter]
    pub fn observer_1_teme_position(&self) -> CartesianVector {
        self.observer_1_teme_position
    }

    #[getter]
    pub fn observer_2_teme_position(&self) -> CartesianVector {
        self.observer_2_teme_position
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
    pub fn set_time_difference(&mut self, time_difference: f64) {
        self.time_difference = time_difference;
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

impl ObservationType for TDOAObservation {
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
