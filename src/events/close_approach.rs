use crate::time::Epoch;

#[derive(Debug, Clone, PartialEq)]
pub struct CloseApproach {
    primary_id: String,
    secondary_id: String,
    epoch: Epoch,
    distance: f64,
}

impl CloseApproach {
    pub fn new(primary_id: String, secondary_id: String, epoch: Epoch, distance: f64) -> Self {
        Self {
            primary_id,
            secondary_id,
            epoch,
            distance,
        }
    }

    pub fn get_primary_id(&self) -> String {
        self.primary_id.clone()
    }

    pub fn get_secondary_id(&self) -> String {
        self.secondary_id.clone()
    }

    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn get_distance(&self) -> f64 {
        self.distance
    }
}
