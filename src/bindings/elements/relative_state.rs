use super::PyCartesianVector;
use crate::bindings::time::PyEpoch;
use crate::elements::RelativeState;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "RelativeState")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyRelativeState {
    inner: RelativeState,
}

impl From<RelativeState> for PyRelativeState {
    fn from(inner: RelativeState) -> Self {
        Self { inner }
    }
}

impl From<PyRelativeState> for RelativeState {
    fn from(value: PyRelativeState) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyRelativeState {
    #[new]
    pub fn new(
        epoch: PyEpoch,
        position: PyCartesianVector,
        velocity: PyCartesianVector,
        origin_id: String,
        secondary_id: String,
    ) -> Self {
        let epoch: Epoch = epoch.into();
        RelativeState::new(epoch, position.into(), velocity.into(), origin_id, secondary_id).into()
    }

    #[getter]
    pub fn get_position(&self) -> PyCartesianVector {
        PyCartesianVector::from(self.inner.position)
    }

    #[getter]
    pub fn get_velocity(&self) -> PyCartesianVector {
        PyCartesianVector::from(self.inner.velocity)
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.epoch.into()
    }

    #[getter]
    pub fn get_origin_satellite_id(&self) -> String {
        self.inner.origin_satellite_id.clone()
    }

    #[getter]
    pub fn get_secondary_satellite_id(&self) -> String {
        self.inner.secondary_satellite_id.clone()
    }
}
