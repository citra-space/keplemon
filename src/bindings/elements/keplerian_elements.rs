use std::ops::{Index, IndexMut};

use crate::bindings::enums::PyKeplerianType;
use crate::elements::KeplerianElements;
use crate::enums::KeplerianType;
use pyo3::prelude::*;

use super::PyEquinoctialElements;
use crate::elements::EquinoctialElements;

#[pyclass(name = "KeplerianElements")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyKeplerianElements {
    inner: KeplerianElements,
}

impl From<KeplerianElements> for PyKeplerianElements {
    fn from(inner: KeplerianElements) -> Self {
        Self { inner }
    }
}

impl From<PyKeplerianElements> for KeplerianElements {
    fn from(value: PyKeplerianElements) -> Self {
        value.inner
    }
}

impl From<&PyEquinoctialElements> for PyKeplerianElements {
    fn from(eqn: &PyEquinoctialElements) -> Self {
        KeplerianElements::from(EquinoctialElements::from(*eqn)).into()
    }
}

impl IndexMut<usize> for PyKeplerianElements {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.inner.semi_major_axis,
            1 => &mut self.inner.eccentricity,
            2 => &mut self.inner.inclination,
            3 => &mut self.inner.raan,
            4 => &mut self.inner.argument_of_perigee,
            5 => &mut self.inner.mean_anomaly,
            _ => panic!("Index out of bounds for Keplerian elements"),
        }
    }
}

impl Index<usize> for PyKeplerianElements {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.inner.semi_major_axis,
            1 => &self.inner.eccentricity,
            2 => &self.inner.inclination,
            3 => &self.inner.raan,
            4 => &self.inner.argument_of_perigee,
            5 => &self.inner.mean_anomaly,
            _ => panic!("Index out of bounds for Keplerian elements"),
        }
    }
}

impl PyKeplerianElements {
    pub fn get_xa_kep(&self) -> [f64; 6] {
        self.inner.get_xa_kep()
    }

    pub fn get_xa_cls(&self, element_type: PyKeplerianType) -> [f64; 6] {
        let element_type: KeplerianType = element_type.into();
        self.inner.get_xa_cls(element_type)
    }
}

#[pymethods]
impl PyKeplerianElements {
    #[new]
    pub fn new(
        semi_major_axis: f64,
        eccentricity: f64,
        inclination: f64,
        raan: f64,
        argument_of_perigee: f64,
        mean_anomaly: f64,
    ) -> Self {
        Self {
            inner: KeplerianElements::new(
                semi_major_axis,
                eccentricity,
                inclination,
                raan,
                argument_of_perigee,
                mean_anomaly,
            ),
        }
    }

    #[getter]
    pub fn get_semi_major_axis(&self) -> f64 {
        self.inner.semi_major_axis
    }

    #[getter]
    pub fn get_eccentricity(&self) -> f64 {
        self.inner.eccentricity
    }

    #[getter]
    pub fn get_inclination(&self) -> f64 {
        self.inner.inclination
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
    pub fn get_raan(&self) -> f64 {
        self.inner.raan
    }

    #[getter]
    pub fn get_argument_of_perigee(&self) -> f64 {
        self.inner.argument_of_perigee
    }

    #[getter]
    pub fn get_mean_anomaly(&self) -> f64 {
        self.inner.mean_anomaly
    }

    pub fn get_mean_motion(&self, element_type: PyKeplerianType) -> f64 {
        let element_type: KeplerianType = element_type.into();
        self.inner.get_mean_motion(element_type)
    }

    #[setter]
    pub fn set_semi_major_axis(&mut self, semi_major_axis: f64) {
        self.inner.semi_major_axis = semi_major_axis;
    }

    #[setter]
    pub fn set_eccentricity(&mut self, eccentricity: f64) {
        self.inner.eccentricity = eccentricity;
    }

    #[setter]
    pub fn set_inclination(&mut self, inclination: f64) {
        self.inner.inclination = inclination;
    }

    #[setter]
    pub fn set_raan(&mut self, raan: f64) {
        self.inner.raan = raan;
    }

    #[setter]
    pub fn set_argument_of_perigee(&mut self, argument_of_perigee: f64) {
        self.inner.argument_of_perigee = argument_of_perigee;
    }

    #[setter]
    pub fn set_mean_anomaly(&mut self, mean_anomaly: f64) {
        self.inner.mean_anomaly = mean_anomaly;
    }

    pub fn to_equinoctial(&self) -> PyEquinoctialElements {
        PyEquinoctialElements::from(EquinoctialElements::from(self.inner))
    }

    pub fn to_mean(&self) -> Self {
        self.inner.to_mean().into()
    }
}
