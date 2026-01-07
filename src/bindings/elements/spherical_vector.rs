use super::PyCartesianVector;
use crate::elements::{CartesianVector, SphericalVector};
use pyo3::prelude::*;

#[pyclass(name = "SphericalVector")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PySphericalVector {
    inner: SphericalVector,
}

impl From<SphericalVector> for PySphericalVector {
    fn from(inner: SphericalVector) -> Self {
        Self { inner }
    }
}

impl From<PySphericalVector> for SphericalVector {
    fn from(value: PySphericalVector) -> Self {
        value.inner
    }
}

#[pymethods]
impl PySphericalVector {
    #[new]
    pub fn new(range: f64, right_ascension: f64, declination: f64) -> Self {
        SphericalVector::new(range, right_ascension, declination).into()
    }

    #[getter]
    pub fn get_range(&self) -> f64 {
        self.inner.range
    }

    #[getter]
    pub fn get_right_ascension(&self) -> f64 {
        self.inner.right_ascension
    }

    #[getter]
    pub fn get_declination(&self) -> f64 {
        self.inner.declination
    }

    #[setter]
    pub fn set_range(&mut self, range: f64) {
        self.inner.range = range;
    }

    #[setter]
    pub fn set_right_ascension(&mut self, right_ascension: f64) {
        self.inner.right_ascension = right_ascension;
    }

    #[setter]
    pub fn set_declination(&mut self, declination: f64) {
        self.inner.declination = declination;
    }

    pub fn to_cartesian(&self) -> PyCartesianVector {
        PyCartesianVector::from(CartesianVector::from(self.inner))
    }
}
