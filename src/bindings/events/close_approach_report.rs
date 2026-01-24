use super::PyCloseApproach;
use crate::bindings::time::PyEpoch;
use crate::events::{CloseApproach, CloseApproachReport};
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "CloseApproachReport")]
pub struct PyCloseApproachReport {
    inner: CloseApproachReport,
}

impl From<CloseApproachReport> for PyCloseApproachReport {
    fn from(inner: CloseApproachReport) -> Self {
        Self { inner }
    }
}

impl From<PyCloseApproachReport> for CloseApproachReport {
    fn from(value: PyCloseApproachReport) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyCloseApproachReport {
    #[new]
    pub fn new(start: PyEpoch, end: PyEpoch, distance_threshold: f64) -> Self {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        CloseApproachReport::new(start, end, distance_threshold).into()
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
    pub fn get_close_approaches(&self) -> Vec<PyCloseApproach> {
        self.inner
            .get_close_approaches()
            .into_iter()
            .map(PyCloseApproach::from)
            .collect()
    }

    #[setter]
    pub fn set_close_approaches(&mut self, close_approaches: Vec<PyCloseApproach>) {
        let close_approaches: Vec<CloseApproach> = close_approaches.into_iter().map(CloseApproach::from).collect();
        self.inner.set_close_approaches(close_approaches);
    }
}
