use std::ops::{Index, IndexMut};

use super::EquinoctialElements;
use crate::enums::KeplerianType;
use saal::astro;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerianElements {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub raan: f64,
    pub argument_of_perigee: f64,
    pub mean_anomaly: f64,
}

impl From<&EquinoctialElements> for KeplerianElements {
    fn from(eqn: &EquinoctialElements) -> Self {
        let xa_kep = astro::equinoctial_to_keplerian(&eqn.into());
        Self::from(xa_kep)
    }
}

impl From<EquinoctialElements> for KeplerianElements {
    fn from(eqn: EquinoctialElements) -> Self {
        Self::from(&eqn)
    }
}

impl From<&KeplerianElements> for [f64; astro::XA_KEP_SIZE] {
    fn from(kep: &KeplerianElements) -> Self {
        let mut xa_kep = [0.0; astro::XA_KEP_SIZE];
        xa_kep[astro::XA_KEP_A] = kep.semi_major_axis;
        xa_kep[astro::XA_KEP_MA] = kep.mean_anomaly;
        xa_kep[astro::XA_KEP_E] = kep.eccentricity;
        xa_kep[astro::XA_KEP_INCLI] = kep.inclination;
        xa_kep[astro::XA_KEP_NODE] = kep.raan;
        xa_kep[astro::XA_KEP_OMEGA] = kep.argument_of_perigee;
        xa_kep
    }
}

impl From<[f64; astro::XA_KEP_SIZE]> for KeplerianElements {
    fn from(xa_kep: [f64; astro::XA_KEP_SIZE]) -> Self {
        Self {
            semi_major_axis: xa_kep[astro::XA_KEP_A],
            eccentricity: xa_kep[astro::XA_KEP_E],
            inclination: xa_kep[astro::XA_KEP_INCLI],
            raan: xa_kep[astro::XA_KEP_NODE],
            argument_of_perigee: xa_kep[astro::XA_KEP_OMEGA],
            mean_anomaly: xa_kep[astro::XA_KEP_MA],
        }
    }
}

impl From<&[f64; astro::XA_KEP_SIZE]> for KeplerianElements {
    fn from(xa_kep: &[f64; astro::XA_KEP_SIZE]) -> Self {
        Self {
            semi_major_axis: xa_kep[astro::XA_KEP_A],
            eccentricity: xa_kep[astro::XA_KEP_E],
            inclination: xa_kep[astro::XA_KEP_INCLI],
            raan: xa_kep[astro::XA_KEP_NODE],
            argument_of_perigee: xa_kep[astro::XA_KEP_OMEGA],
            mean_anomaly: xa_kep[astro::XA_KEP_MA],
        }
    }
}

impl IndexMut<usize> for KeplerianElements {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            astro::XA_KEP_A => &mut self.semi_major_axis,
            astro::XA_KEP_E => &mut self.eccentricity,
            astro::XA_KEP_INCLI => &mut self.inclination,
            astro::XA_KEP_NODE => &mut self.raan,
            astro::XA_KEP_OMEGA => &mut self.argument_of_perigee,
            astro::XA_KEP_MA => &mut self.mean_anomaly,
            _ => panic!("Index out of bounds for Keplerian elements"),
        }
    }
}

impl Index<usize> for KeplerianElements {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            astro::XA_KEP_A => &self.semi_major_axis,
            astro::XA_KEP_E => &self.eccentricity,
            astro::XA_KEP_INCLI => &self.inclination,
            astro::XA_KEP_NODE => &self.raan,
            astro::XA_KEP_OMEGA => &self.argument_of_perigee,
            astro::XA_KEP_MA => &self.mean_anomaly,
            _ => panic!("Index out of bounds for Keplerian elements"),
        }
    }
}

impl KeplerianElements {
    pub fn new(
        semi_major_axis: f64,
        eccentricity: f64,
        inclination: f64,
        raan: f64,
        argument_of_perigee: f64,
        mean_anomaly: f64,
    ) -> Self {
        Self {
            semi_major_axis,
            eccentricity,
            inclination,
            raan,
            argument_of_perigee,
            mean_anomaly,
        }
    }

    pub fn get_mean_motion(&self, element_type: KeplerianType) -> f64 {
        let mean_motion = astro::sma_to_mean_motion(self.semi_major_axis);
        match element_type {
            KeplerianType::MeanKozaiGP => astro::brouwer_to_kozai(self.eccentricity, self.inclination, mean_motion),
            _ => mean_motion,
        }
    }

    pub fn to_mean(&self) -> Self {
        astro::osculating_to_mean(&self.get_xa_kep()).into()
    }

    pub fn get_xa_kep(&self) -> [f64; astro::XA_KEP_SIZE] {
        let mut xa_kep = [0.0; astro::XA_KEP_SIZE];
        xa_kep[astro::XA_KEP_A] = self.semi_major_axis;
        xa_kep[astro::XA_KEP_MA] = self.mean_anomaly;
        xa_kep[astro::XA_KEP_E] = self.eccentricity;
        xa_kep[astro::XA_KEP_INCLI] = self.inclination;
        xa_kep[astro::XA_KEP_NODE] = self.raan;
        xa_kep[astro::XA_KEP_OMEGA] = self.argument_of_perigee;
        xa_kep
    }

    pub fn get_xa_cls(&self, element_type: KeplerianType) -> [f64; astro::XA_CLS_SIZE] {
        let mut xa_cls = [0.0; astro::XA_CLS_SIZE];
        xa_cls[astro::XA_CLS_N] = self.get_mean_motion(element_type);
        xa_cls[astro::XA_CLS_MA] = self.mean_anomaly;
        xa_cls[astro::XA_CLS_E] = self.eccentricity;
        xa_cls[astro::XA_CLS_INCLI] = self.inclination;
        xa_cls[astro::XA_CLS_NODE] = self.raan;
        xa_cls[astro::XA_CLS_OMEGA] = self.argument_of_perigee;
        xa_cls
    }

    pub fn get_apoapsis(&self) -> f64 {
        self.semi_major_axis * (1.0 + self.eccentricity)
    }

    pub fn get_periapsis(&self) -> f64 {
        self.semi_major_axis * (1.0 - self.eccentricity)
    }
}
