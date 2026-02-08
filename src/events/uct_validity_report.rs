use crate::configs::{REAL_UCT_MAX_RMS, REAL_UCT_MIN_OB_COUNT, REAL_UCT_MIN_OB_SPAN_MINUTES};
use crate::enums::{UCTObservability, UCTValidity};
use crate::estimation::ObservationAssociation;
use crate::events::{CloseApproach, ProximityEvent};
use crate::time::TimeSpan;

pub struct UCTValidityReport {
    satellite_id: String,
    associations: Vec<ObservationAssociation>,
    possible_cross_tags: Vec<ProximityEvent>,
    possible_origins: Vec<CloseApproach>,
    observability: UCTObservability,
}

impl UCTValidityReport {
    pub fn new(
        satellite_id: String,
        associations: Vec<ObservationAssociation>,
        possible_cross_tags: Vec<ProximityEvent>,
        possible_origins: Vec<CloseApproach>,
        observability: UCTObservability,
    ) -> Self {
        Self {
            satellite_id,
            associations,
            possible_cross_tags,
            possible_origins,
            observability,
        }
    }

    pub fn get_satellite_id(&self) -> String {
        self.satellite_id.clone()
    }

    pub fn get_associations(&self) -> Vec<ObservationAssociation> {
        self.associations.clone()
    }
    pub fn get_possible_cross_tags(&self) -> Vec<ProximityEvent> {
        self.possible_cross_tags.clone()
    }
    pub fn get_possible_origins(&self) -> Vec<CloseApproach> {
        self.possible_origins.clone()
    }
    pub fn get_observability(&self) -> UCTObservability {
        self.observability
    }
    pub fn get_validity(&self) -> crate::enums::UCTValidity {
        match (
            self.satisfies_real_criteria(),
            self.possible_cross_tags.is_empty(),
            self.possible_origins.is_empty(),
        ) {
            (true, _, _) => UCTValidity::LikelyReal,
            (false, false, _) => UCTValidity::PossibleCrossTag,
            (false, true, false) => UCTValidity::PossibleManeuver,
            (false, true, true) => UCTValidity::Inconclusive,
        }
    }
    fn satisfies_real_criteria(&self) -> bool {
        !self.associations.is_empty()
            && self.associations.len() >= REAL_UCT_MIN_OB_COUNT
            && self.get_rms().unwrap() <= REAL_UCT_MAX_RMS
            && self.get_observation_span().unwrap().in_minutes() >= REAL_UCT_MIN_OB_SPAN_MINUTES
    }

    pub fn get_rms(&self) -> Option<f64> {
        if self.associations.is_empty() {
            return None;
        }
        let squared_residuals_sum: f64 = self
            .associations
            .iter()
            .map(|assoc| assoc.get_residual().get_range().powi(2))
            .sum();
        let rms = (squared_residuals_sum / self.associations.len() as f64).sqrt();
        Some(rms)
    }

    pub fn get_observation_span(&self) -> Option<TimeSpan> {
        if self.associations.is_empty() {
            return None;
        }
        let first_epoch = self
            .associations
            .iter()
            .map(|assoc| assoc.get_residual().get_epoch())
            .min()?;
        let last_epoch = self
            .associations
            .iter()
            .map(|assoc| assoc.get_residual().get_epoch())
            .max()?;
        Some(last_epoch - first_epoch)
    }
}
