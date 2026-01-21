use super::KeplerianElements;
use saal::astro;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquinoctialElements {
    pub a_f: f64,
    pub a_g: f64,
    pub chi: f64,
    pub psi: f64,
    pub mean_longitude: f64,
    pub mean_motion: f64,
}

impl EquinoctialElements {
    pub fn new(a_f: f64, a_g: f64, chi: f64, psi: f64, mean_longitude: f64, mean_motion: f64) -> Self {
        Self {
            a_f,
            a_g,
            chi,
            psi,
            mean_longitude,
            mean_motion,
        }
    }
}

impl From<&EquinoctialElements> for [f64; astro::XA_EQNX_SIZE] {
    fn from(eqn: &EquinoctialElements) -> Self {
        let mut xa_eqnx = [0.0; astro::XA_EQNX_SIZE];
        xa_eqnx[astro::XA_EQNX_AF] = eqn.a_f;
        xa_eqnx[astro::XA_EQNX_AG] = eqn.a_g;
        xa_eqnx[astro::XA_EQNX_CHI] = eqn.chi;
        xa_eqnx[astro::XA_EQNX_PSI] = eqn.psi;
        xa_eqnx[astro::XA_EQNX_L] = eqn.mean_longitude;
        xa_eqnx[astro::XA_EQNX_N] = eqn.mean_motion;
        xa_eqnx
    }
}

impl From<EquinoctialElements> for [f64; astro::XA_EQNX_SIZE] {
    fn from(eqn: EquinoctialElements) -> Self {
        Self::from(&eqn)
    }
}

impl From<&KeplerianElements> for EquinoctialElements {
    fn from(kep: &KeplerianElements) -> Self {
        let xa_eqnx = astro::keplerian_to_equinoctial(&kep.into());
        Self::from(xa_eqnx)
    }
}

impl From<KeplerianElements> for EquinoctialElements {
    fn from(kep: KeplerianElements) -> Self {
        Self::from(&kep)
    }
}

impl From<[f64; astro::XA_EQNX_SIZE]> for EquinoctialElements {
    fn from(xa_eqnx: [f64; astro::XA_EQNX_SIZE]) -> Self {
        Self {
            a_f: xa_eqnx[astro::XA_EQNX_AF],
            a_g: xa_eqnx[astro::XA_EQNX_AG],
            chi: xa_eqnx[astro::XA_EQNX_CHI],
            psi: xa_eqnx[astro::XA_EQNX_PSI],
            mean_longitude: xa_eqnx[astro::XA_EQNX_L],
            mean_motion: xa_eqnx[astro::XA_EQNX_N],
        }
    }
}

impl Index<usize> for EquinoctialElements {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            astro::XA_EQNX_AF => &self.a_f,
            astro::XA_EQNX_AG => &self.a_g,
            astro::XA_EQNX_CHI => &self.chi,
            astro::XA_EQNX_PSI => &self.psi,
            astro::XA_EQNX_L => &self.mean_longitude,
            astro::XA_EQNX_N => &self.mean_motion,
            _ => panic!("Index out of bounds for equinoctial elements"),
        }
    }
}

impl IndexMut<usize> for EquinoctialElements {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            astro::XA_EQNX_AF => &mut self.a_f,
            astro::XA_EQNX_AG => &mut self.a_g,
            astro::XA_EQNX_CHI => &mut self.chi,
            astro::XA_EQNX_PSI => &mut self.psi,
            astro::XA_EQNX_L => &mut self.mean_longitude,
            astro::XA_EQNX_N => &mut self.mean_motion,
            _ => panic!("Index out of bounds for equinoctial elements"),
        }
    }
}
