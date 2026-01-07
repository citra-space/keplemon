#![allow(non_upper_case_globals)]

use crate::enums::Classification;
use pyo3::prelude::*;

#[pyclass(name = "Classification")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PyClassification {
    inner: Classification,
}

#[pymethods]
impl PyClassification {
    #[classattr]
    pub const Unclassified: Self = Self {
        inner: Classification::Unclassified,
    };
    #[classattr]
    pub const Confidential: Self = Self {
        inner: Classification::Confidential,
    };
    #[classattr]
    pub const Secret: Self = Self {
        inner: Classification::Secret,
    };

    #[getter]
    fn value(&self) -> &str {
        self.inner.as_char()
    }

    fn __repr__(&self) -> &str {
        match self.inner {
            Classification::Unclassified => "Classification.Unclassified",
            Classification::Confidential => "Classification.Confidential",
            Classification::Secret => "Classification.Secret",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<Classification> for PyClassification {
    fn from(inner: Classification) -> Self {
        Self { inner }
    }
}

impl From<PyClassification> for Classification {
    fn from(value: PyClassification) -> Self {
        value.inner
    }
}
