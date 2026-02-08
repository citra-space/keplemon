use crate::bindings::elements::PyCartesianState;
use crate::bindings::events::PyCloseApproach;
use crate::bindings::events::PyHorizonAccess;
use crate::bindings::time::PyEpoch;
use crate::bindings::time::PyTimeSpan;
use crate::elements::Ephemeris;
use crate::time::{Epoch, TimeSpan};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass(name = "Ephemeris")]
#[derive(Debug, Clone)]
pub struct PyEphemeris {
    inner: Ephemeris,
}

impl From<Ephemeris> for PyEphemeris {
    fn from(inner: Ephemeris) -> Self {
        Self { inner }
    }
}

impl From<PyEphemeris> for Ephemeris {
    fn from(value: PyEphemeris) -> Self {
        value.inner
    }
}

impl PyEphemeris {
    pub fn inner(&self) -> &Ephemeris {
        &self.inner
    }

    pub fn get_number_of_states(&self) -> Result<i32, String> {
        self.inner.get_number_of_states()
    }

    pub fn add_state(&self, state: PyCartesianState) -> Result<(), String> {
        self.inner.add_state(state.into())
    }

    pub fn new(satellite_id: String, norad_id: Option<i32>, state: PyCartesianState) -> Result<Self, String> {
        Ephemeris::new(satellite_id, norad_id, state.into()).map(PyEphemeris::from)
    }
}

#[pymethods]
impl PyEphemeris {
    #[new]
    pub fn py_new(satellite_id: String, norad_id: Option<i32>, state: PyCartesianState) -> PyResult<Self> {
        PyEphemeris::new(satellite_id, norad_id, state).map_err(PyException::new_err)
    }

    #[getter("number_of_states")]
    pub fn py_get_number_of_states(&self) -> PyResult<i32> {
        self.get_number_of_states()
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    pub fn get_state_at_epoch(&self, epoch: PyEpoch) -> Option<PyCartesianState> {
        let epoch: Epoch = epoch.into();
        self.inner.get_state_at_epoch(epoch).map(PyCartesianState::from)
    }

    pub fn get_horizon_accesses(
        &self,
        sensor: &PyEphemeris,
        min_el: f64,
        min_duration: PyTimeSpan,
    ) -> Option<Vec<PyHorizonAccess>> {
        let min_duration: TimeSpan = min_duration.into();
        self.inner
            .get_horizon_accesses(sensor.inner(), min_el, min_duration)
            .map(|accesses| accesses.into_iter().map(PyHorizonAccess::from).collect())
    }

    pub fn get_close_approach(&self, other: &PyEphemeris, distance_threshold: f64) -> Option<PyCloseApproach> {
        self.inner
            .get_close_approach(other.inner(), distance_threshold)
            .map(PyCloseApproach::from)
    }

    pub fn get_satellite_id(&self) -> String {
        self.inner.get_satellite_id()
    }

    pub fn get_norad_id(&self) -> i32 {
        self.inner.get_norad_id()
    }
}
