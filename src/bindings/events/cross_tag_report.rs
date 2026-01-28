use super::PyCrossTagEvidence;
use crate::bindings::time::PyEpoch;
use crate::events::CrossTagReport;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "CrossTagReport")]
pub struct PyCrossTagReport {
    inner: CrossTagReport,
}

impl From<CrossTagReport> for PyCrossTagReport {
    fn from(inner: CrossTagReport) -> Self {
        Self { inner }
    }
}

impl From<PyCrossTagReport> for CrossTagReport {
    fn from(value: PyCrossTagReport) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyCrossTagReport {
    #[new]
    pub fn new(
        uct_id: String,
        is_likely_crosstag: bool,
        approved_candidate_id: Option<String>,
        confidence: f64,
        evidence: Vec<PyCrossTagEvidence>,
        reason: String,
        total_collections_analyzed: usize,
        collections_with_orphans: usize,
        collections_without_orphans: usize,
    ) -> Self {
        let evidence = evidence
            .into_iter()
            .map(crate::events::CrossTagEvidence::from)
            .collect();
        CrossTagReport::new(
            uct_id,
            is_likely_crosstag,
            approved_candidate_id,
            confidence,
            evidence,
            reason,
            total_collections_analyzed,
            collections_with_orphans,
            collections_without_orphans,
        )
        .into()
    }

    #[getter]
    pub fn get_uct_id(&self) -> String {
        self.inner.get_uct_id()
    }

    #[getter]
    pub fn get_is_likely_crosstag(&self) -> bool {
        self.inner.get_is_likely_crosstag()
    }

    #[getter]
    pub fn get_approved_candidate_id(&self) -> Option<String> {
        self.inner.get_approved_candidate_id()
    }

    #[getter]
    pub fn get_confidence(&self) -> f64 {
        self.inner.get_confidence()
    }

    #[getter]
    pub fn get_evidence(&self) -> Vec<PyCrossTagEvidence> {
        self.inner
            .get_evidence()
            .into_iter()
            .map(PyCrossTagEvidence::from)
            .collect()
    }

    #[getter]
    pub fn get_reason(&self) -> String {
        self.inner.get_reason()
    }

    #[getter]
    pub fn get_total_collections_analyzed(&self) -> usize {
        self.inner.get_total_collections_analyzed()
    }

    #[getter]
    pub fn get_collections_with_orphans(&self) -> usize {
        self.inner.get_collections_with_orphans()
    }

    #[getter]
    pub fn get_collections_without_orphans(&self) -> usize {
        self.inner.get_collections_without_orphans()
    }
}
