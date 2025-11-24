use std::vec;
use uuid::Uuid;

use super::{ObservationResidual, ObservationType};
use crate::bodies::{Satellite, Sensor};
use crate::elements::{CartesianVector, TopocentricElements};
use crate::saal::{astro_func_interface, sat_state_interface};

use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    id: String,
    sensor: Sensor,
    epoch: Epoch,
    observed_teme_topocentric: TopocentricElements,
    observer_teme_position: CartesianVector,
    observed_satellite_id: Option<i32>,
}

impl Observation {
    pub fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        let mut m_vec = vec![self.get_right_ascension(), self.get_declination()];
        let mut w_vec = vec![
            1.0 / self.sensor.get_angular_noise().powi(2),
            1.0 / self.sensor.get_angular_noise().powi(2),
        ];
        if self.get_range().is_some() && self.sensor.get_range_noise().is_some() {
            m_vec.push(self.get_range().unwrap());
            w_vec.push(1.0 / self.sensor.get_range_noise().unwrap().powi(2));
        }
        if self.get_range_rate().is_some() && self.sensor.get_range_rate_noise().is_some() {
            m_vec.push(self.get_range_rate().unwrap());
            w_vec.push(1.0 / self.sensor.get_range_rate_noise().unwrap().powi(2));
        }
        if self.get_right_ascension_rate().is_some() && self.sensor.get_angular_rate_noise().is_some() {
            m_vec.push(self.get_right_ascension_rate().unwrap());
            w_vec.push(1.0 / self.sensor.get_angular_rate_noise().unwrap().powi(2));
        }
        if self.get_declination_rate().is_some() && self.sensor.get_angular_rate_noise().is_some() {
            m_vec.push(self.get_declination_rate().unwrap());
            w_vec.push(1.0 / self.sensor.get_angular_rate_noise().unwrap().powi(2));
        }
        (m_vec, w_vec)
    }

    pub fn get_predicted_vector(&self, satellite: &Satellite) -> Result<Vec<f64>, String> {
        match satellite.get_state_at_epoch(self.get_epoch()) {
            Some(satellite_state) => {
                let topo = astro_func_interface::teme_to_topocentric(self.observer_teme_position, satellite_state);
                let mut predicted = vec![topo.get_right_ascension(), topo.get_declination()];
                if self.get_range().is_some() {
                    predicted.push(topo.get_range().unwrap());
                }
                if self.get_range_rate().is_some() {
                    predicted.push(topo.get_range_rate().unwrap());
                }
                if self.get_right_ascension_rate().is_some() {
                    predicted.push(topo.get_right_ascension_rate().unwrap());
                }
                if self.get_declination_rate().is_some() {
                    predicted.push(topo.get_declination_rate().unwrap());
                }
                Ok(predicted)
            }
            None => Err(format!(
                "Error propagating satellite {} to {}",
                satellite.get_id(),
                self.get_epoch().to_iso()
            )),
        }
    }
}

#[pymethods]
impl Observation {
    #[new]
    pub fn new(
        sensor: Sensor,
        epoch: Epoch,
        observed_teme_topocentric: TopocentricElements,
        observer_teme_position: CartesianVector,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sensor,
            epoch,
            observed_teme_topocentric,
            observer_teme_position,
            observed_satellite_id: None,
        }
    }

    #[getter]
    pub fn get_sensor(&self) -> Sensor {
        self.sensor.clone()
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.observed_teme_topocentric.get_range()
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.observed_teme_topocentric.get_range_rate()
    }

    #[getter]
    pub fn get_right_ascension(&self) -> f64 {
        self.observed_teme_topocentric.get_right_ascension()
    }

    #[getter]
    pub fn get_declination(&self) -> f64 {
        self.observed_teme_topocentric.get_declination()
    }

    #[getter]
    pub fn get_right_ascension_rate(&self) -> Option<f64> {
        self.observed_teme_topocentric.get_right_ascension_rate()
    }

    #[getter]
    pub fn get_declination_rate(&self) -> Option<f64> {
        self.observed_teme_topocentric.get_declination_rate()
    }

    #[getter]
    pub fn get_observed_satellite_id(&self) -> Option<i32> {
        self.observed_satellite_id
    }

    #[setter]
    pub fn set_range(&mut self, range: Option<f64>) {
        self.observed_teme_topocentric.set_range(range);
    }

    #[setter]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[setter]
    pub fn set_range_rate(&mut self, range_rate: Option<f64>) {
        self.observed_teme_topocentric.set_range_rate(range_rate);
    }

    #[setter]
    pub fn set_right_ascension(&mut self, right_ascension: f64) {
        self.observed_teme_topocentric.set_right_ascension(right_ascension);
    }

    #[setter]
    pub fn set_declination(&mut self, declination: f64) {
        self.observed_teme_topocentric.set_declination(declination);
    }

    #[setter]
    pub fn set_observed_satellite_id(&mut self, observed_satellite_id: i32) {
        self.observed_satellite_id = Some(observed_satellite_id);
    }

    pub fn get_residual(&self, satellite: &Satellite) -> Option<ObservationResidual> {
        match satellite.get_state_at_epoch(self.epoch) {
            Some(satellite_state) => {
                let sensor_to_satellite = satellite_state.position - self.observer_teme_position;
                let teme_estimate = self.observer_teme_position
                    + (self.observed_teme_topocentric.get_observed_direction() * sensor_to_satellite.get_magnitude());

                let posvel_1 = [
                    satellite_state.position[0],
                    satellite_state.position[1],
                    satellite_state.position[2],
                    satellite_state.velocity[0],
                    satellite_state.velocity[1],
                    satellite_state.velocity[2],
                ];
                let posvel_2 = [
                    teme_estimate[0],
                    teme_estimate[1],
                    teme_estimate[2],
                    satellite_state.velocity[0],
                    satellite_state.velocity[1],
                    satellite_state.velocity[2],
                ];

                Some(ObservationResidual::from(sat_state_interface::get_relative_state(
                    &posvel_1,
                    &posvel_2,
                    self.epoch.days_since_1950,
                )))
            }
            None => None,
        }
    }
}

impl ObservationType for Observation {
    fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        Observation::get_measurement_and_weight_vector(self)
    }

    fn get_predicted_vector(&self, satellite: &Satellite) -> Result<Vec<f64>, String> {
        Observation::get_predicted_vector(self, satellite)
    }

    fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    fn get_satellite_id(&self) -> Option<i32> {
        self.observed_satellite_id
    }

    fn get_residual(&self, satellite: &Satellite) -> Option<ObservationResidual> {
        // Delegate to the existing inherent method
        Observation::get_residual(self, satellite)
    }
}
