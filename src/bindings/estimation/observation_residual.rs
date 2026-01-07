use crate::estimation::ObservationResidual;
use pyo3::prelude::*;

#[pyclass(name = "ObservationResidual")]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PyObservationResidual {
    inner: ObservationResidual,
}

impl From<ObservationResidual> for PyObservationResidual {
    fn from(inner: ObservationResidual) -> Self {
        Self { inner }
    }
}

impl From<PyObservationResidual> for ObservationResidual {
    fn from(value: PyObservationResidual) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyObservationResidual {
    #[getter]
    pub fn get_range(&self) -> f64 {
        self.inner.get_range()
    }

    #[getter]
    pub fn get_time(&self) -> f64 {
        self.inner.get_time()
    }

    #[getter]
    pub fn get_radial(&self) -> f64 {
        self.inner.get_radial()
    }

    #[getter]
    pub fn get_in_track(&self) -> f64 {
        self.inner.get_in_track()
    }

    #[getter]
    pub fn get_cross_track(&self) -> f64 {
        self.inner.get_cross_track()
    }

    #[getter]
    pub fn get_velocity(&self) -> f64 {
        self.inner.get_velocity()
    }

    #[getter]
    pub fn get_radial_velocity(&self) -> f64 {
        self.inner.get_radial_velocity()
    }

    #[getter]
    pub fn get_in_track_velocity(&self) -> f64 {
        self.inner.get_in_track_velocity()
    }

    #[getter]
    pub fn get_cross_track_velocity(&self) -> f64 {
        self.inner.get_cross_track_velocity()
    }

    #[getter]
    pub fn get_beta(&self) -> f64 {
        self.inner.get_beta()
    }

    #[getter]
    pub fn get_height(&self) -> f64 {
        self.inner.get_height()
    }

    #[getter]
    pub fn get_angular_momentum(&self) -> f64 {
        self.inner.get_angular_momentum()
    }
}
