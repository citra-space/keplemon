use crate::bindings::elements::PyCartesianVector;
use crate::bindings::time::PyEpoch;
use crate::events::ManeuverEvent;
use pyo3::prelude::*;

#[pyclass(name = "ManeuverEvent")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyManeuverEvent {
    inner: ManeuverEvent,
}

impl From<ManeuverEvent> for PyManeuverEvent {
    fn from(inner: ManeuverEvent) -> Self {
        Self { inner }
    }
}

impl From<PyManeuverEvent> for ManeuverEvent {
    fn from(value: PyManeuverEvent) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyManeuverEvent {
    #[getter]
    pub fn get_satellite_id(&self) -> String {
        self.inner.get_satellite_id()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    #[getter]
    pub fn get_delta_v(&self) -> PyCartesianVector {
        self.inner.get_delta_v().into()
    }
}
