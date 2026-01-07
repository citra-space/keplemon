#![allow(non_upper_case_globals)]

use crate::enums::MeanEquinox;
use pyo3::prelude::*;

#[pyclass(name = "MeanEquinox")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyMeanEquinox {
    inner: MeanEquinox,
}

#[pymethods]
impl PyMeanEquinox {
    #[classattr]
    pub const OfDate: Self = Self {
        inner: MeanEquinox::OfDate,
    };
    #[classattr]
    pub const J2000: Self = Self {
        inner: MeanEquinox::J2000,
    };

    #[classattr]
    pub const B1950: Self = Self {
        inner: MeanEquinox::B1950,
    };

    #[classattr]
    pub const OfYear: Self = Self {
        inner: MeanEquinox::OfYear,
    };

    #[getter]
    pub fn get_value(&self) -> i32 {
        match self.inner {
            MeanEquinox::OfDate => MeanEquinox::OfDate.get_value(),
            MeanEquinox::J2000 => MeanEquinox::J2000.get_value(),
            MeanEquinox::B1950 => MeanEquinox::B1950.get_value(),
            MeanEquinox::OfYear => MeanEquinox::OfYear.get_value(),
        }
    }

    fn __repr__(&self) -> &str {
        match self.inner {
            MeanEquinox::OfDate => "MeanEquinox.OfDate",
            MeanEquinox::OfYear => "MeanEquinox.OfYear",
            MeanEquinox::B1950 => "MeanEquinox.B1950",
            MeanEquinox::J2000 => "MeanEquinox.J2000",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<MeanEquinox> for PyMeanEquinox {
    fn from(inner: MeanEquinox) -> Self {
        Self { inner }
    }
}

impl From<PyMeanEquinox> for MeanEquinox {
    fn from(value: PyMeanEquinox) -> Self {
        value.inner
    }
}
