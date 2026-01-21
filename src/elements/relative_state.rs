use super::CartesianVector;
use crate::time::Epoch;

#[derive(Debug, Clone, PartialEq)]
pub struct RelativeState {
    pub epoch: Epoch,
    pub position: CartesianVector,
    pub velocity: CartesianVector,
    pub origin_satellite_id: String,
    pub secondary_satellite_id: String,
}

impl RelativeState {
    pub fn new(
        epoch: Epoch,
        position: CartesianVector,
        velocity: CartesianVector,
        origin_id: String,
        secondary_id: String,
    ) -> Self {
        Self {
            epoch,
            position,
            velocity,
            origin_satellite_id: origin_id,
            secondary_satellite_id: secondary_id,
        }
    }
}
