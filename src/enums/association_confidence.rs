use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssociationConfidence {
    High,
    Medium,
    Low,
}

#[pymethods]
impl AssociationConfidence {
    #[getter]
    fn value(&self) -> &str {
        match self {
            AssociationConfidence::High => "High",
            AssociationConfidence::Medium => "Medium",
            AssociationConfidence::Low => "Low",
        }
    }

    fn __repr__(&self) -> &str {
        match self {
            AssociationConfidence::High => "AssociationConfidence.High",
            AssociationConfidence::Medium => "AssociationConfidence.Medium",
            AssociationConfidence::Low => "AssociationConfidence.Low",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}
