use super::{PyCovariance, PyObservation, PyObservationResidual, PyTDOAObservation, PyFDOAObservation};
use crate::bindings::bodies::PySatellite;
use crate::bindings::enums::PyKeplerianType;
use crate::bindings::time::PyEpoch;
use crate::enums::KeplerianType;
use crate::estimation::{BatchLeastSquares, Observation, TDOAObservation, FDOAObservation};
use pyo3::prelude::*;

#[pyclass(name = "BatchLeastSquares")]
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

    #[staticmethod]
    pub fn from_mixed_observations(
        angle_obs: Vec<PyObservation>,
        tdoa_obs: Vec<PyTDOAObservation>,
        fdoa_obs: Vec<PyFDOAObservation>,
        a_priori: &PySatellite,
    ) -> Self {
        let angle_obs: Vec<Observation> = angle_obs.into_iter().map(Observation::from).collect();
        let tdoa_obs: Vec<TDOAObservation> = tdoa_obs.into_iter().map(TDOAObservation::from).collect();
        let fdoa_obs: Vec<FDOAObservation> = fdoa_obs.into_iter().map(FDOAObservation::from).collect();
        BatchLeastSquares::from_mixed_observations(angle_obs, tdoa_obs, fdoa_obs, a_priori.inner()).into()
    }

    pub fn solve(&mut self, py: Python<'_>) -> PyResult<()> {
        py.allow_threads(|| self.inner.solve())
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

    // Note: get_observations is not available because BatchLeastSquares now supports
    // mixed observation types (Observation, TDOAObservation, FDOAObservation) which
    // cannot be represented as a single Vec<Observation>

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
}
