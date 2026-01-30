use super::{PyCovariance, PyObservation, PyObservationResidual};
use crate::bindings::bodies::PySatellite;
use crate::bindings::elements::PyCartesianVector;
use crate::bindings::enums::PyKeplerianType;
use crate::bindings::time::PyEpoch;
use crate::enums::KeplerianType;
use crate::estimation::{BatchLeastSquares, Observation};
use pyo3::prelude::*;

#[pyclass(name = "BatchLeastSquares")]
#[derive(Debug, Clone)]
pub struct PyBatchLeastSquares {
    inner: BatchLeastSquares,
}

impl From<BatchLeastSquares> for PyBatchLeastSquares {
    fn from(inner: BatchLeastSquares) -> Self {
        Self { inner }
    }
}

impl From<PyBatchLeastSquares> for BatchLeastSquares {
    fn from(value: PyBatchLeastSquares) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyBatchLeastSquares {
    #[new]
    pub fn new(obs: Vec<PyObservation>, a_priori: &PySatellite) -> Self {
        let obs: Vec<Observation> = obs.into_iter().map(Observation::from).collect();
        BatchLeastSquares::new(obs, a_priori.inner()).into()
    }

    pub fn solve(&mut self, py: Python<'_>) -> PyResult<()> {
        py.detach(|| self.inner.solve())
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    #[getter]
    pub fn get_output_type(&self) -> PyKeplerianType {
        PyKeplerianType::from(self.inner.get_output_type())
    }

    #[setter]
    pub fn set_output_type(&mut self, output_keplerian_type: PyKeplerianType) {
        let output_keplerian_type: KeplerianType = output_keplerian_type.into();
        self.inner.set_output_type(output_keplerian_type);
    }

    #[getter]
    pub fn get_converged(&self) -> bool {
        self.inner.get_converged()
    }

    #[getter]
    pub fn get_current_estimate(&self) -> PySatellite {
        PySatellite::from(self.inner.get_current_estimate())
    }

    #[getter]
    pub fn get_iteration_count(&self) -> usize {
        self.inner.get_iteration_count()
    }

    #[getter]
    pub fn get_weighted_rms(&self) -> Option<f64> {
        self.inner.get_weighted_rms()
    }

    #[getter]
    pub fn get_rms(&self) -> Option<f64> {
        self.inner.get_rms()
    }

    #[setter]
    pub fn set_a_priori(&mut self, a_priori: &PySatellite) {
        self.inner.set_a_priori(a_priori.inner());
    }

    #[setter]
    pub fn set_observations(&mut self, obs: Vec<PyObservation>) {
        let obs: Vec<Observation> = obs.into_iter().map(Observation::from).collect();
        self.inner.set_observations(obs);
    }

    #[getter]
    pub fn get_observations(&self) -> Vec<PyObservation> {
        self.inner
            .get_observations()
            .into_iter()
            .map(PyObservation::from)
            .collect()
    }

    #[getter]
    pub fn get_residuals(&self) -> Vec<(PyEpoch, PyObservationResidual)> {
        self.inner
            .get_residuals()
            .into_iter()
            .map(|(epoch, residual)| (PyEpoch::from(epoch), PyObservationResidual::from(residual)))
            .collect()
    }

    #[setter]
    pub fn set_max_iterations(&mut self, max_iterations: usize) {
        self.inner.set_max_iterations(max_iterations);
    }

    #[getter]
    pub fn get_max_iterations(&self) -> usize {
        self.inner.get_max_iterations()
    }

    #[setter]
    pub fn set_estimate_drag(&mut self, use_drag: bool) {
        self.inner.set_estimate_drag(use_drag);
    }

    #[getter]
    pub fn get_estimate_drag(&self) -> bool {
        self.inner.get_estimate_drag()
    }

    #[setter]
    pub fn set_estimate_srp(&mut self, use_srp: bool) {
        self.inner.set_estimate_srp(use_srp);
    }

    #[getter]
    pub fn get_estimate_srp(&self) -> bool {
        self.inner.get_estimate_srp()
    }

    #[getter]
    pub fn get_eccentricity_constraint_weight(&self) -> Option<f64> {
        self.inner.get_eccentricity_constraint_weight()
    }

    #[setter]
    pub fn set_eccentricity_constraint_weight(&mut self, weight: Option<f64>) {
        self.inner.set_eccentricity_constraint_weight(weight);
    }

    #[getter]
    pub fn get_covariance(&self) -> Option<PyCovariance> {
        self.inner.get_covariance().map(PyCovariance::from)
    }

    #[setter]
    pub fn set_estimate_maneuver(&mut self, estimate_maneuver: bool) {
        self.inner.set_estimate_maneuver(estimate_maneuver);
    }

    #[getter]
    pub fn get_estimate_maneuver(&self) -> bool {
        self.inner.get_estimate_maneuver()
    }

    #[getter]
    pub fn get_maneuver_epoch(&self) -> Option<PyEpoch> {
        self.inner.get_maneuver_epoch().map(PyEpoch::from)
    }

    #[getter]
    pub fn get_delta_v(&self) -> Option<PyCartesianVector> {
        self.inner.get_delta_v().map(PyCartesianVector::from)
    }

    #[setter]
    pub fn set_allow_radial_delta_v(&mut self, allow: bool) {
        self.inner.set_allow_radial_delta_v(allow);
    }

    #[getter]
    pub fn get_allow_radial_delta_v(&self) -> bool {
        self.inner.get_allow_radial_delta_v()
    }
}
