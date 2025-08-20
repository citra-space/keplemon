use super::Sensor;
use crate::elements::{CartesianState, CartesianVector, Ephemeris};
use crate::enums::ReferenceFrame;
use crate::saal::astro_func_interface;
use crate::time::{Epoch, TimeSpan};
use pyo3::prelude::*;
use uuid::Uuid;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct Observatory {
    site_id: String,
    name: String,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    sensors: Vec<Sensor>,
}

impl Observatory {
    pub fn get_ephemeris(&self, start_epoch: Epoch, end_epoch: Epoch, step: TimeSpan) -> Ephemeris {
        let ephemeris = Ephemeris::new(self.site_id.clone(), self.get_state_at_epoch(start_epoch));
        let mut next_epoch: Epoch = start_epoch + step;
        while next_epoch <= end_epoch {
            ephemeris.add_state(self.get_state_at_epoch(next_epoch));
            next_epoch += step;
        }
        ephemeris
    }
}

#[pymethods]
impl Observatory {
    #[new]
    pub fn new(name: String, latitude: f64, longitude: f64, altitude: f64) -> Self {
        Self {
            site_id: Uuid::new_v4().to_string(),
            name,
            latitude,
            longitude,
            altitude,
            sensors: Vec::new(),
        }
    }

    #[getter]
    pub fn get_site_id(&self) -> String {
        self.site_id.clone()
    }

    #[getter]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[getter]
    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    #[getter]
    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    #[getter]
    pub fn get_altitude(&self) -> f64 {
        self.altitude
    }

    #[getter]
    pub fn get_sensors(&self) -> Vec<Sensor> {
        self.sensors.clone()
    }

    #[setter]
    pub fn set_site_id(&mut self, site_id: String) {
        self.site_id = site_id;
    }

    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[setter]
    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    #[setter]
    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    #[setter]
    pub fn set_altitude(&mut self, altitude: f64) {
        self.altitude = altitude;
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    pub fn get_state_at_epoch(&self, epoch: Epoch) -> CartesianState {
        let teme_pos = astro_func_interface::lla_to_teme_position(
            epoch.days_since_1950,
            &[self.latitude, self.longitude, self.altitude],
        );
        CartesianState::new(
            epoch,
            CartesianVector::from(teme_pos),
            CartesianVector::from([0.0, 0.0, 0.0]),
            ReferenceFrame::TEME,
        )
    }
}
