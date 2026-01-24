use super::PyProximityEvent;
use crate::bindings::time::PyEpoch;
use crate::events::{ProximityEvent, ProximityReport};
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "ProximityReport")]
pub struct PyProximityReport {
    inner: ProximityReport,
}

impl From<ProximityReport> for PyProximityReport {
    fn from(inner: ProximityReport) -> Self {
        Self { inner }
    }
}

impl From<PyProximityReport> for ProximityReport {
    fn from(value: PyProximityReport) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyProximityReport {
    #[new]
    pub fn new(start: PyEpoch, end: PyEpoch, distance_threshold: f64) -> Self {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        ProximityReport::new(start, end, distance_threshold).into()
    }

    #[getter]
    pub fn get_start(&self) -> PyEpoch {
        self.inner.get_start().into()
    }

    #[getter]
    pub fn get_end(&self) -> PyEpoch {
        self.inner.get_end().into()
    }

    #[getter]
    pub fn get_distance_threshold(&self) -> f64 {
        self.inner.get_distance_threshold()
    }

    #[getter]
    pub fn get_events(&self) -> Vec<PyProximityEvent> {
        self.inner
            .get_events()
            .into_iter()
            .map(PyProximityEvent::from)
            .collect()
    }

    #[setter]
    pub fn set_events(&mut self, events: Vec<PyProximityEvent>) {
        let events: Vec<ProximityEvent> = events.into_iter().map(ProximityEvent::from).collect();
        self.inner.set_events(events);
    }
}
