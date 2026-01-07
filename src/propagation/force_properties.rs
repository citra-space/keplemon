use crate::configs;
use crate::elements::B_STAR_TO_B_TERM;
use crate::enums::KeplerianType;
use saal::tle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForceProperties {
    pub srp_coefficient: f64,
    pub srp_area: f64,
    pub drag_coefficient: f64,
    pub drag_area: f64,
    pub mass: f64,
    pub mean_motion_dot: f64,
    pub mean_motion_dot_dot: f64,
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

impl From<&[f64; tle::XA_TLE_SIZE]> for ForceProperties {
    fn from(xa_tle: &[f64; tle::XA_TLE_SIZE]) -> Self {
        let mass = 1.0;
        let srp_area = 1.0;
        let drag_area = 1.0;
        let keplerian_type = KeplerianType::try_from(xa_tle[tle::XA_TLE_EPHTYPE]).unwrap();
        let mean_motion_dot = xa_tle[tle::XA_TLE_NDOT];
        let mean_motion_dot_dot = xa_tle[tle::XA_TLE_NDOTDOT];
        let srp_coefficient = match keplerian_type {
            KeplerianType::Osculating => xa_tle[tle::XA_TLE_SP_AGOM],
            KeplerianType::MeanBrouwerXP => xa_tle[tle::XA_TLE_AGOMGP],
            _ => 0.0,
        };
        let drag_coefficient = match keplerian_type {
            KeplerianType::MeanBrouwerXP => xa_tle[tle::XA_TLE_BTERM],
            KeplerianType::Osculating => xa_tle[tle::XA_TLE_SP_BTERM],
            _ => xa_tle[tle::XA_TLE_BSTAR] * B_STAR_TO_B_TERM,
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

impl ForceProperties {
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

    pub fn get_srp_term(&self) -> f64 {
        self.srp_coefficient * (self.srp_area / self.mass)
    }

    pub fn get_drag_term(&self) -> f64 {
        self.drag_coefficient * (self.drag_area / self.mass)
    }

    pub fn get_b_star(&self) -> f64 {
        self.get_drag_term() / B_STAR_TO_B_TERM
    }
}

pub fn b_star_to_drag_coefficient(b_star: f64) -> f64 {
    b_star * B_STAR_TO_B_TERM
}

pub fn drag_coefficient_to_b_star(drag_coefficient: f64) -> f64 {
    drag_coefficient / B_STAR_TO_B_TERM
}
