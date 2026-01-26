use super::PyCartesianVector;
use crate::bindings::time::PyEpoch;
use crate::elements::TopocentricElements;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "TopocentricElements")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyTopocentricElements {
    inner: TopocentricElements,
}

impl Copy for PyTopocentricElements {}

impl From<TopocentricElements> for PyTopocentricElements {
    fn from(inner: TopocentricElements) -> Self {
        Self { inner }
    }
}

impl From<PyTopocentricElements> for TopocentricElements {
    fn from(value: PyTopocentricElements) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyTopocentricElements {
    #[new]
    pub fn new(right_ascension: f64, declination: f64) -> Self {
        Self {
            inner: TopocentricElements::new(right_ascension, declination),
        }
    }

    #[staticmethod]
    fn from_j2000(epoch: PyEpoch, right_ascension: f64, declination: f64) -> Self {
        let epoch: Epoch = epoch.into();
        Self {
            inner: TopocentricElements::from_j2000(epoch, right_ascension, declination),
        }
    }

    #[getter]
    pub fn get_right_ascension(&self) -> f64 {
        self.inner.right_ascension
    }

    #[getter]
    pub fn get_declination(&self) -> f64 {
        self.inner.declination
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
    pub fn get_right_ascension_rate(&self) -> Option<f64> {
        self.inner.right_ascension_rate
    }

    #[getter]
    pub fn get_declination_rate(&self) -> Option<f64> {
        self.inner.declination_rate
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
    pub fn set_right_ascension_rate(&mut self, right_ascension_rate: Option<f64>) {
        self.inner.right_ascension_rate = right_ascension_rate;
    }

    #[setter]
    pub fn set_declination_rate(&mut self, declination_rate: Option<f64>) {
        self.inner.declination_rate = declination_rate;
    }

    #[setter]
    pub fn set_right_ascension(&mut self, right_ascension: f64) {
        let mut new_inner = TopocentricElements::new(right_ascension, self.inner.declination);
        new_inner.range = self.inner.range;
        new_inner.range_rate = self.inner.range_rate;
        new_inner.right_ascension_rate = self.inner.right_ascension_rate;
        new_inner.declination_rate = self.inner.declination_rate;
        self.inner = new_inner;
    }

    #[setter]
    pub fn set_declination(&mut self, declination: f64) {
        let mut new_inner = TopocentricElements::new(self.inner.right_ascension, declination);
        new_inner.range = self.inner.range;
        new_inner.range_rate = self.inner.range_rate;
        new_inner.right_ascension_rate = self.inner.right_ascension_rate;
        new_inner.declination_rate = self.inner.declination_rate;
        self.inner = new_inner;
    }

    #[getter]
    pub fn get_observed_direction(&self) -> PyCartesianVector {
        PyCartesianVector::from(*self.inner.get_observed_direction())
    }
}
