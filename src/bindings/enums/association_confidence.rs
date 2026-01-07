#![allow(non_upper_case_globals)]

use crate::enums::AssociationConfidence;
use pyo3::prelude::*;

#[pyclass(name = "AssociationConfidence")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyAssociationConfidence {
    inner: AssociationConfidence,
}

#[pymethods]
impl PyAssociationConfidence {
    #[classattr]
    pub const High: Self = Self {
        inner: AssociationConfidence::High,
    };
    #[classattr]
    pub const Medium: Self = Self {
        inner: AssociationConfidence::Medium,
    };
    #[classattr]
    pub const Low: Self = Self {
        inner: AssociationConfidence::Low,
    };

    #[getter]
    fn value(&self) -> &str {
        match self.inner {
            AssociationConfidence::High => "High",
            AssociationConfidence::Medium => "Medium",
            AssociationConfidence::Low => "Low",
        }
    }

    fn __repr__(&self) -> &str {
        match self.inner {
            AssociationConfidence::High => "AssociationConfidence.High",
            AssociationConfidence::Medium => "AssociationConfidence.Medium",
            AssociationConfidence::Low => "AssociationConfidence.Low",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<AssociationConfidence> for PyAssociationConfidence {
    fn from(inner: AssociationConfidence) -> Self {
        Self { inner }
    }
}

impl From<PyAssociationConfidence> for AssociationConfidence {
    fn from(value: PyAssociationConfidence) -> Self {
        value.inner
    }
}
