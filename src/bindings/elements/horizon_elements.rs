use crate::elements::HorizonElements;
use pyo3::prelude::*;

#[pyclass(name = "HorizonElements")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyHorizonElements {
    inner: HorizonElements,
}
impl Copy for PyHorizonElements {}

impl From<HorizonElements> for PyHorizonElements {
    fn from(inner: HorizonElements) -> Self {
        Self { inner }
    }
}

impl From<PyHorizonElements> for HorizonElements {
    fn from(value: PyHorizonElements) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyHorizonElements {
    #[new]
    pub fn new(azimuth: f64, elevation: f64) -> Self {
        Self {
            inner: HorizonElements::new(azimuth, elevation),
        }
    }

    #[getter]
    pub fn get_xa_rae(&self) -> [f64; 6] {
        self.inner.into()
    }

    #[getter]
    pub fn get_azimuth(&self) -> f64 {
        self.inner.azimuth
    }

    #[getter]
    pub fn get_elevation(&self) -> f64 {
        self.inner.elevation
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.inner.range
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.inner.range_rate
    }

    #[getter]
    pub fn get_azimuth_rate(&self) -> Option<f64> {
        self.inner.azimuth_rate
    }

    #[getter]
    pub fn get_elevation_rate(&self) -> Option<f64> {
        self.inner.elevation_rate
    }

    #[setter]
    pub fn set_range(&mut self, range: Option<f64>) {
        self.inner.range = range;
    }

    #[setter]
    pub fn set_range_rate(&mut self, range_rate: Option<f64>) {
        self.inner.range_rate = range_rate;
    }

    #[setter]
    pub fn set_azimuth_rate(&mut self, azimuth_rate: Option<f64>) {
        self.inner.azimuth_rate = azimuth_rate;
    }

    #[setter]
    pub fn set_elevation_rate(&mut self, elevation_rate: Option<f64>) {
        self.inner.elevation_rate = elevation_rate;
    }

    #[setter]
    pub fn set_azimuth(&mut self, azimuth: f64) {
        self.inner.azimuth = azimuth;
    }

    #[setter]
    pub fn set_elevation(&mut self, elevation: f64) {
        self.inner.elevation = elevation;
    }
}
