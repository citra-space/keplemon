use super::PyCartesianState;
use super::PyKeplerianElements;
use crate::bindings::enums::{PyKeplerianType, PyReferenceFrame};
use crate::bindings::time::PyEpoch;
use crate::elements::{CartesianState, KeplerianState};
use crate::enums::{KeplerianType, ReferenceFrame};
use crate::time::Epoch;
use pyo3::prelude::*;

const XA_TLE_SIZE: usize = 64;

#[pyclass(name = "KeplerianState")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyKeplerianState {
    inner: KeplerianState,
}

impl PyKeplerianState {
    pub fn get_elements(&self) -> PyKeplerianElements {
        PyKeplerianElements::from(self.inner.elements)
    }

    pub fn from_xa_tle(xa_tle: &[f64; XA_TLE_SIZE]) -> Self {
        let inner = KeplerianState::from(xa_tle);
        Self { inner }
    }
}

impl From<KeplerianState> for PyKeplerianState {
    fn from(inner: KeplerianState) -> Self {
        Self { inner }
    }
}

impl From<PyKeplerianState> for KeplerianState {
    fn from(state: PyKeplerianState) -> Self {
        state.inner
    }
}

#[pymethods]
impl PyKeplerianState {
    #[new]
    pub fn new(
        epoch: PyEpoch,
        elements: PyKeplerianElements,
        frame: PyReferenceFrame,
        keplerian_type: PyKeplerianType,
    ) -> Self {
        let epoch: Epoch = epoch.into();
        let frame: ReferenceFrame = frame.into();
        let keplerian_type: KeplerianType = keplerian_type.into();
        Self {
            inner: KeplerianState::new(epoch, elements.into(), frame, keplerian_type),
        }
    }

    pub fn to_cartesian(&self) -> PyCartesianState {
        PyCartesianState::from(CartesianState::from(self.inner))
    }

    pub fn to_frame(&self, frame: PyReferenceFrame) -> PyKeplerianState {
        let frame: ReferenceFrame = frame.into();
        PyKeplerianState::from(self.inner.to_frame(frame))
    }

    #[getter]
    pub fn get_semi_major_axis(&self) -> f64 {
        self.inner.get_semi_major_axis()
    }

    #[getter]
    pub fn get_mean_anomaly(&self) -> f64 {
        self.inner.get_mean_anomaly()
    }

    #[getter]
    pub fn get_eccentricity(&self) -> f64 {
        self.inner.get_eccentricity()
    }

    #[getter]
    pub fn get_inclination(&self) -> f64 {
        self.inner.get_inclination()
    }

    #[getter]
    pub fn get_raan(&self) -> f64 {
        self.inner.get_raan()
    }

    #[getter]
    pub fn get_argument_of_perigee(&self) -> f64 {
        self.inner.get_argument_of_perigee()
    }

    #[getter]
    pub fn get_apoapsis(&self) -> f64 {
        self.inner.get_apoapsis()
    }

    #[getter]
    pub fn get_periapsis(&self) -> f64 {
        self.inner.get_periapsis()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.epoch.into()
    }

    #[getter]
    pub fn get_mean_motion(&self) -> f64 {
        self.inner.get_mean_motion()
    }

    #[getter]
    pub fn get_frame(&self) -> PyReferenceFrame {
        PyReferenceFrame::from(self.inner.get_frame())
    }

    #[getter]
    pub fn get_type(&self) -> PyKeplerianType {
        PyKeplerianType::from(self.inner.get_type())
    }

    #[setter]
    pub fn set_epoch(&mut self, epoch: PyEpoch) {
        let epoch: Epoch = epoch.into();
        self.inner.epoch = epoch;
    }

    #[setter]
    pub fn set_type(&mut self, keplerian_type: PyKeplerianType) {
        let keplerian_type: KeplerianType = keplerian_type.into();
        self.inner.keplerian_type = keplerian_type;
    }
}
