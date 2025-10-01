use crate::configs;
use crate::elements::B_STAR_TO_B_TERM;
use crate::enums::KeplerianType;
use crate::saal::tle_interface;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForceProperties {
    srp_coefficient: f64,
    srp_area: f64,
    drag_coefficient: f64,
    drag_area: f64,
    mass: f64,
    mean_motion_dot: f64,
    mean_motion_dot_dot: f64,
}

impl Default for ForceProperties {
    fn default() -> Self {
        Self {
            srp_coefficient: configs::DEFAULT_SRP_TERM,
            srp_area: 1.0,
            drag_coefficient: configs::DEFAULT_DRAG_TERM,
            drag_area: 1.0,
            mass: 1.0,
            mean_motion_dot: 0.0,
            mean_motion_dot_dot: 0.0,
        }
    }
}

impl ForceProperties {
    pub fn from_xa_tle(xa_tle: &[f64; tle_interface::XA_TLE_SIZE]) -> Self {
        let mass = 1.0;
        let srp_area = 1.0;
        let drag_area = 1.0;
        let keplerian_type = KeplerianType::try_from(xa_tle[tle_interface::XA_TLE_EPHTYPE]).unwrap();
        let mean_motion_dot = xa_tle[tle_interface::XA_TLE_NDOT];
        let mean_motion_dot_dot = xa_tle[tle_interface::XA_TLE_NDOTDOT];
        let srp_coefficient = match keplerian_type {
            KeplerianType::Osculating => xa_tle[tle_interface::XA_TLE_SP_AGOM],
            KeplerianType::MeanBrouwerXP => xa_tle[tle_interface::XA_TLE_AGOMGP],
            _ => 0.0,
        };
        let drag_coefficient = match keplerian_type {
            KeplerianType::MeanBrouwerXP => xa_tle[tle_interface::XA_TLE_BTERM],
            KeplerianType::Osculating => xa_tle[tle_interface::XA_TLE_SP_BTERM],
            _ => xa_tle[tle_interface::XA_TLE_BSTAR] * B_STAR_TO_B_TERM,
        };

        Self {
            srp_coefficient,
            srp_area,
            drag_coefficient,
            drag_area,
            mass,
            mean_motion_dot,
            mean_motion_dot_dot,
        }
    }
}

#[pymethods]
impl ForceProperties {
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
            srp_coefficient,
            srp_area,
            drag_coefficient,
            drag_area,
            mass,
            mean_motion_dot,
            mean_motion_dot_dot,
        }
    }

    #[getter]
    pub fn get_srp_term(&self) -> f64 {
        self.srp_coefficient * (self.srp_area / self.mass)
    }

    #[getter]
    pub fn get_drag_term(&self) -> f64 {
        self.drag_coefficient * (self.drag_area / self.mass)
    }

    #[getter]
    pub fn get_b_star(&self) -> f64 {
        self.get_drag_term() / B_STAR_TO_B_TERM
    }

    #[getter]
    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    #[getter]
    pub fn get_mean_motion_dot(&self) -> f64 {
        self.mean_motion_dot
    }

    #[getter]
    pub fn get_mean_motion_dot_dot(&self) -> f64 {
        self.mean_motion_dot_dot
    }

    #[getter]
    pub fn get_srp_coefficient(&self) -> f64 {
        self.srp_coefficient
    }

    #[getter]
    pub fn get_drag_coefficient(&self) -> f64 {
        self.drag_coefficient
    }

    #[getter]
    pub fn get_drag_area(&self) -> f64 {
        self.drag_area
    }

    #[setter]
    pub fn set_srp_coefficient(&mut self, srp_coefficient: f64) {
        self.srp_coefficient = srp_coefficient;
    }

    #[setter]
    pub fn set_srp_area(&mut self, srp_area: f64) {
        self.srp_area = srp_area;
    }

    #[setter]
    pub fn set_drag_coefficient(&mut self, drag_coefficient: f64) {
        self.drag_coefficient = drag_coefficient;
    }

    #[setter]
    pub fn set_drag_area(&mut self, drag_area: f64) {
        self.drag_area = drag_area;
    }

    #[setter]
    pub fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }

    #[setter]
    pub fn set_mean_motion_dot(&mut self, mean_motion_dot: f64) {
        self.mean_motion_dot = mean_motion_dot;
    }

    #[setter]
    pub fn set_mean_motion_dot_dot(&mut self, mean_motion_dot_dot: f64) {
        self.mean_motion_dot_dot = mean_motion_dot_dot;
    }
}

#[pyfunction]
pub fn b_star_to_drag_coefficient(b_star: f64) -> f64 {
    b_star * B_STAR_TO_B_TERM
}

#[pyfunction]
pub fn drag_coefficient_to_b_star(drag_coefficient: f64) -> f64 {
    drag_coefficient / B_STAR_TO_B_TERM
}
