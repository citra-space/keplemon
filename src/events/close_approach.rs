use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
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
}

#[pymethods]
impl CloseApproach {
    #[getter]
    pub fn get_primary_id(&self) -> String {
        self.primary_id.clone()
    }

    #[getter]
    pub fn get_secondary_id(&self) -> String {
        self.secondary_id.clone()
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_distance(&self) -> f64 {
        self.distance
    }
}
