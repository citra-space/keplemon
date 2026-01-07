use crate::bindings::enums::PyCovarianceType;
use crate::estimation::Covariance;
use nalgebra::DMatrix;
use pyo3::prelude::*;

#[pyclass(name = "Covariance")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyCovariance {
    inner: Covariance,
}

impl From<Covariance> for PyCovariance {
    fn from(inner: Covariance) -> Self {
        Self { inner }
    }
}

impl From<PyCovariance> for Covariance {
    fn from(value: PyCovariance) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyCovariance {
    pub fn get_element(&self, i: usize, j: usize) -> f64 {
        self.inner.get_element(i, j)
    }

    pub fn set_element(&mut self, i: usize, j: usize, value: f64) {
        self.inner.set_element(i, j, value);
    }

    #[getter]
    pub fn get_covariance_type(&self) -> PyCovarianceType {
        PyCovarianceType::from(self.inner.get_covariance_type())
    }

    #[getter]
    pub fn get_sigmas(&self) -> Vec<f64> {
        self.inner.get_sigmas()
    }
}

impl From<PyCovariance> for [[f64; 6]; 6] {
    fn from(cov: PyCovariance) -> Self {
        let inner: Covariance = cov.into();
        inner.into()
    }
}

impl From<([[f64; 6]; 6], PyCovarianceType)> for PyCovariance {
    fn from(input: ([[f64; 6]; 6], PyCovarianceType)) -> Self {
        let (elements, covariance_type) = input;
        Covariance::from((elements, covariance_type.into())).into()
    }
}

impl From<(DMatrix<f64>, PyCovarianceType)> for PyCovariance {
    fn from(input: (DMatrix<f64>, PyCovarianceType)) -> Self {
        let (input_cov, covariance_type) = input;
        Covariance::from((input_cov, covariance_type.into())).into()
    }
}
