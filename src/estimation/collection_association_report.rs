use super::{Observation, ObservationAssociation};

#[derive(Debug, Clone)]
pub struct CollectionAssociationReport {
    id: String,
    orphan_observations: Vec<Observation>,
    associations: Vec<ObservationAssociation>,
    moving_satellite_ids: Vec<String>,
}

impl CollectionAssociationReport {
    pub fn new(
        id: String,
        orphan_observations: Vec<Observation>,
        associations: Vec<ObservationAssociation>,
        moving_satellite_ids: Vec<String>,
    ) -> Self {
        Self {
            id,
            orphan_observations,
            associations,
            moving_satellite_ids,
        }
    }

    pub fn get_orphan_observations(&self) -> &Vec<Observation> {
        &self.orphan_observations
    }

    pub fn get_associations(&self) -> &Vec<ObservationAssociation> {
        &self.associations
    }

    pub fn get_moving_satellite_ids(&self) -> &Vec<String> {
        &self.moving_satellite_ids
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
