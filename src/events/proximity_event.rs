use crate::time::Epoch;

#[derive(Debug, Clone, PartialEq)]
pub struct ProximityEvent {
    primary_id: String,
    secondary_id: String,
    start_epoch: Epoch,
    end_epoch: Epoch,
    minimum_distance: f64,
    maximum_distance: f64,
}

impl ProximityEvent {
    pub fn new(
        primary_id: String,
        secondary_id: String,
        start_epoch: Epoch,
        end_epoch: Epoch,
        minimum_distance: f64,
        maximum_distance: f64,
    ) -> Self {
        Self {
            primary_id,
            secondary_id,
            start_epoch,
            end_epoch,
            minimum_distance,
            maximum_distance,
        }
    }

    pub fn get_primary_id(&self) -> String {
        self.primary_id.clone()
    }

    pub fn get_secondary_id(&self) -> String {
        self.secondary_id.clone()
    }

    pub fn get_start_epoch(&self) -> Epoch {
        self.start_epoch
    }

    pub fn get_end_epoch(&self) -> Epoch {
        self.end_epoch
    }

    pub fn get_minimum_distance(&self) -> f64 {
        self.minimum_distance
    }

    pub fn get_maximum_distance(&self) -> f64 {
        self.maximum_distance
    }
}
