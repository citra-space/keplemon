use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeodeticPosition {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

#[pymethods]
impl GeodeticPosition {
    #[new]
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
        }
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
}
