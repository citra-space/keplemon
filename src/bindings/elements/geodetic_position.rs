use crate::elements::GeodeticPosition;
use pyo3::prelude::*;

#[pyclass(name = "GeodeticPosition")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyGeodeticPosition {
    inner: GeodeticPosition,
}

impl From<GeodeticPosition> for PyGeodeticPosition {
    fn from(inner: GeodeticPosition) -> Self {
        Self { inner }
    }
}

impl From<PyGeodeticPosition> for GeodeticPosition {
    fn from(value: PyGeodeticPosition) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyGeodeticPosition {
    #[new]
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        GeodeticPosition::new(latitude, longitude, altitude).into()
    }

    #[getter]
    pub fn get_latitude(&self) -> f64 {
        self.inner.latitude
    }

    #[getter]
    pub fn get_longitude(&self) -> f64 {
        self.inner.longitude
    }

    #[getter]
    pub fn get_altitude(&self) -> f64 {
        self.inner.altitude
    }
}
