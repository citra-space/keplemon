use crate::bindings::time::PyEpoch;
use crate::events::CloseApproach;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "CloseApproach")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyCloseApproach {
    inner: CloseApproach,
}

impl From<CloseApproach> for PyCloseApproach {
    fn from(inner: CloseApproach) -> Self {
        Self { inner }
    }
}

impl From<PyCloseApproach> for CloseApproach {
    fn from(value: PyCloseApproach) -> Self {
        value.inner
    }
}

impl PyCloseApproach {
    pub fn new(primary_id: String, secondary_id: String, epoch: PyEpoch, distance: f64) -> Self {
        let epoch: Epoch = epoch.into();
        CloseApproach::new(primary_id, secondary_id, epoch, distance).into()
    }
}

#[pymethods]
impl PyCloseApproach {
    #[getter]
    pub fn get_primary_id(&self) -> String {
        self.inner.get_primary_id()
    }

    #[getter]
    pub fn get_secondary_id(&self) -> String {
        self.inner.get_secondary_id()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    #[getter]
    pub fn get_distance(&self) -> f64 {
        self.inner.get_distance()
    }
}
