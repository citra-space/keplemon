use crate::enums::{UCTObservability, UCTValidity};
use crate::estimation::ObservationAssociation;
use crate::events::{CloseApproach, ProximityEvent};

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
            self.associations.is_empty(),
            self.possible_cross_tags.is_empty(),
            self.possible_origins.is_empty(),
        ) {
            (false, _, _) => UCTValidity::LikelyReal,
            (true, false, _) => UCTValidity::PossibleCrossTag,
            (true, true, false) => UCTValidity::PossibleManeuver,
            (true, true, true) => UCTValidity::Inconclusive,
        }
    }
}
