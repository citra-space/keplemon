use crate::time::TimeSpan;
use pyo3::prelude::*;

#[pyclass(name = "TimeSpan")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyTimeSpan {
    inner: TimeSpan,
}

impl From<TimeSpan> for PyTimeSpan {
    fn from(inner: TimeSpan) -> Self {
        Self { inner }
    }
}

impl From<PyTimeSpan> for TimeSpan {
    fn from(span: PyTimeSpan) -> Self {
        span.inner
    }
}

#[pymethods]
impl PyTimeSpan {
    #[staticmethod]
    pub fn from_days(days: f64) -> Self {
        Self {
            inner: TimeSpan::from_days(days),
        }
    }

    #[staticmethod]
    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            inner: TimeSpan::from_seconds(seconds),
        }
    }

    #[staticmethod]
    pub fn from_minutes(minutes: f64) -> Self {
        Self {
            inner: TimeSpan::from_minutes(minutes),
        }
    }

    #[staticmethod]
    pub fn from_hours(hours: f64) -> Self {
        Self {
            inner: TimeSpan::from_hours(hours),
        }
    }

    pub fn in_days(&self) -> f64 {
        self.inner.in_days()
    }

    pub fn in_seconds(&self) -> f64 {
        self.inner.in_seconds()
    }

    pub fn in_minutes(&self) -> f64 {
        self.inner.in_minutes()
    }

    pub fn in_hours(&self) -> f64 {
        self.inner.in_hours()
    }
}
