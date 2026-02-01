use crate::bindings::enums::{PyUCTObservability, PyUCTValidity};
use crate::bindings::estimation::PyObservationAssociation;
use crate::bindings::events::{PyCloseApproach, PyProximityEvent};
use crate::events::UCTValidityReport;
use pyo3::prelude::*;

#[pyclass(name = "UCTValidityReport")]
pub struct PyUCTValidityReport {
    inner: UCTValidityReport,
}

impl From<UCTValidityReport> for PyUCTValidityReport {
    fn from(inner: UCTValidityReport) -> Self {
        Self { inner }
    }
}

impl From<PyUCTValidityReport> for UCTValidityReport {
    fn from(value: PyUCTValidityReport) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyUCTValidityReport {
    #[getter]
    pub fn get_satellite_id(&self) -> String {
        self.inner.get_satellite_id()
    }

    #[getter]
    pub fn get_associations(&self) -> Vec<PyObservationAssociation> {
        self.inner
            .get_associations()
            .into_iter()
            .map(PyObservationAssociation::from)
            .collect()
    }

    #[getter]
    pub fn get_possible_cross_tags(&self) -> Vec<PyProximityEvent> {
        self.inner
            .get_possible_cross_tags()
            .into_iter()
            .map(PyProximityEvent::from)
            .collect()
    }

    #[getter]
    pub fn get_possible_origins(&self) -> Vec<PyCloseApproach> {
        self.inner
            .get_possible_origins()
            .into_iter()
            .map(PyCloseApproach::from)
            .collect()
    }

    #[getter]
    pub fn get_observability(&self) -> PyUCTObservability {
        self.inner.get_observability().into()
    }

    #[getter]
    pub fn get_validity(&self) -> PyUCTValidity {
        self.inner.get_validity().into()
    }
}
