use crate::elements::TopocentricElements;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfViewCandidate {
    satellite_id: String,
    direction: TopocentricElements,
}

impl FieldOfViewCandidate {
    pub fn new(satellite_id: String, direction: &TopocentricElements) -> Self {
        Self {
            satellite_id,
            direction: *direction,
        }
    }
}

#[pymethods]
impl FieldOfViewCandidate {
    #[getter]
    pub fn get_satellite_id(&self) -> String {
        self.satellite_id.clone()
    }

    #[getter]
    pub fn get_direction(&self) -> TopocentricElements {
        self.direction
    }
}
