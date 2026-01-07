use super::ObservationResidual;
use crate::enums::AssociationConfidence;

#[derive(Debug, Clone, PartialEq)]
pub struct ObservationAssociation {
    observation_id: String,
    satellite_id: String,
    residual: ObservationResidual,
    confidence: AssociationConfidence,
}

impl ObservationAssociation {
    pub fn new(
        observation_id: String,
        satellite_id: String,
        residual: ObservationResidual,
        confidence: AssociationConfidence,
    ) -> Self {
        ObservationAssociation {
            observation_id,
            satellite_id,
            residual,
            confidence,
        }
    }

    pub fn get_observation_id(&self) -> &str {
        &self.observation_id
    }

    pub fn get_satellite_id(&self) -> &str {
        &self.satellite_id
    }

    pub fn get_confidence(&self) -> AssociationConfidence {
        self.confidence
    }

    pub fn get_residual(&self) -> ObservationResidual {
        self.residual
    }
}
