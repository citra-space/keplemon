use crate::estimation::{Observation, ObservationAssociation};
use crate::time::Epoch;

#[derive(Debug, Clone, PartialEq)]
pub struct CrossTagEvidence {
    epoch: Epoch,
    sensor_id: String,
    orphan_count: usize,
    approved_satellite_matched: bool,
    uct_was_visible: bool,
    conclusion: String,
    approved_associations: Vec<ObservationAssociation>,
    orphan_observations: Vec<Observation>,
}

impl CrossTagEvidence {
    pub fn new(
        epoch: Epoch,
        sensor_id: String,
        orphan_count: usize,
        approved_satellite_matched: bool,
        uct_was_visible: bool,
        conclusion: String,
        approved_associations: Vec<ObservationAssociation>,
        orphan_observations: Vec<Observation>,
    ) -> Self {
        Self {
            epoch,
            sensor_id,
            orphan_count,
            approved_satellite_matched,
            uct_was_visible,
            conclusion,
            approved_associations,
            orphan_observations,
        }
    }

    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn get_sensor_id(&self) -> String {
        self.sensor_id.clone()
    }

    pub fn get_orphan_count(&self) -> usize {
        self.orphan_count
    }

    pub fn get_approved_satellite_matched(&self) -> bool {
        self.approved_satellite_matched
    }

    pub fn get_uct_was_visible(&self) -> bool {
        self.uct_was_visible
    }

    pub fn get_conclusion(&self) -> String {
        self.conclusion.clone()
    }

    pub fn get_approved_associations(&self) -> Vec<ObservationAssociation> {
        self.approved_associations.clone()
    }

    pub fn get_orphan_observations(&self) -> Vec<Observation> {
        self.orphan_observations.clone()
    }
}

pub struct CrossTagReport {
    uct_id: String,
    is_likely_crosstag: bool,
    approved_candidate_id: Option<String>,
    confidence: f64,
    evidence: Vec<CrossTagEvidence>,
    reason: String,
    total_collections_analyzed: usize,
    collections_with_orphans: usize,
    collections_without_orphans: usize,
}

impl CrossTagReport {
    pub fn new(
        uct_id: String,
        is_likely_crosstag: bool,
        approved_candidate_id: Option<String>,
        confidence: f64,
        evidence: Vec<CrossTagEvidence>,
        reason: String,
        total_collections_analyzed: usize,
        collections_with_orphans: usize,
        collections_without_orphans: usize,
    ) -> Self {
        Self {
            uct_id,
            is_likely_crosstag,
            approved_candidate_id,
            confidence,
            evidence,
            reason,
            total_collections_analyzed,
            collections_with_orphans,
            collections_without_orphans,
        }
    }

    pub fn get_uct_id(&self) -> String {
        self.uct_id.clone()
    }

    pub fn get_is_likely_crosstag(&self) -> bool {
        self.is_likely_crosstag
    }

    pub fn get_approved_candidate_id(&self) -> Option<String> {
        self.approved_candidate_id.clone()
    }

    pub fn get_confidence(&self) -> f64 {
        self.confidence
    }

    pub fn get_evidence(&self) -> Vec<CrossTagEvidence> {
        self.evidence.clone()
    }

    pub fn get_reason(&self) -> String {
        self.reason.clone()
    }

    pub fn get_total_collections_analyzed(&self) -> usize {
        self.total_collections_analyzed
    }

    pub fn get_collections_with_orphans(&self) -> usize {
        self.collections_with_orphans
    }

    pub fn get_collections_without_orphans(&self) -> usize {
        self.collections_without_orphans
    }
}
