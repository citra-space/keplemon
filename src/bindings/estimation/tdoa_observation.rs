use super::{PyObservationResidual, PyObservation};
use crate::bindings::bodies::{PySatellite, PySensor};
use crate::bindings::elements::PyCartesianVector;
use crate::bindings::time::PyEpoch;
use crate::estimation::{TDOAObservation, ObservationType};
use pyo3::prelude::*;

#[pyclass(name = "TDOAObservation")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyTDOAObservation {
    inner: TDOAObservation,
}

impl From<TDOAObservation> for PyTDOAObservation {
    fn from(inner: TDOAObservation) -> Self {
        Self { inner }
    }
}

impl From<PyTDOAObservation> for TDOAObservation {
    fn from(value: PyTDOAObservation) -> Self {
        value.inner
    }
}

impl PyTDOAObservation {
    pub fn inner(&self) -> &TDOAObservation {
        &self.inner
    }
}

#[pymethods]
impl PyTDOAObservation {
    #[new]
    pub fn new(
        sensor_1: PySensor,
        sensor_2: PySensor,
        epoch: PyEpoch,
        time_difference: f64,
        observer_1_teme_position: PyCartesianVector,
        observer_2_teme_position: PyCartesianVector,
    ) -> Self {
        TDOAObservation::new(
            sensor_1.into(),
            sensor_2.into(),
            epoch.into(),
            time_difference,
            observer_1_teme_position.into(),
            observer_2_teme_position.into(),
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
    pub fn time_difference(&self) -> f64 {
        self.inner.time_difference
    }

    #[getter]
    pub fn observer_1_teme_position(&self) -> PyCartesianVector {
        self.inner.observer_1_teme_position.into()
    }

    #[getter]
    pub fn observer_2_teme_position(&self) -> PyCartesianVector {
        self.inner.observer_2_teme_position.into()
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
    pub fn set_time_difference(&mut self, time_difference: f64) {
        self.inner.time_difference = time_difference;
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
