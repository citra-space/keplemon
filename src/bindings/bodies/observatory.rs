use super::{PyConstellation, PySatellite, PySensor};
use crate::bindings::elements::{PyCartesianState, PyTopocentricElements};
use crate::bindings::enums::PyReferenceFrame;
use crate::bindings::events::PyFieldOfViewReport;
use crate::bindings::time::PyEpoch;
use crate::bodies::{Observatory, Sensor};
use crate::elements::{CartesianState, Ephemeris};
use crate::enums::ReferenceFrame;
use crate::time::{Epoch, TimeSpan};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass(name = "Observatory")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyObservatory {
    inner: Observatory,
}

impl From<Observatory> for PyObservatory {
    fn from(inner: Observatory) -> Self {
        Self { inner }
    }
}

impl From<PyObservatory> for Observatory {
    fn from(value: PyObservatory) -> Self {
        value.inner
    }
}

impl PyObservatory {
    pub fn inner(&self) -> &Observatory {
        &self.inner
    }

    pub fn get_ephemeris(&self, start_epoch: PyEpoch, end_epoch: PyEpoch, step: TimeSpan) -> Ephemeris {
        let start_epoch: Epoch = start_epoch.into();
        let end_epoch: Epoch = end_epoch.into();
        self.inner.get_ephemeris(start_epoch, end_epoch, step)
    }
}

#[pymethods]
impl PyObservatory {
    #[new]
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        Observatory::new(latitude, longitude, altitude).into()
    }

    #[staticmethod]
    pub fn from_cartesian_state(state: PyCartesianState) -> Self {
        Observatory::from(CartesianState::from(state)).into()
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.inner.id.clone()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[getter]
    pub fn get_latitude(&self) -> f64 {
        self.inner.latitude
    }

    #[getter]
    pub fn get_longitude(&self) -> f64 {
        self.inner.longitude
    }

    #[getter]
    pub fn get_altitude(&self) -> f64 {
        self.inner.altitude
    }

    #[getter]
    pub fn get_sensors(&self) -> Vec<PySensor> {
        self.inner.sensors.iter().cloned().map(PySensor::from).collect()
    }

    #[setter]
    pub fn set_id(&mut self, site_id: String) {
        self.inner.id = site_id;
    }

    #[setter]
    pub fn set_name(&mut self, name: Option<String>) {
        self.inner.name = name;
    }

    #[setter]
    pub fn set_latitude(&mut self, latitude: f64) {
        self.inner.latitude = latitude;
    }

    #[setter]
    pub fn set_longitude(&mut self, longitude: f64) {
        self.inner.longitude = longitude;
    }

    #[setter]
    pub fn set_altitude(&mut self, altitude: f64) {
        self.inner.altitude = altitude;
    }

    pub fn add_sensor(&mut self, sensor: PySensor) {
        let sensor: Sensor = sensor.into();
        self.inner.add_sensor(sensor);
    }

    pub fn get_topocentric_to_satellite(
        &self,
        epoch: PyEpoch,
        sat: &PySatellite,
        reference_frame: PyReferenceFrame,
    ) -> PyResult<PyTopocentricElements> {
        let epoch: Epoch = epoch.into();
        let reference_frame: ReferenceFrame = reference_frame.into();
        self.inner
            .get_topocentric_to_satellite(epoch, sat.inner(), reference_frame)
            .map(PyTopocentricElements::from)
            .map_err(PyRuntimeError::new_err)
    }

    pub fn get_field_of_view_report(
        &self,
        py: Python<'_>,
        epoch: PyEpoch,
        sensor_direction: PyTopocentricElements,
        angular_threshold: f64,
        sats: PyConstellation,
        reference_frame: PyReferenceFrame,
    ) -> PyFieldOfViewReport {
        let epoch: Epoch = epoch.into();
        let reference_frame: ReferenceFrame = reference_frame.into();
        py.detach(|| {
            PyFieldOfViewReport::from(self.inner.get_field_of_view_report(
                epoch,
                sensor_direction.into(),
                angular_threshold,
                sats.into(),
                reference_frame,
            ))
        })
    }

    pub fn get_state_at_epoch(&self, epoch: PyEpoch) -> PyCartesianState {
        let epoch: Epoch = epoch.into();
        PyCartesianState::from(self.inner.get_state_at_epoch(epoch))
    }

    pub fn get_theta(&self, epoch: PyEpoch) -> f64 {
        let epoch: Epoch = epoch.into();
        self.inner.get_theta(epoch)
    }
}
