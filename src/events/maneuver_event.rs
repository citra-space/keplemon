use crate::elements::CartesianVector;
use crate::time::Epoch;

#[derive(Debug, Clone, PartialEq)]
pub struct ManeuverEvent {
    satellite_id: String,
    epoch: Epoch,
    delta_v: CartesianVector,
}

impl ManeuverEvent {
    pub fn new(satellite_id: String, epoch: Epoch, delta_v: CartesianVector) -> Self {
        Self {
            satellite_id,
            epoch,
            delta_v,
        }
    }

    pub fn get_satellite_id(&self) -> String {
        self.satellite_id.clone()
    }

    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn get_delta_v(&self) -> CartesianVector {
        self.delta_v
    }
}
