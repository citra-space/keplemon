use crate::propagation::ForceProperties;
use pyo3::prelude::*;

const XA_TLE_SIZE: usize = 64;

#[pyclass(name = "ForceProperties")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyForceProperties {
    inner: ForceProperties,
}

impl Default for PyForceProperties {
    fn default() -> Self {
        Self {
            inner: ForceProperties::default(),
        }
    }
}

impl From<ForceProperties> for PyForceProperties {
    fn from(inner: ForceProperties) -> Self {
        Self { inner }
    }
}

impl From<PyForceProperties> for ForceProperties {
    fn from(value: PyForceProperties) -> Self {
        value.inner
    }
}

impl PyForceProperties {
    pub fn from_xa_tle(xa_tle: &[f64; XA_TLE_SIZE]) -> Self {
        ForceProperties::from(xa_tle).into()
    }
}

#[pymethods]
impl PyForceProperties {
    #[new]
    pub fn new(
        srp_coefficient: f64,
        srp_area: f64,
        drag_coefficient: f64,
        drag_area: f64,
        mass: f64,
        mean_motion_dot: f64,
        mean_motion_dot_dot: f64,
    ) -> Self {
        Self {
            inner: ForceProperties::new(
                srp_coefficient,
                srp_area,
                drag_coefficient,
                drag_area,
                mass,
                mean_motion_dot,
                mean_motion_dot_dot,
            ),
        }
    }

    #[getter]
    pub fn get_srp_term(&self) -> f64 {
        self.inner.get_srp_term()
    }

    #[getter]
    pub fn get_drag_term(&self) -> f64 {
        self.inner.get_drag_term()
    }

    #[getter]
    pub fn get_b_star(&self) -> f64 {
        self.inner.get_b_star()
    }

    #[getter]
    pub fn get_mass(&self) -> f64 {
        self.inner.mass
    }

    #[getter]
    pub fn get_mean_motion_dot(&self) -> f64 {
        self.inner.mean_motion_dot
    }

    #[getter]
    pub fn get_mean_motion_dot_dot(&self) -> f64 {
        self.inner.mean_motion_dot_dot
    }

    #[getter]
    pub fn get_srp_coefficient(&self) -> f64 {
        self.inner.srp_coefficient
    }

    #[getter]
    pub fn get_drag_coefficient(&self) -> f64 {
        self.inner.drag_coefficient
    }

    #[getter]
    pub fn get_drag_area(&self) -> f64 {
        self.inner.drag_area
    }

    #[setter]
    pub fn set_srp_coefficient(&mut self, srp_coefficient: f64) {
        self.inner.srp_coefficient = srp_coefficient;
    }

    #[setter]
    pub fn set_srp_area(&mut self, srp_area: f64) {
        self.inner.srp_area = srp_area;
    }

    #[setter]
    pub fn set_drag_coefficient(&mut self, drag_coefficient: f64) {
        self.inner.drag_coefficient = drag_coefficient;
    }

    #[setter]
    pub fn set_drag_area(&mut self, drag_area: f64) {
        self.inner.drag_area = drag_area;
    }

    #[setter]
    pub fn set_mass(&mut self, mass: f64) {
        self.inner.mass = mass;
    }

    #[setter]
    pub fn set_mean_motion_dot(&mut self, mean_motion_dot: f64) {
        self.inner.mean_motion_dot = mean_motion_dot;
    }

    #[setter]
    pub fn set_mean_motion_dot_dot(&mut self, mean_motion_dot_dot: f64) {
        self.inner.mean_motion_dot_dot = mean_motion_dot_dot;
    }
}
