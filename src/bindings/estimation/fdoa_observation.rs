use super::{PyObservationResidual, PyObservation};
use crate::bindings::bodies::{PySatellite, PySensor};
use crate::bindings::elements::PyCartesianState;
use crate::bindings::time::PyEpoch;
use crate::estimation::{FDOAObservation, ObservationType};
use pyo3::prelude::*;

#[pyclass(name = "FDOAObservation")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyFDOAObservation {
    inner: FDOAObservation,
}

impl From<FDOAObservation> for PyFDOAObservation {
    fn from(inner: FDOAObservation) -> Self {
        Self { inner }
    }
}

impl From<PyFDOAObservation> for FDOAObservation {
    fn from(value: PyFDOAObservation) -> Self {
        value.inner
    }
}

impl PyFDOAObservation {
    pub fn inner(&self) -> &FDOAObservation {
        &self.inner
    }
}

#[pymethods]
impl PyFDOAObservation {
    #[new]
    pub fn new(
        sensor_1: PySensor,
        sensor_2: PySensor,
        epoch: PyEpoch,
        frequency_difference: f64,
        observer_1_teme_state: PyCartesianState,
        observer_2_teme_state: PyCartesianState,
        transmit_frequency: f64,
    ) -> Self {
        FDOAObservation::new(
            sensor_1.into(),
            sensor_2.into(),
            epoch.into(),
            frequency_difference,
            observer_1_teme_state.into(),
            observer_2_teme_state.into(),
            transmit_frequency,
        )
        .into()
    }

    #[getter]
    pub fn id(&self) -> String {
        self.inner.id().to_string()
    }

    #[getter]
    pub fn sensor_1(&self) -> PySensor {
        self.inner.sensor_1.clone().into()
    }

    #[getter]
    pub fn sensor_2(&self) -> PySensor {
        self.inner.sensor_2.clone().into()
    }

    #[getter]
    pub fn epoch(&self) -> PyEpoch {
        self.inner.epoch.into()
    }

    #[getter]
    pub fn frequency_difference(&self) -> f64 {
        self.inner.frequency_difference
    }

    #[getter]
    pub fn observer_1_teme_state(&self) -> PyCartesianState {
        self.inner.observer_1_teme_state.into()
    }

    #[getter]
    pub fn observer_2_teme_state(&self) -> PyCartesianState {
        self.inner.observer_2_teme_state.into()
    }

    #[getter]
    pub fn transmit_frequency(&self) -> f64 {
        self.inner.transmit_frequency
    }

    #[getter]
    pub fn observed_satellite_id(&self) -> Option<i32> {
        self.inner.observed_satellite_id
    }

    #[setter]
    pub fn set_id(&mut self, id: String) {
        self.inner.id = id;
    }

    #[setter]
    pub fn set_frequency_difference(&mut self, frequency_difference: f64) {
        self.inner.frequency_difference = frequency_difference;
    }

    #[setter]
    pub fn set_observed_satellite_id(&mut self, observed_satellite_id: i32) {
        self.inner.observed_satellite_id = Some(observed_satellite_id);
    }

    pub fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        self.inner.get_measurement_and_weight_vector()
    }

    pub fn get_predicted_vector(&self, satellite: &PySatellite) -> PyResult<Vec<f64>> {
        self.inner
            .get_predicted_vector(satellite.inner())
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))
    }

    pub fn get_residual(&self, satellite: &PySatellite) -> Option<PyObservationResidual> {
        self.inner
            .get_residual(satellite.inner())
            .map(PyObservationResidual::from)
    }
}
