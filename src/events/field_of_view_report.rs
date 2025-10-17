use super::FieldOfViewCandidate;
use crate::elements::{CartesianVector, TopocentricElements};
use crate::enums::ReferenceFrame;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
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
}

#[pymethods]
impl FieldOfViewReport {
    #[new]
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
            sensor_direction: sensor_direction.clone(),
            fov_angle,
            reference_frame,
            candidates: Vec::new(),
        }
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_reference_frame(&self) -> ReferenceFrame {
        self.reference_frame
    }

    #[getter]
    pub fn get_sensor_position(&self) -> CartesianVector {
        self.sensor_position
    }

    #[getter]
    pub fn get_sensor_direction(&self) -> TopocentricElements {
        self.sensor_direction.clone()
    }

    #[getter]
    pub fn get_fov_angle(&self) -> f64 {
        self.fov_angle
    }

    #[getter]
    pub fn get_candidates(&self) -> Vec<FieldOfViewCandidate> {
        self.candidates.clone()
    }
}
