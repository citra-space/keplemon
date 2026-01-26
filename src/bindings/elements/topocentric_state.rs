use super::{PyHorizonState, PyTopocentricElements};
use crate::bindings::bodies::PyObservatory;
use crate::bindings::time::PyEpoch;
use crate::bodies::Observatory;
use crate::elements::TopocentricState;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "TopocentricState")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyTopocentricState {
    inner: TopocentricState,
}

impl Copy for PyTopocentricState {}

impl From<TopocentricState> for PyTopocentricState {
    fn from(inner: TopocentricState) -> Self {
        Self { inner }
    }
}

impl From<PyTopocentricState> for TopocentricState {
    fn from(value: PyTopocentricState) -> Self {
        value.inner
    }
}

impl PyTopocentricState {
    pub fn new(epoch: PyEpoch, elements: PyTopocentricElements) -> Self {
        let epoch: Epoch = epoch.into();
        Self {
            inner: TopocentricState::new(epoch, elements.into()),
        }
    }

    pub fn from_horizon_state(state: &PyHorizonState, observer: &Observatory) -> Result<Self, String> {
        let core_state = (*state).into();
        Ok(TopocentricState::from((&core_state, observer)).into())
    }
}

#[pymethods]
impl PyTopocentricState {
    #[new]
    pub fn __init__(epoch: PyEpoch, elements: PyTopocentricElements) -> Self {
        Self::new(epoch, elements)
    }

    #[staticmethod]
    #[pyo3(name = "from_horizon_state")]
    pub fn py_from_horizon_state(state: &PyHorizonState, observer: &PyObservatory) -> PyResult<Self> {
        Self::from_horizon_state(state, observer.inner()).map_err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>)
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.epoch.into()
    }

    #[getter]
    pub fn get_elements(&self) -> PyTopocentricElements {
        PyTopocentricElements::from(self.inner.elements)
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.inner.elements.range
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.inner.elements.range_rate
    }

    #[getter]
    pub fn get_right_ascension_rate(&self) -> Option<f64> {
        self.inner.elements.right_ascension_rate
    }

    #[getter]
    pub fn get_declination_rate(&self) -> Option<f64> {
        self.inner.elements.declination_rate
    }

    #[getter]
    pub fn get_right_ascension(&self) -> f64 {
        self.inner.elements.right_ascension
    }
    #[getter]
    pub fn get_declination(&self) -> f64 {
        self.inner.elements.declination
    }
}
