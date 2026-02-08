#![allow(non_upper_case_globals)]

use crate::enums::UCTObservability;
use pyo3::prelude::*;

#[pyclass(name = "UCTObservability")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyUCTObservability {
    inner: UCTObservability,
}

#[pymethods]
impl PyUCTObservability {
    #[classattr]
    pub const Confirmed: Self = Self {
        inner: UCTObservability::Confirmed,
    };
    #[classattr]
    pub const Possible: Self = Self {
        inner: UCTObservability::Possible,
    };
    #[classattr]
    pub const Unavailable: Self = Self {
        inner: UCTObservability::Unavailable,
    };

    #[getter]
    fn value(&self) -> &str {
        match self.inner {
            UCTObservability::Confirmed => "Confirmed",
            UCTObservability::Possible => "Possible",
            UCTObservability::Unavailable => "Unavailable",
        }
    }

    fn __repr__(&self) -> &str {
        match self.inner {
            UCTObservability::Confirmed => "UCTObservability.Confirmed",
            UCTObservability::Possible => "UCTObservability.Possible",
            UCTObservability::Unavailable => "UCTObservability.Unavailable",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<UCTObservability> for PyUCTObservability {
    fn from(inner: UCTObservability) -> Self {
        Self { inner }
    }
}

impl From<PyUCTObservability> for UCTObservability {
    fn from(value: PyUCTObservability) -> Self {
        value.inner
    }
}
