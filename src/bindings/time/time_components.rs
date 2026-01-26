use crate::time::TimeComponents;
use pyo3::prelude::*;

#[pyclass(name = "TimeComponents")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyTimeComponents {
    inner: TimeComponents,
}

impl From<TimeComponents> for PyTimeComponents {
    fn from(inner: TimeComponents) -> Self {
        Self { inner }
    }
}

impl From<PyTimeComponents> for TimeComponents {
    fn from(value: PyTimeComponents) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyTimeComponents {
    #[getter]
    pub fn year(&self) -> i32 {
        self.inner.year
    }

    #[getter]
    pub fn month(&self) -> i32 {
        self.inner.month
    }

    #[getter]
    pub fn day(&self) -> i32 {
        self.inner.day
    }

    #[getter]
    pub fn hour(&self) -> i32 {
        self.inner.hour
    }

    #[getter]
    pub fn minute(&self) -> i32 {
        self.inner.minute
    }

    #[getter]
    pub fn second(&self) -> f64 {
        self.inner.second
    }

    #[setter]
    pub fn set_year(&mut self, year: i32) {
        self.inner.year = year;
    }

    #[setter]
    pub fn set_month(&mut self, month: i32) {
        self.inner.month = month;
    }

    #[setter]
    pub fn set_day(&mut self, day: i32) {
        self.inner.day = day;
    }

    #[setter]
    pub fn set_hour(&mut self, hour: i32) {
        self.inner.hour = hour;
    }

    #[setter]
    pub fn set_minute(&mut self, minute: i32) {
        self.inner.minute = minute;
    }

    #[setter]
    pub fn set_second(&mut self, second: f64) {
        self.inner.second = second;
    }

    #[new]
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: f64) -> Self {
        TimeComponents::new(year, month, day, hour, minute, second).into()
    }

    pub fn to_iso(&self) -> String {
        self.inner.to_iso()
    }

    #[staticmethod]
    pub fn from_iso(iso: &str) -> Self {
        TimeComponents::from_iso(iso).into()
    }

    fn __eq__(&self, other: &Self) -> PyResult<bool> {
        Ok(self.inner == other.inner)
    }

    fn __ne__(&self, other: &Self) -> PyResult<bool> {
        Ok(self.inner != other.inner)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "TimeComponents(year={}, month={}, day={}, hour={}, minute={}, second={})",
            self.inner.year, self.inner.month, self.inner.day, self.inner.hour, self.inner.minute, self.inner.second
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_iso())
    }
}
