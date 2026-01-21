use super::PyHorizonAccess;
use crate::bindings::time::PyEpoch;
use crate::bindings::time::PyTimeSpan;
use crate::events::{HorizonAccess, HorizonAccessReport};
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "HorizonAccessReport")]
pub struct PyHorizonAccessReport {
    inner: HorizonAccessReport,
}

impl From<HorizonAccessReport> for PyHorizonAccessReport {
    fn from(inner: HorizonAccessReport) -> Self {
        Self { inner }
    }
}

impl From<PyHorizonAccessReport> for HorizonAccessReport {
    fn from(value: PyHorizonAccessReport) -> Self {
        value.inner
    }
}

impl PyHorizonAccessReport {
    pub fn new(start: PyEpoch, end: PyEpoch, elevation_threshold: f64, duration_threshold: PyTimeSpan) -> Self {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        HorizonAccessReport::new(start, end, elevation_threshold, duration_threshold.into()).into()
    }

    pub fn set_accesses(&mut self, horizon_accesses: Vec<PyHorizonAccess>) {
        let accesses: Vec<HorizonAccess> = horizon_accesses.into_iter().map(HorizonAccess::from).collect();
        self.inner.set_accesses(accesses);
    }
}

#[pymethods]
impl PyHorizonAccessReport {
    #[getter]
    pub fn get_start(&self) -> PyEpoch {
        self.inner.get_start().into()
    }

    #[getter]
    pub fn get_end(&self) -> PyEpoch {
        self.inner.get_end().into()
    }

    #[getter]
    pub fn get_elevation_threshold(&self) -> f64 {
        self.inner.get_elevation_threshold()
    }

    #[getter]
    pub fn get_duration_threshold(&self) -> PyTimeSpan {
        PyTimeSpan::from(self.inner.get_duration_threshold())
    }

    #[getter]
    pub fn get_accesses(&self) -> Vec<PyHorizonAccess> {
        self.inner
            .get_accesses()
            .into_iter()
            .map(PyHorizonAccess::from)
            .collect()
    }
}
