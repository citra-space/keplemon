use crate::bodies::Sensor;
use pyo3::prelude::*;

#[pyclass(name = "Sensor")]
#[derive(Debug, Clone, PartialEq)]
pub struct PySensor {
    inner: Sensor,
}

impl From<Sensor> for PySensor {
    fn from(inner: Sensor) -> Self {
        Self { inner }
    }
}

impl From<PySensor> for Sensor {
    fn from(value: PySensor) -> Self {
        value.inner
    }
}

#[pymethods]
impl PySensor {
    #[new]
    pub fn new(angular_noise: f64) -> Self {
        Sensor::new(angular_noise).into()
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.inner.id.clone()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[getter]
    pub fn get_angular_noise(&self) -> f64 {
        self.inner.angular_noise
    }

    #[getter]
    pub fn get_range_noise(&self) -> Option<f64> {
        self.inner.range_noise
    }

    #[getter]
    pub fn get_range_rate_noise(&self) -> Option<f64> {
        self.inner.range_rate_noise
    }

    #[getter]
    pub fn get_angular_rate_noise(&self) -> Option<f64> {
        self.inner.angular_rate_noise
    }

    #[setter]
    pub fn set_range_noise(&mut self, range_noise: f64) {
        self.inner.range_noise = Some(range_noise);
    }

    #[setter]
    pub fn set_range_rate_noise(&mut self, range_rate_noise: f64) {
        self.inner.range_rate_noise = Some(range_rate_noise);
    }

    #[setter]
    pub fn set_angular_rate_noise(&mut self, angular_rate_noise: f64) {
        self.inner.angular_rate_noise = Some(angular_rate_noise);
    }

    #[getter]
    pub fn get_tdoa_noise(&self) -> Option<f64> {
        self.inner.tdoa_noise
    }

    #[setter]
    pub fn set_tdoa_noise(&mut self, tdoa_noise: f64) {
        self.inner.tdoa_noise = Some(tdoa_noise);
    }

    #[getter]
    pub fn get_fdoa_noise(&self) -> Option<f64> {
        self.inner.fdoa_noise
    }

    #[setter]
    pub fn set_fdoa_noise(&mut self, fdoa_noise: f64) {
        self.inner.fdoa_noise = Some(fdoa_noise);
    }
}
