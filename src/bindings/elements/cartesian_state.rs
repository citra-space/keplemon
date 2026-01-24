use super::{PyCartesianVector, PyKeplerianState};
use crate::bindings::enums::PyReferenceFrame;
use crate::bindings::time::PyEpoch;
use crate::elements::{CartesianState, KeplerianState};
use crate::enums::ReferenceFrame;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "CartesianState")]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PyCartesianState {
    inner: CartesianState,
}

impl From<CartesianState> for PyCartesianState {
    fn from(inner: CartesianState) -> Self {
        Self { inner }
    }
}

impl From<PyCartesianState> for CartesianState {
    fn from(state: PyCartesianState) -> Self {
        state.inner
    }
}

#[pymethods]
impl PyCartesianState {
    #[new]
    pub fn new(
        epoch: PyEpoch,
        position: PyCartesianVector,
        velocity: PyCartesianVector,
        frame: PyReferenceFrame,
    ) -> Self {
        let epoch: Epoch = epoch.into();
        let frame: ReferenceFrame = frame.into();
        CartesianState::new(epoch, position.into(), velocity.into(), frame).into()
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
    pub fn get_frame(&self) -> PyReferenceFrame {
        PyReferenceFrame::from(self.inner.get_frame())
    }

    pub fn to_keplerian(&self) -> PyKeplerianState {
        let state = KeplerianState::from(self.inner);
        PyKeplerianState::from(state)
    }

    pub fn to_frame(&self, frame: PyReferenceFrame) -> PyCartesianState {
        let frame: ReferenceFrame = frame.into();
        PyCartesianState::from(self.inner.to_frame(frame))
    }
}
