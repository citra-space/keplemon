use super::PyKeplerianElements;
use crate::elements::{EquinoctialElements, KeplerianElements};
use pyo3::prelude::*;
use std::ops::{Index, IndexMut};

#[pyclass(name = "EquinoctialElements")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyEquinoctialElements {
    inner: EquinoctialElements,
}

impl From<EquinoctialElements> for PyEquinoctialElements {
    fn from(inner: EquinoctialElements) -> Self {
        Self { inner }
    }
}

impl From<PyEquinoctialElements> for EquinoctialElements {
    fn from(value: PyEquinoctialElements) -> Self {
        value.inner
    }
}

impl From<&PyKeplerianElements> for PyEquinoctialElements {
    fn from(kep: &PyKeplerianElements) -> Self {
        EquinoctialElements::from(KeplerianElements::from(*kep)).into()
    }
}

impl Index<usize> for PyEquinoctialElements {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.inner.a_f,
            1 => &self.inner.a_g,
            2 => &self.inner.chi,
            3 => &self.inner.psi,
            4 => &self.inner.mean_longitude,
            5 => &self.inner.mean_motion,
            _ => panic!("Index out of bounds for equinoctial elements"),
        }
    }
}

impl IndexMut<usize> for PyEquinoctialElements {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.inner.a_f,
            1 => &mut self.inner.a_g,
            2 => &mut self.inner.chi,
            3 => &mut self.inner.psi,
            4 => &mut self.inner.mean_longitude,
            5 => &mut self.inner.mean_motion,
            _ => panic!("Index out of bounds for equinoctial elements"),
        }
    }
}

#[pymethods]
impl PyEquinoctialElements {
    #[new]
    pub fn new(a_f: f64, a_g: f64, chi: f64, psi: f64, mean_longitude: f64, mean_motion: f64) -> Self {
        Self {
            inner: EquinoctialElements::new(a_f, a_g, chi, psi, mean_longitude, mean_motion),
        }
    }

    pub fn to_keplerian(&self) -> PyKeplerianElements {
        PyKeplerianElements::from(KeplerianElements::from(self.inner))
    }

    #[getter]
    pub fn get_mean_motion(&self) -> f64 {
        self.inner.mean_motion
    }
}
