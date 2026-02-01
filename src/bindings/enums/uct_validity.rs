#![allow(non_upper_case_globals)]

use crate::enums::UCTValidity;
use pyo3::prelude::*;

#[pyclass(name = "UCTValidity")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyUCTValidity {
    inner: UCTValidity,
}

#[pymethods]
impl PyUCTValidity {
    #[classattr]
    pub const LikelyReal: Self = Self {
        inner: UCTValidity::LikelyReal,
    };
    #[classattr]
    pub const PossibleCrossTag: Self = Self {
        inner: UCTValidity::PossibleCrossTag,
    };
    #[classattr]
    pub const PossibleManeuver: Self = Self {
        inner: UCTValidity::PossibleManeuver,
    };
    #[classattr]
    pub const Inconclusive: Self = Self {
        inner: UCTValidity::Inconclusive,
    };

    #[getter]
    fn value(&self) -> &str {
        match self.inner {
            UCTValidity::LikelyReal => "LikelyReal",
            UCTValidity::PossibleCrossTag => "PossibleCrossTag",
            UCTValidity::PossibleManeuver => "PossibleManeuver",
            UCTValidity::Inconclusive => "Inconclusive",
        }
    }

    fn __repr__(&self) -> &str {
        match self.inner {
            UCTValidity::LikelyReal => "UCTValidity.LikelyReal",
            UCTValidity::PossibleCrossTag => "UCTValidity.PossibleCrossTag",
            UCTValidity::PossibleManeuver => "UCTValidity.PossibleManeuver",
            UCTValidity::Inconclusive => "UCTValidity.Inconclusive",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<UCTValidity> for PyUCTValidity {
    fn from(inner: UCTValidity) -> Self {
        Self { inner }
    }
}

impl From<PyUCTValidity> for UCTValidity {
    fn from(value: PyUCTValidity) -> Self {
        value.inner
    }
}
