use super::PyCartesianState;
use crate::bindings::time::PyEpoch;
use crate::bindings::enums::{PyClassification, PyKeplerianType};
use crate::elements::TLE;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "TLE")]
#[derive(Debug, PartialEq, Clone)]
pub struct PyTLE {
    inner: TLE,
}

impl From<TLE> for PyTLE {
    fn from(inner: TLE) -> Self {
        Self { inner }
    }
}

impl From<PyTLE> for TLE {
    fn from(value: PyTLE) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyTLE {
    #[staticmethod]
    #[pyo3(signature = (line_1, line_2, line_3 = None))]
    pub fn from_lines(line_1: &str, line_2: &str, line_3: Option<&str>) -> PyResult<PyTLE> {
        match TLE::from_lines(line_1, line_2, line_3) {
            Ok(tle) => Ok(PyTLE::from(tle)),
            Err(e) => Err(PyValueError::new_err(e)),
        }
    }

    #[getter]
    pub fn get_lines(&self) -> (String, String) {
        self.inner.get_lines().unwrap()
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
    pub fn get_semi_major_axis(&self) -> f64 {
        self.inner.get_semi_major_axis()
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
    pub fn get_eccentricity(&self) -> f64 {
        self.inner.get_eccentricity()
    }

    #[getter]
    pub fn get_argument_of_perigee(&self) -> f64 {
        self.inner.get_argument_of_perigee()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.inner.get_name()
    }

    #[getter]
    pub fn get_mean_anomaly(&self) -> f64 {
        self.inner.get_mean_anomaly()
    }

    #[getter]
    pub fn get_mean_motion(&self) -> f64 {
        self.inner.get_mean_motion()
    }

    #[getter]
    pub fn get_type(&self) -> PyKeplerianType {
        PyKeplerianType::from(self.inner.get_type())
    }

    #[getter]
    pub fn get_b_star(&self) -> f64 {
        self.inner.get_b_star()
    }

    #[getter]
    pub fn get_mean_motion_dot(&self) -> f64 {
        self.inner.get_mean_motion_dot()
    }

    #[getter]
    pub fn get_mean_motion_dot_dot(&self) -> f64 {
        self.inner.get_mean_motion_dot_dot()
    }

    #[getter]
    pub fn get_agom(&self) -> f64 {
        self.inner.get_agom()
    }

    #[getter]
    pub fn get_b_term(&self) -> f64 {
        self.inner.get_b_term()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    #[getter]
    pub fn get_classification(&self) -> PyClassification {
        PyClassification::from(self.inner.classification)
    }

    #[getter]
    pub fn get_designator(&self) -> String {
        self.inner.designator.clone()
    }

    #[getter]
    pub fn get_norad_id(&self) -> i32 {
        self.inner.norad_id
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.inner.satellite_id.clone()
    }

    #[getter]
    fn get_cartesian_state(&self) -> PyCartesianState {
        PyCartesianState::from(self.inner.get_cartesian_state())
    }
}
