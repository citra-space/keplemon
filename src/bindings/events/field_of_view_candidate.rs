use crate::bindings::elements::PyTopocentricElements;
use crate::events::FieldOfViewCandidate;
use pyo3::prelude::*;

#[pyclass(name = "FieldOfViewCandidate")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyFieldOfViewCandidate {
    inner: FieldOfViewCandidate,
}

impl From<FieldOfViewCandidate> for PyFieldOfViewCandidate {
    fn from(inner: FieldOfViewCandidate) -> Self {
        Self { inner }
    }
}

impl From<PyFieldOfViewCandidate> for FieldOfViewCandidate {
    fn from(value: PyFieldOfViewCandidate) -> Self {
        value.inner
    }
}

impl PyFieldOfViewCandidate {
    pub fn new(satellite_id: String, direction: &PyTopocentricElements) -> Self {
        FieldOfViewCandidate::new(satellite_id, &(*direction).into()).into()
    }
}

#[pymethods]
impl PyFieldOfViewCandidate {
    #[getter]
    pub fn get_satellite_id(&self) -> String {
        self.inner.get_satellite_id()
    }

    #[getter]
    pub fn get_direction(&self) -> PyTopocentricElements {
        PyTopocentricElements::from(self.inner.get_direction())
    }
}
