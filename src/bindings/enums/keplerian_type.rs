#![allow(non_upper_case_globals)]

use crate::enums::KeplerianType;
use pyo3::prelude::*;

#[pyclass(name = "KeplerianType")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PyKeplerianType {
    inner: KeplerianType,
}

#[pymethods]
impl PyKeplerianType {
    #[classattr]
    pub const MeanKozaiGP: Self = Self {
        inner: KeplerianType::MeanKozaiGP,
    };
    #[classattr]
    pub const MeanBrouwerGP: Self = Self {
        inner: KeplerianType::MeanBrouwerGP,
    };
    #[classattr]
    pub const MeanBrouwerXP: Self = Self {
        inner: KeplerianType::MeanBrouwerXP,
    };
    #[classattr]
    pub const Osculating: Self = Self {
        inner: KeplerianType::Osculating,
    };

    pub fn __repr__(&self) -> &'static str {
        match self.inner {
            KeplerianType::MeanKozaiGP => "KeplerianType.MeanKozaiGP",
            KeplerianType::MeanBrouwerGP => "KeplerianType.MeanBrouwerGP",
            KeplerianType::MeanBrouwerXP => "KeplerianType.MeanBrouwerXP",
            KeplerianType::Osculating => "KeplerianType.Osculating",
        }
    }

    #[getter]
    pub fn value(&self) -> i32 {
        match self.inner {
            KeplerianType::MeanKozaiGP => KeplerianType::MeanKozaiGP as i32,
            KeplerianType::MeanBrouwerGP => KeplerianType::MeanBrouwerGP as i32,
            KeplerianType::MeanBrouwerXP => KeplerianType::MeanBrouwerXP as i32,
            KeplerianType::Osculating => KeplerianType::Osculating as i32,
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<KeplerianType> for PyKeplerianType {
    fn from(inner: KeplerianType) -> Self {
        Self { inner }
    }
}

impl From<PyKeplerianType> for KeplerianType {
    fn from(value: PyKeplerianType) -> Self {
        value.inner
    }
}
