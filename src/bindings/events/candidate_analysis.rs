use super::PyCrossTagEvidence;
use crate::bindings::events::cross_tag_report::PyCrossTagResult;
use crate::events::CandidateAnalysis;
use pyo3::prelude::*;

#[pyclass(name = "CandidateAnalysis")]
#[derive(Clone)]
pub struct PyCandidateAnalysis {
    inner: CandidateAnalysis,
}

impl From<CandidateAnalysis> for PyCandidateAnalysis {
    fn from(inner: CandidateAnalysis) -> Self {
        Self { inner }
    }
}

impl From<PyCandidateAnalysis> for CandidateAnalysis {
    fn from(value: PyCandidateAnalysis) -> Self {
        value.inner
    }
}

impl PyCandidateAnalysis {
    pub fn inner(&self) -> &CandidateAnalysis {
        &self.inner
    }
}

#[pymethods]
impl PyCandidateAnalysis {
    #[new]
    pub fn new(
        candidate_id: String,
        result: PyCrossTagResult,
        confidence: f64,
        real_uct_votes: usize,
        cross_tag_votes: usize,
        inconclusive_votes: usize,
        total_collections_analyzed: usize,
        evidence: Vec<PyCrossTagEvidence>,
    ) -> Self {
        let evidence = evidence
            .into_iter()
            .map(crate::events::CrossTagEvidence::from)
            .collect();
        CandidateAnalysis::new(
            candidate_id,
            result.into(),
            confidence,
            real_uct_votes,
            cross_tag_votes,
            inconclusive_votes,
            total_collections_analyzed,
            evidence,
        )
        .into()
    }

    #[getter]
    pub fn get_candidate_id(&self) -> String {
        self.inner.get_candidate_id()
    }

    #[getter]
    pub fn get_result(&self) -> PyCrossTagResult {
        self.inner.get_result().into()
    }

    #[getter]
    pub fn get_confidence(&self) -> f64 {
        self.inner.get_confidence()
    }

    #[getter]
    pub fn get_real_uct_votes(&self) -> usize {
        self.inner.get_real_uct_votes()
    }

    #[getter]
    pub fn get_cross_tag_votes(&self) -> usize {
        self.inner.get_cross_tag_votes()
    }

    #[getter]
    pub fn get_inconclusive_votes(&self) -> usize {
        self.inner.get_inconclusive_votes()
    }

    #[getter]
    pub fn get_total_collections_analyzed(&self) -> usize {
        self.inner.get_total_collections_analyzed()
    }

    #[getter]
    pub fn get_evidence(&self) -> Vec<PyCrossTagEvidence> {
        self.inner
            .get_evidence()
            .into_iter()
            .map(PyCrossTagEvidence::from)
            .collect()
    }
}
