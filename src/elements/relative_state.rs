use super::CartesianVector;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct RelativeState {
    pub epoch: Epoch,
    pub position: CartesianVector,
    pub velocity: CartesianVector,
    pub origin_satellite_id: String,
    pub secondary_satellite_id: String,
}

#[pymethods]
impl RelativeState {
    #[new]
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

    #[getter]
    pub fn get_position(&self) -> CartesianVector {
        self.position
    }

    #[getter]
    pub fn get_velocity(&self) -> CartesianVector {
        self.velocity
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_origin_satellite_id(&self) -> String {
        self.origin_satellite_id.clone()
    }

    #[getter]
    pub fn get_secondary_satellite_id(&self) -> String {
        self.secondary_satellite_id.clone()
    }
}
