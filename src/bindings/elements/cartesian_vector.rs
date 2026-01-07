use super::PySphericalVector;
use crate::elements::{CartesianVector, SphericalVector};
use pyo3::prelude::*;

#[pyclass(name = "CartesianVector")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyCartesianVector {
    inner: CartesianVector,
}

#[pymethods]
impl PyCartesianVector {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            inner: CartesianVector::new(x, y, z),
        }
    }

    fn __add__(&self, other: &Self) -> Self {
        (self.inner + other.inner).into()
    }

    fn __sub__(&self, other: &Self) -> Self {
        (self.inner - other.inner).into()
    }

    #[getter]
    fn x(&self) -> f64 {
        self.inner.get_x()
    }

    #[getter]
    fn y(&self) -> f64 {
        self.inner.get_y()
    }

    #[getter]
    fn z(&self) -> f64 {
        self.inner.get_z()
    }

    #[getter]
    pub fn get_magnitude(&self) -> f64 {
        self.inner.get_magnitude()
    }

    pub fn distance(&self, other: &Self) -> f64 {
        (self.inner - other.inner).get_magnitude()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.inner.dot(&other.inner)
    }

    pub fn angle(&self, other: &Self) -> f64 {
        self.inner.angle(&other.inner)
    }

    pub fn to_spherical(&self) -> PySphericalVector {
        let sph = SphericalVector::from(self.inner);
        PySphericalVector::from(sph)
    }
}

impl From<CartesianVector> for PyCartesianVector {
    fn from(value: CartesianVector) -> Self {
        Self::new(value.get_x(), value.get_y(), value.get_z())
    }
}

impl From<PyCartesianVector> for CartesianVector {
    fn from(value: PyCartesianVector) -> Self {
        CartesianVector::new(value.x(), value.y(), value.z())
    }
}

impl From<PyCartesianVector> for [f64; 3] {
    fn from(cartesian_vector: PyCartesianVector) -> Self {
        [cartesian_vector.x(), cartesian_vector.y(), cartesian_vector.z()]
    }
}
