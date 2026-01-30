use super::{PyCandidateAnalysis, PyCrossTagEvidence};
use crate::events::{CrossTagReport, CrossTagResult};
use pyo3::prelude::*;

#[pyclass(name = "CrossTagResult")]
#[derive(Debug, Clone, PartialEq)]
pub enum PyCrossTagResult {
    NoProximityFound,
    NoObservationsDuringProximity,
    InsufficientEvidence,
    RealUCT,
    CrossTag,
}

impl From<CrossTagResult> for PyCrossTagResult {
    fn from(result: CrossTagResult) -> Self {
        match result {
            CrossTagResult::NoProximityFound => PyCrossTagResult::NoProximityFound,
            CrossTagResult::NoObservationsDuringProximity => PyCrossTagResult::NoObservationsDuringProximity,
            CrossTagResult::InsufficientEvidence => PyCrossTagResult::InsufficientEvidence,
            CrossTagResult::RealUCT => PyCrossTagResult::RealUCT,
            CrossTagResult::CrossTag => PyCrossTagResult::CrossTag,
        }
    }
}

impl From<PyCrossTagResult> for CrossTagResult {
    fn from(result: PyCrossTagResult) -> Self {
        match result {
            PyCrossTagResult::NoProximityFound => CrossTagResult::NoProximityFound,
            PyCrossTagResult::NoObservationsDuringProximity => CrossTagResult::NoObservationsDuringProximity,
            PyCrossTagResult::InsufficientEvidence => CrossTagResult::InsufficientEvidence,
            PyCrossTagResult::RealUCT => CrossTagResult::RealUCT,
            PyCrossTagResult::CrossTag => CrossTagResult::CrossTag,
        }
    }
}

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
        py: Python<'_>,
        uct_id: String,
        result: PyCrossTagResult,
        approved_candidate_id: Option<String>,
        confidence: f64,
        evidence: Vec<PyCrossTagEvidence>,
        reason: String,
        total_collections_analyzed: usize,
        real_uct_votes: usize,
        cross_tag_votes: usize,
        inconclusive_votes: usize,
        all_candidates: Vec<Py<PyCandidateAnalysis>>,
    ) -> PyResult<Self> {
        let evidence = evidence
            .into_iter()
            .map(crate::events::CrossTagEvidence::from)
            .collect();

        let all_candidates_vec: Vec<crate::events::CandidateAnalysis> = all_candidates
            .into_iter()
            .map(|item| {
                let candidate = item.borrow(py);
                candidate.inner().clone()
            })
            .collect();

        Ok(CrossTagReport::new(
            uct_id,
            result.into(),
            approved_candidate_id,
            confidence,
            evidence,
            reason,
            total_collections_analyzed,
            real_uct_votes,
            cross_tag_votes,
            inconclusive_votes,
            all_candidates_vec,
        )
        .into())
    }

    #[getter]
    pub fn get_uct_id(&self) -> String {
        self.inner.get_uct_id()
    }

    #[getter]
    pub fn get_result(&self) -> PyCrossTagResult {
        self.inner.get_result().into()
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
    pub fn get_all_candidates(&self) -> Vec<PyCandidateAnalysis> {
        self.inner
            .get_all_candidates()
            .into_iter()
            .map(PyCandidateAnalysis::from)
            .collect()
    }
}
