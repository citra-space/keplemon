use super::PyManeuverEvent;
use crate::bindings::time::PyEpoch;
use crate::events::{ManeuverEvent, ManeuverReport};
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "ManeuverReport")]
pub struct PyManeuverReport {
    inner: ManeuverReport,
}

impl From<ManeuverReport> for PyManeuverReport {
    fn from(inner: ManeuverReport) -> Self {
        Self { inner }
    }
}

impl From<PyManeuverReport> for ManeuverReport {
    fn from(value: PyManeuverReport) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyManeuverReport {
    #[new]
    pub fn new(start: PyEpoch, end: PyEpoch, distance_threshold: f64, velocity_threshold: f64) -> Self {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        ManeuverReport::new(start, end, distance_threshold, velocity_threshold).into()
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
    pub fn get_velocity_threshold(&self) -> f64 {
        self.inner.get_velocity_threshold()
    }

    #[getter]
    pub fn get_maneuvers(&self) -> Vec<PyManeuverEvent> {
        self.inner
            .get_maneuvers()
            .into_iter()
            .map(PyManeuverEvent::from)
            .collect()
    }

    #[setter]
    pub fn set_maneuvers(&mut self, maneuvers: Vec<PyManeuverEvent>) {
        let maneuvers: Vec<ManeuverEvent> = maneuvers.into_iter().map(ManeuverEvent::from).collect();
        self.inner.set_maneuvers(maneuvers);
    }
}
