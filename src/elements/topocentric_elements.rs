use super::{CartesianVector, SphericalVector};
use crate::enums::EquinoxType;
use crate::saal::astro_func_interface;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct TopocentricElements {
    range: Option<f64>,
    range_rate: Option<f64>,
    right_ascension: f64,
    declination: f64,
    right_ascension_rate: Option<f64>,
    declination_rate: Option<f64>,
    observed_direction: CartesianVector,
}

impl Copy for TopocentricElements {}

#[pymethods]
impl TopocentricElements {
    #[new]
    pub fn new(right_ascension: f64, declination: f64) -> Self {
        let observed_direction = SphericalVector::new(1.0, right_ascension, declination).to_cartesian();
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

    #[staticmethod]
    fn from_j2000(epoch: Epoch, right_ascension: f64, declination: f64) -> Self {
        let (ra, dec) = astro_func_interface::topo_equinox_to_date(
            EquinoxType::J2000.get_value(),
            epoch.days_since_1950,
            right_ascension,
            declination,
        );
        let observed_direction = SphericalVector::new(1.0, ra, dec).to_cartesian();
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

    #[getter]
    pub fn get_right_ascension(&self) -> f64 {
        self.right_ascension
    }

    #[getter]
    pub fn get_declination(&self) -> f64 {
        self.declination
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.range
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.range_rate
    }

    #[getter]
    pub fn get_right_ascension_rate(&self) -> Option<f64> {
        self.right_ascension_rate
    }

    #[getter]
    pub fn get_declination_rate(&self) -> Option<f64> {
        self.declination_rate
    }

    #[setter]
    pub fn set_range(&mut self, range: Option<f64>) {
        match range {
            Some(r) => self.range = Some(r),
            None => self.range = None,
        }
    }

    #[setter]
    pub fn set_range_rate(&mut self, range_rate: Option<f64>) {
        match range_rate {
            Some(rr) => self.range_rate = Some(rr),
            None => self.range_rate = None,
        }
    }

    #[setter]
    pub fn set_right_ascension_rate(&mut self, right_ascension_rate: Option<f64>) {
        match right_ascension_rate {
            Some(ra) => self.right_ascension_rate = Some(ra),
            None => self.right_ascension_rate = None,
        }
    }

    #[setter]
    pub fn set_declination_rate(&mut self, declination_rate: Option<f64>) {
        match declination_rate {
            Some(dec) => self.declination_rate = Some(dec),
            None => self.declination_rate = None,
        }
    }

    #[setter]
    pub fn set_right_ascension(&mut self, right_ascension: f64) {
        self.right_ascension = right_ascension;
        self.observed_direction = SphericalVector::new(1.0, right_ascension, self.declination).to_cartesian();
    }

    #[setter]
    pub fn set_declination(&mut self, declination: f64) {
        self.declination = declination;
        self.observed_direction = SphericalVector::new(1.0, self.right_ascension, declination).to_cartesian();
    }

    #[getter]
    pub fn get_observed_direction(&self) -> CartesianVector {
        self.observed_direction
    }
}
