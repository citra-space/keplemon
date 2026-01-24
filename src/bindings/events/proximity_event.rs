use crate::bindings::time::PyEpoch;
use crate::events::ProximityEvent;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "ProximityEvent")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyProximityEvent {
    inner: ProximityEvent,
}

impl From<ProximityEvent> for PyProximityEvent {
    fn from(inner: ProximityEvent) -> Self {
        Self { inner }
    }
}

impl From<PyProximityEvent> for ProximityEvent {
    fn from(value: PyProximityEvent) -> Self {
        value.inner
    }
}

impl PyProximityEvent {
    pub fn new(
        primary_id: String,
        secondary_id: String,
        start_epoch: PyEpoch,
        end_epoch: PyEpoch,
        minimum_distance: f64,
        maximum_distance: f64,
    ) -> Self {
        let start_epoch: Epoch = start_epoch.into();
        let end_epoch: Epoch = end_epoch.into();
        ProximityEvent::new(
            primary_id,
            secondary_id,
            start_epoch,
            end_epoch,
            minimum_distance,
            maximum_distance,
        )
        .into()
    }
}

#[pymethods]
impl PyProximityEvent {
    #[getter]
    pub fn get_primary_id(&self) -> String {
        self.inner.get_primary_id()
    }

    #[getter]
    pub fn get_secondary_id(&self) -> String {
        self.inner.get_secondary_id()
    }

    #[getter]
    pub fn get_start_epoch(&self) -> PyEpoch {
        self.inner.get_start_epoch().into()
    }

    #[getter]
    pub fn get_end_epoch(&self) -> PyEpoch {
        self.inner.get_end_epoch().into()
    }

    #[getter]
    pub fn get_minimum_distance(&self) -> f64 {
        self.inner.get_minimum_distance()
    }

    #[getter]
    pub fn get_maximum_distance(&self) -> f64 {
        self.inner.get_maximum_distance()
    }
}
