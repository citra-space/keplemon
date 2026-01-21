use super::FieldOfViewCandidate;
use crate::elements::{CartesianVector, TopocentricElements};
use crate::enums::ReferenceFrame;
use crate::time::Epoch;

pub struct FieldOfViewReport {
    epoch: Epoch,
    sensor_position: CartesianVector,
    sensor_direction: TopocentricElements,
    fov_angle: f64,
    candidates: Vec<FieldOfViewCandidate>,
    reference_frame: ReferenceFrame,
}

impl FieldOfViewReport {
    pub fn set_candidates(&mut self, candidates: Vec<FieldOfViewCandidate>) {
        self.candidates = candidates;
    }

    pub fn new(
        epoch: Epoch,
        sensor_position: CartesianVector,
        sensor_direction: &TopocentricElements,
        fov_angle: f64,
        reference_frame: ReferenceFrame,
    ) -> Self {
        if reference_frame != ReferenceFrame::J2000 && reference_frame != ReferenceFrame::TEME {
            panic!("FieldOfViewReport only supports J2000 and TEME reference frames.");
        }

        Self {
            epoch,
            sensor_position,
            sensor_direction: *sensor_direction,
            fov_angle,
            reference_frame,
            candidates: Vec::new(),
        }
    }

    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn get_reference_frame(&self) -> ReferenceFrame {
        self.reference_frame
    }

    pub fn get_sensor_position(&self) -> CartesianVector {
        self.sensor_position
    }

    pub fn get_sensor_direction(&self) -> TopocentricElements {
        self.sensor_direction
    }

    pub fn get_fov_angle(&self) -> f64 {
        self.fov_angle
    }

    pub fn get_candidates(&self) -> Vec<FieldOfViewCandidate> {
        self.candidates.clone()
    }
}
