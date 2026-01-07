use super::{CartesianVector, SphericalVector};
use crate::enums::MeanEquinox;
use crate::time::Epoch;
use saal::astro;

#[derive(Debug, Clone, PartialEq)]
pub struct TopocentricElements {
    pub range: Option<f64>,
    pub range_rate: Option<f64>,
    pub right_ascension: f64,
    pub declination: f64,
    pub right_ascension_rate: Option<f64>,
    pub declination_rate: Option<f64>,
    observed_direction: CartesianVector,
}

impl Copy for TopocentricElements {}

impl TopocentricElements {
    pub fn new(right_ascension: f64, declination: f64) -> Self {
        let observed_direction: CartesianVector = SphericalVector::new(1.0, right_ascension, declination).into();
        Self {
            range: None,
            range_rate: None,
            right_ascension,
            declination,
            right_ascension_rate: None,
            declination_rate: None,
            observed_direction,
        }
    }

    pub fn from_j2000(epoch: Epoch, right_ascension: f64, declination: f64) -> Self {
        let (ra, dec) = astro::topo_meme_to_teme(
            MeanEquinox::J2000.get_value(),
            epoch.days_since_1950,
            right_ascension,
            declination,
        );
        let observed_direction: CartesianVector = SphericalVector::new(1.0, ra, dec).into();
        Self {
            range: None,
            range_rate: None,
            right_ascension: ra,
            declination: dec,
            right_ascension_rate: None,
            declination_rate: None,
            observed_direction,
        }
    }

    pub fn get_observed_direction(&self) -> &CartesianVector {
        &self.observed_direction
    }
}
