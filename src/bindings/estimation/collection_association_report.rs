use super::{PyObservation, PyObservationAssociation};
use crate::estimation::CollectionAssociationReport;
use pyo3::prelude::*;

#[pyclass(name = "CollectionAssociationReport")]
#[derive(Debug, Clone)]
pub struct PyCollectionAssociationReport {
    inner: CollectionAssociationReport,
}

impl From<CollectionAssociationReport> for PyCollectionAssociationReport {
    fn from(inner: CollectionAssociationReport) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyCollectionAssociationReport {
    #[getter]
    pub fn get_orphan_observations(&self) -> Vec<PyObservation> {
        self.inner
            .get_orphan_observations()
            .iter()
            .cloned()
            .map(PyObservation::from)
            .collect()
    }

    #[getter]
    pub fn get_associations(&self) -> Vec<PyObservationAssociation> {
        self.inner
            .get_associations()
            .iter()
            .cloned()
            .map(PyObservationAssociation::from)
            .collect()
    }

    #[getter]
    pub fn get_moving_satellite_ids(&self) -> Vec<String> {
        self.inner.get_moving_satellite_ids().clone()
    }
}
