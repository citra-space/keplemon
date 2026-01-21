use super::{PyCartesianState, PyKeplerianState};
use crate::bindings::time::PyEpoch;
use crate::elements::{CartesianState, KeplerianState, OrbitPlotData, OrbitPlotState};
use pyo3::prelude::*;

#[pyclass(name = "OrbitPlotState")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyOrbitPlotState {
    inner: OrbitPlotState,
}

impl From<OrbitPlotState> for PyOrbitPlotState {
    fn from(inner: OrbitPlotState) -> Self {
        Self { inner }
    }
}

impl From<PyOrbitPlotState> for OrbitPlotState {
    fn from(value: PyOrbitPlotState) -> Self {
        value.inner
    }
}

impl PyOrbitPlotState {
    pub fn from_keplerian_state(keplerian_state: &PyKeplerianState) -> Self {
        OrbitPlotState::from(&KeplerianState::from(*keplerian_state)).into()
    }

    pub fn from_cartesian_state(cartesian_state: &PyCartesianState) -> Self {
        OrbitPlotState::from(CartesianState::from(*cartesian_state)).into()
    }
}

#[pymethods]
impl PyOrbitPlotState {
    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    #[getter]
    pub fn get_latitude(&self) -> f64 {
        self.inner.get_latitude()
    }

    #[getter]
    pub fn get_longitude(&self) -> f64 {
        self.inner.get_longitude()
    }

    #[getter]
    pub fn get_altitude(&self) -> f64 {
        self.inner.get_altitude()
    }

    #[getter]
    pub fn get_semi_major_axis(&self) -> f64 {
        self.inner.get_semi_major_axis()
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
    pub fn get_radius(&self) -> f64 {
        self.inner.get_radius()
    }

    #[getter]
    pub fn get_apogee_radius(&self) -> f64 {
        self.inner.get_apogee_radius()
    }

    #[getter]
    pub fn get_perigee_radius(&self) -> f64 {
        self.inner.get_perigee_radius()
    }
}

#[pyclass(name = "OrbitPlotData")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyOrbitPlotData {
    inner: OrbitPlotData,
}

impl From<OrbitPlotData> for PyOrbitPlotData {
    fn from(inner: OrbitPlotData) -> Self {
        Self { inner }
    }
}

impl From<PyOrbitPlotData> for OrbitPlotData {
    fn from(value: PyOrbitPlotData) -> Self {
        value.inner
    }
}

impl PyOrbitPlotData {
    pub fn new(satellite_id: String) -> Self {
        Self {
            inner: OrbitPlotData::new(satellite_id),
        }
    }

    pub fn add_state(&mut self, plot_state: PyOrbitPlotState) {
        self.inner.add_state(plot_state.into());
    }
}

#[pymethods]
impl PyOrbitPlotData {
    #[getter]
    pub fn get_satellite_id(&self) -> String {
        self.inner.get_satellite_id()
    }

    #[getter]
    pub fn get_epochs(&self) -> Vec<String> {
        self.inner.get_epochs()
    }

    #[getter]
    pub fn get_latitudes(&self) -> Vec<f64> {
        self.inner.get_latitudes()
    }

    #[getter]
    pub fn get_longitudes(&self) -> Vec<f64> {
        self.inner.get_longitudes()
    }

    #[getter]
    pub fn get_altitudes(&self) -> Vec<f64> {
        self.inner.get_altitudes()
    }

    #[getter]
    pub fn get_semi_major_axes(&self) -> Vec<f64> {
        self.inner.get_semi_major_axes()
    }

    #[getter]
    pub fn get_eccentricities(&self) -> Vec<f64> {
        self.inner.get_eccentricities()
    }

    #[getter]
    pub fn get_inclinations(&self) -> Vec<f64> {
        self.inner.get_inclinations()
    }

    #[getter]
    pub fn get_raans(&self) -> Vec<f64> {
        self.inner.get_raans()
    }

    #[getter]
    pub fn get_radii(&self) -> Vec<f64> {
        self.inner.get_radii()
    }

    #[getter]
    pub fn get_apogee_radii(&self) -> Vec<f64> {
        self.inner.get_apogee_radii()
    }

    #[getter]
    pub fn get_perigee_radii(&self) -> Vec<f64> {
        self.inner.get_perigee_radii()
    }
}
