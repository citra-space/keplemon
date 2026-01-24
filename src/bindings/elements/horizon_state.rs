use super::{PyCartesianState, PyHorizonElements, PyTopocentricState};
use crate::bindings::bodies::PyObservatory;
use crate::bindings::time::PyEpoch;
use crate::bodies::Observatory;
use crate::elements::{CartesianState, HorizonState};
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "HorizonState")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyHorizonState {
    inner: HorizonState,
}

impl Copy for PyHorizonState {}

impl From<HorizonState> for PyHorizonState {
    fn from(inner: HorizonState) -> Self {
        Self { inner }
    }
}

impl From<PyHorizonState> for HorizonState {
    fn from(value: PyHorizonState) -> Self {
        value.inner
    }
}

impl PyHorizonState {
    pub fn new(epoch: PyEpoch, elements: PyHorizonElements) -> Self {
        let epoch: Epoch = epoch.into();
        Self {
            inner: HorizonState::new(epoch, elements.into()),
        }
    }

    pub fn from_topocentric_state(state: &PyTopocentricState, observer: &Observatory) -> Result<Self, String> {
        let core_state = (*state).into();
        Ok(HorizonState::from((&core_state, observer)).into())
    }
}

#[pymethods]
impl PyHorizonState {
    #[new]
    pub fn py_new(epoch: PyEpoch, elements: PyHorizonElements) -> Self {
        Self::new(epoch, elements)
    }

    #[staticmethod]
    #[pyo3(name = "from_topocentric_state")]
    pub fn py_from_topocentric_state(state: &PyTopocentricState, observer: &PyObservatory) -> PyResult<Self> {
        Self::from_topocentric_state(state, observer.inner()).map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
    }

    #[staticmethod]
    pub fn from_teme_states(sensor_teme: PyCartesianState, target_teme: PyCartesianState) -> Self {
        let core_sensor = CartesianState::from(sensor_teme);
        let core_target = CartesianState::from(target_teme);
        HorizonState::from((core_sensor, core_target)).into()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.epoch.into()
    }

    #[getter]
    pub fn get_elements(&self) -> PyHorizonElements {
        PyHorizonElements::from(self.inner.elements)
    }

    #[getter]
    pub fn get_azimuth(&self) -> f64 {
        self.inner.elements.azimuth
    }

    #[getter]
    pub fn get_elevation(&self) -> f64 {
        self.inner.elements.elevation
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.inner.elements.range
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.inner.elements.range_rate
    }

    #[getter]
    pub fn get_azimuth_rate(&self) -> Option<f64> {
        self.inner.elements.azimuth_rate
    }

    #[getter]
    pub fn get_elevation_rate(&self) -> Option<f64> {
        self.inner.elements.elevation_rate
    }

    #[setter]
    pub fn set_elements(&mut self, elements: PyHorizonElements) {
        self.inner.elements = elements.into();
    }

    #[setter]
    pub fn set_epoch(&mut self, epoch: PyEpoch) {
        let epoch: Epoch = epoch.into();
        self.inner.epoch = epoch;
    }

    #[setter]
    pub fn set_azimuth(&mut self, azimuth: f64) {
        self.inner.elements.azimuth = azimuth;
    }

    #[setter]
    pub fn set_elevation(&mut self, elevation: f64) {
        self.inner.elements.elevation = elevation;
    }

    #[setter]
    pub fn set_range(&mut self, range: Option<f64>) {
        self.inner.elements.range = range;
    }

    #[setter]
    pub fn set_range_rate(&mut self, range_rate: Option<f64>) {
        self.inner.elements.range_rate = range_rate;
    }

    #[setter]
    pub fn set_azimuth_rate(&mut self, azimuth_rate: Option<f64>) {
        self.inner.elements.azimuth_rate = azimuth_rate;
    }

    #[setter]
    pub fn set_elevation_rate(&mut self, elevation_rate: Option<f64>) {
        self.inner.elements.elevation_rate = elevation_rate;
    }
}
