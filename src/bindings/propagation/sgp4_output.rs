use crate::bindings::elements::{PyCartesianState, PyGeodeticPosition, PyKeplerianElements};
use crate::propagation::SGP4Output;
use pyo3::prelude::*;

#[pyclass(name = "SGP4Output")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PySGP4Output {
    inner: SGP4Output,
}

impl From<SGP4Output> for PySGP4Output {
    fn from(inner: SGP4Output) -> Self {
        Self { inner }
    }
}

impl From<PySGP4Output> for SGP4Output {
    fn from(value: PySGP4Output) -> Self {
        value.inner
    }
}

#[pymethods]
impl PySGP4Output {
    #[getter]
    pub fn get_cartesian_state(&self) -> PyCartesianState {
        PyCartesianState::from(self.inner.get_cartesian_state())
    }

    #[getter]
    pub fn get_mean_elements(&self) -> PyKeplerianElements {
        PyKeplerianElements::from(self.inner.get_mean_elements())
    }

    #[getter]
    pub fn get_osculating_elements(&self) -> PyKeplerianElements {
        PyKeplerianElements::from(self.inner.get_osculating_elements())
    }

    #[getter]
    pub fn get_geodetic_position(&self) -> PyGeodeticPosition {
        PyGeodeticPosition::from(self.inner.get_geodetic_position())
    }
}
