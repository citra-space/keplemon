use super::PyForceProperties;
use crate::bindings::elements::PyTLE;
use crate::bindings::elements::{PyCartesianState, PyKeplerianState};
use crate::bindings::time::PyEpoch;
use crate::bodies::Satellite;
use crate::elements::TLE;
use crate::estimation::Observation;
use crate::propagation::InertialPropagator;
use crate::time::Epoch;
use nalgebra::{DMatrix, DVector};
use pyo3::prelude::*;

#[pyclass(name = "InertialPropagator")]
#[derive(Debug, PartialEq, Clone)]
pub struct PyInertialPropagator {
    inner: InertialPropagator,
}

impl From<InertialPropagator> for PyInertialPropagator {
    fn from(inner: InertialPropagator) -> Self {
        Self { inner }
    }
}

impl From<PyInertialPropagator> for InertialPropagator {
    fn from(value: PyInertialPropagator) -> Self {
        value.inner
    }
}

impl PyInertialPropagator {
    pub fn step_to_epoch(&mut self, epoch: PyEpoch) -> Result<(), String> {
        let epoch: Epoch = epoch.into();
        self.inner.step_to_epoch(epoch)
    }
}

#[pymethods]
impl PyInertialPropagator {
    #[staticmethod]
    pub fn from_tle(tle: PyTLE) -> Self {
        let tle: TLE = tle.into();
        InertialPropagator::from(tle).into()
    }

    pub fn get_cartesian_state_at_epoch(&self, epoch: PyEpoch) -> Option<PyCartesianState> {
        let epoch: Epoch = epoch.into();
        self.inner
            .get_cartesian_state_at_epoch(epoch)
            .map(PyCartesianState::from)
    }

    pub fn get_keplerian_state_at_epoch(&self, epoch: PyEpoch) -> Option<PyKeplerianState> {
        let epoch: Epoch = epoch.into();
        self.inner
            .get_keplerian_state_at_epoch(epoch)
            .map(PyKeplerianState::from)
    }

    #[getter]
    pub fn get_keplerian_state(&self) -> PyResult<PyKeplerianState> {
        self.inner
            .get_keplerian_state()
            .map(PyKeplerianState::from)
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    #[getter]
    pub fn get_force_properties(&self) -> PyResult<PyForceProperties> {
        self.inner
            .get_force_properties()
            .map(PyForceProperties::from)
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }
}

impl PyInertialPropagator {
    pub fn get_prior_node(&self, epoch: PyEpoch) -> Result<PyEpoch, String> {
        let epoch: Epoch = epoch.into();
        self.inner.get_prior_node(epoch).map(PyEpoch::from)
    }

    pub fn get_stm(&self, epoch: PyEpoch, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        let epoch: Epoch = epoch.into();
        self.inner.get_stm(epoch, use_drag, use_srp)
    }

    pub fn get_jacobian(&self, ob: &Observation, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        self.inner.get_jacobian(ob, use_drag, use_srp)
    }

    pub fn build_perturbed_satellites(&self, use_drag: bool, use_srp: bool) -> Result<Vec<(Satellite, f64)>, String> {
        self.inner.build_perturbed_satellites(use_drag, use_srp)
    }

    pub fn new_with_delta_x(&self, delta_x: &DVector<f64>, use_drag: bool, use_srp: bool) -> Result<Self, String> {
        self.inner
            .new_with_delta_x(delta_x, use_drag, use_srp)
            .map(PyInertialPropagator::from)
    }

    pub fn clone_at_epoch(&self, epoch: PyEpoch) -> Result<Self, String> {
        let epoch: Epoch = epoch.into();
        self.inner.clone_at_epoch(epoch).map(PyInertialPropagator::from)
    }
}
