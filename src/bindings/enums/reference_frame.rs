use crate::enums::ReferenceFrame;
use pyo3::prelude::*;

#[pyclass(name = "ReferenceFrame")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyReferenceFrame {
    inner: ReferenceFrame,
}

#[pymethods]
impl PyReferenceFrame {
    #[classattr]
    pub const TEME: Self = Self {
        inner: ReferenceFrame::TEME,
    };
    #[classattr]
    pub const EFG: Self = Self {
        inner: ReferenceFrame::EFG,
    };
    #[classattr]
    pub const ECR: Self = Self {
        inner: ReferenceFrame::ECR,
    };
    #[classattr]
    pub const J2000: Self = Self {
        inner: ReferenceFrame::J2000,
    };

    #[getter]
    fn value(&self) -> &str {
        match self.inner {
            ReferenceFrame::TEME => "TEME",
            ReferenceFrame::EFG => "EFG",
            ReferenceFrame::ECR => "ECR",
            ReferenceFrame::J2000 => "J2000",
        }
    }

    fn __repr__(&self) -> &str {
        match self.inner {
            ReferenceFrame::TEME => "ReferenceFrame.TEME",
            ReferenceFrame::EFG => "ReferenceFrame.EFG",
            ReferenceFrame::ECR => "ReferenceFrame.ECR",
            ReferenceFrame::J2000 => "ReferenceFrame.J2000",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<ReferenceFrame> for PyReferenceFrame {
    fn from(inner: ReferenceFrame) -> Self {
        Self { inner }
    }
}

impl From<PyReferenceFrame> for ReferenceFrame {
    fn from(value: PyReferenceFrame) -> Self {
        value.inner
    }
}
