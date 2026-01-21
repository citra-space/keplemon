use crate::elements::BoreToBodyAngles;
use pyo3::prelude::*;

#[pyclass(name = "BoreToBodyAngles")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyBoreToBodyAngles {
    inner: BoreToBodyAngles,
}

impl From<BoreToBodyAngles> for PyBoreToBodyAngles {
    fn from(inner: BoreToBodyAngles) -> Self {
        Self { inner }
    }
}

impl From<PyBoreToBodyAngles> for BoreToBodyAngles {
    fn from(value: PyBoreToBodyAngles) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyBoreToBodyAngles {
    #[new]
    pub fn new(earth_angle: f64, sun_angle: f64, moon_angle: f64) -> Self {
        Self {
            inner: BoreToBodyAngles::new(earth_angle, sun_angle, moon_angle),
        }
    }

    #[getter]
    pub fn get_earth_angle(&self) -> f64 {
        self.inner.get_earth_angle()
    }

    #[getter]
    pub fn get_sun_angle(&self) -> f64 {
        self.inner.get_sun_angle()
    }

    #[getter]
    pub fn get_moon_angle(&self) -> f64 {
        self.inner.get_moon_angle()
    }
}
