use super::{PyObservationAssociation, PyObservationResidual};
use crate::bindings::bodies::{PyConstellation, PySatellite, PySensor};
use crate::bindings::elements::{PyCartesianVector, PyTopocentricElements};
use crate::bindings::time::PyEpoch;
use crate::bodies::Satellite;
use crate::elements::CartesianState;
use crate::estimation::Observation;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "Observation")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyObservation {
    inner: Observation,
}

impl From<Observation> for PyObservation {
    fn from(inner: Observation) -> Self {
        Self { inner }
    }
}

impl From<PyObservation> for Observation {
    fn from(value: PyObservation) -> Self {
        value.inner
    }
}

impl PyObservation {
    pub fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        self.inner.get_measurement_and_weight_vector()
    }

    pub fn get_predicted_vector(&self, satellite: &Satellite) -> Result<Vec<f64>, String> {
        self.inner.get_predicted_vector(satellite)
    }

    pub fn fill_predicted_vector(&self, satellite: &Satellite, out: &mut Vec<f64>) -> Result<(), String> {
        self.inner.fill_predicted_vector(satellite, out)
    }

    pub fn fill_predicted_from_state(&self, state: &CartesianState, out: &mut Vec<f64>) -> Result<(), String> {
        self.inner.fill_predicted_from_state(state, out)
    }
}

#[pymethods]
impl PyObservation {
    #[new]
    pub fn new(
        sensor: PySensor,
        epoch: PyEpoch,
        observed_teme_topocentric: PyTopocentricElements,
        observer_teme_position: PyCartesianVector,
    ) -> Self {
        let epoch: Epoch = epoch.into();
        Observation::new(
            sensor.into(),
            epoch,
            observed_teme_topocentric.into(),
            observer_teme_position.into(),
        )
        .into()
    }

    #[staticmethod]
    pub fn from_saal_files(sensor_file: &str, observation_file: &str) -> PyResult<Vec<PyObservation>> {
        Observation::from_saal_files(sensor_file, observation_file)
            .map(|observations| observations.into_iter().map(PyObservation::from).collect())
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    #[getter]
    pub fn get_sensor(&self) -> PySensor {
        PySensor::from(self.inner.get_sensor())
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.inner.id.clone()
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.inner.get_range()
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.inner.get_range_rate()
    }

    #[getter]
    pub fn get_right_ascension(&self) -> f64 {
        self.inner.get_right_ascension()
    }

    #[getter]
    pub fn get_declination(&self) -> f64 {
        self.inner.get_declination()
    }

    #[getter]
    pub fn get_right_ascension_rate(&self) -> Option<f64> {
        self.inner.get_right_ascension_rate()
    }

    #[getter]
    pub fn get_declination_rate(&self) -> Option<f64> {
        self.inner.get_declination_rate()
    }

    #[getter]
    pub fn get_observed_satellite_id(&self) -> Option<String> {
        self.inner.observed_satellite_id.clone()
    }

    #[setter]
    pub fn set_range(&mut self, range: Option<f64>) {
        self.inner.set_range(range);
    }

    #[setter]
    pub fn set_id(&mut self, id: String) {
        self.inner.id = id;
    }

    #[setter]
    pub fn set_range_rate(&mut self, range_rate: Option<f64>) {
        self.inner.set_range_rate(range_rate);
    }

    #[setter]
    pub fn set_right_ascension(&mut self, right_ascension: f64) {
        self.inner.set_right_ascension(right_ascension);
    }

    #[setter]
    pub fn set_declination(&mut self, declination: f64) {
        self.inner.set_declination(declination);
    }

    #[setter]
    pub fn set_observed_satellite_id(&mut self, observed_satellite_id: Option<String>) {
        self.inner.observed_satellite_id = observed_satellite_id;
    }

    pub fn get_associations(&self, py: Python<'_>, constellation: &PyConstellation) -> Vec<PyObservationAssociation> {
        py.detach(|| {
            self.inner
                .get_associations(constellation.inner())
                .into_iter()
                .map(PyObservationAssociation::from)
                .collect()
        })
    }

    pub fn get_residual(&self, satellite: &PySatellite) -> Option<PyObservationResidual> {
        self.inner
            .get_residual(satellite.inner())
            .map(PyObservationResidual::from)
    }
}
