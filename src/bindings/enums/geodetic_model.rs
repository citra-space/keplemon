use crate::enums::GeodeticModel;
use pyo3::prelude::*;

#[pyclass(name = "GeodeticModel")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyGeodeticModel {
    inner: GeodeticModel,
}

#[pymethods]
impl PyGeodeticModel {
    #[classattr]
    pub const WGS72: Self = Self {
        inner: GeodeticModel::WGS72,
    };
    #[classattr]
    pub const WGS84: Self = Self {
        inner: GeodeticModel::WGS84,
    };
    #[classattr]
    pub const EGM96: Self = Self {
        inner: GeodeticModel::EGM96,
    };

    #[getter]
    fn value(&self) -> i32 {
        self.inner as i32
    }

    fn __repr__(&self) -> &'static str {
        match self.inner {
            GeodeticModel::WGS72 => "GeodeticModel.WGS72",
            GeodeticModel::WGS84 => "GeodeticModel.WGS84",
            GeodeticModel::EGM96 => "GeodeticModel.EGM96",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<GeodeticModel> for PyGeodeticModel {
    fn from(inner: GeodeticModel) -> Self {
        Self { inner }
    }
}

impl From<PyGeodeticModel> for GeodeticModel {
    fn from(value: PyGeodeticModel) -> Self {
        value.inner
    }
}
