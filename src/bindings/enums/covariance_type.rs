#![allow(non_upper_case_globals)]

use crate::enums::CovarianceType;
use pyo3::prelude::*;

#[pyclass(name = "CovarianceType")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyCovarianceType {
    inner: CovarianceType,
}

#[pymethods]
impl PyCovarianceType {
    #[classattr]
    pub const Inertial: Self = Self {
        inner: CovarianceType::Inertial,
    };
    #[classattr]
    pub const Relative: Self = Self {
        inner: CovarianceType::Relative,
    };
    #[classattr]
    pub const Equinoctial: Self = Self {
        inner: CovarianceType::Equinoctial,
    };

    #[getter]
    fn get_value(&self) -> &str {
        match self.inner {
            CovarianceType::Inertial => "Inertial",
            CovarianceType::Relative => "Relative",
            CovarianceType::Equinoctial => "Equinoctial",
        }
    }

    fn __repr__(&self) -> &str {
        match self.inner {
            CovarianceType::Inertial => "CovarianceType.Inertial",
            CovarianceType::Relative => "CovarianceType.Relative",
            CovarianceType::Equinoctial => "CovarianceType.Equinoctial",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<CovarianceType> for PyCovarianceType {
    fn from(inner: CovarianceType) -> Self {
        Self { inner }
    }
}

impl From<PyCovarianceType> for CovarianceType {
    fn from(value: PyCovarianceType) -> Self {
        value.inner
    }
}
