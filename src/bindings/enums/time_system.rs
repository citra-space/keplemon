use crate::enums::TimeSystem;
use pyo3::prelude::*;
use std::fmt::{Display, Formatter, Result};

#[pyclass(name = "TimeSystem")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyTimeSystem {
    inner: TimeSystem,
}

impl Display for PyTimeSystem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.inner {
            TimeSystem::UTC => write!(f, "UTC"),
            TimeSystem::TAI => write!(f, "TAI"),
            TimeSystem::UT1 => write!(f, "UT1"),
            TimeSystem::TT => write!(f, "TT"),
        }
    }
}

#[pymethods]
impl PyTimeSystem {
    #[classattr]
    pub const UTC: Self = Self { inner: TimeSystem::UTC };
    #[classattr]
    pub const TAI: Self = Self { inner: TimeSystem::TAI };
    #[classattr]
    pub const UT1: Self = Self { inner: TimeSystem::UT1 };
    #[classattr]
    pub const TT: Self = Self { inner: TimeSystem::TT };

    #[getter]
    fn value(&self) -> &str {
        match self.inner {
            TimeSystem::UTC => "UTC",
            TimeSystem::TAI => "TAI",
            TimeSystem::UT1 => "UT1",
            TimeSystem::TT => "TT",
        }
    }

    fn __repr__(&self) -> &str {
        match self.inner {
            TimeSystem::UTC => "TimeSystem.UTC",
            TimeSystem::TAI => "TimeSystem.TAI",
            TimeSystem::UT1 => "TimeSystem.UT1",
            TimeSystem::TT => "TimeSystem.TT",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl From<TimeSystem> for PyTimeSystem {
    fn from(inner: TimeSystem) -> Self {
        Self { inner }
    }
}

impl From<PyTimeSystem> for TimeSystem {
    fn from(value: PyTimeSystem) -> Self {
        value.inner
    }
}
