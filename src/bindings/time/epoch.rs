use super::{PyTimeComponents, PyTimeSpan};
use crate::bindings::enums::PyTimeSystem;
use crate::enums::TimeSystem;
use crate::time::{Epoch, TimeComponents, TimeSpan};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass(name = "Epoch")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PyEpoch {
    inner: Epoch,
}

impl From<Epoch> for PyEpoch {
    fn from(inner: Epoch) -> Self {
        Self { inner }
    }
}

impl From<PyEpoch> for Epoch {
    fn from(value: PyEpoch) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyEpoch {
    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __add__(&self, span: &PyTimeSpan) -> Self {
        let span = TimeSpan::from_days(span.in_days());
        Self {
            inner: self.inner + span,
        }
    }

    fn __sub__<'py>(&self, other: &Bound<'py, PyAny>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(other_epoch) = other.extract::<PyEpoch>() {
            let result: PyTimeSpan = (self.inner - other_epoch.inner).into();
            Ok(Py::new(py, result)?.into_bound(py).into_any())
        } else if let Ok(other_span) = other.extract::<PyTimeSpan>() {
            let span = TimeSpan::from_days(other_span.in_days());
            let result = Self {
                inner: self.inner - span,
            };
            Ok(Py::new(py, result)?.into_bound(py).into_any())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported operand type for -",
            ))
        }
    }

    #[getter]
    pub fn get_days_since_1950(&self) -> f64 {
        self.inner.days_since_1950
    }

    #[getter]
    pub fn get_time_system(&self) -> PyTimeSystem {
        PyTimeSystem::from(self.inner.get_time_system())
    }

    #[staticmethod]
    pub fn from_days_since_1950(days_since_1950: f64, time_system: PyTimeSystem) -> Self {
        let time_system: TimeSystem = time_system.into();
        Self {
            inner: Epoch::from_days_since_1950(days_since_1950, time_system),
        }
    }

    #[staticmethod]
    pub fn from_iso(iso: &str, time_system: PyTimeSystem) -> Self {
        let time_system: TimeSystem = time_system.into();
        Self {
            inner: Epoch::from_iso(iso, time_system),
        }
    }

    #[staticmethod]
    pub fn from_dtg(dtg: &str, time_system: PyTimeSystem) -> Self {
        let time_system: TimeSystem = time_system.into();
        Self {
            inner: Epoch::from_dtg(dtg, time_system),
        }
    }

    #[staticmethod]
    pub fn from_time_components(components: &PyTimeComponents, time_system: PyTimeSystem) -> Self {
        let components: TimeComponents = (*components).into();
        let time_system: TimeSystem = time_system.into();
        Self {
            inner: Epoch::from_time_components(&components, time_system),
        }
    }

    #[staticmethod]
    pub fn now<'py>(py: Python<'py>) -> PyResult<Self> {
        let datetime = py.import("datetime")?.getattr("datetime")?;
        let timezone = py.import("datetime")?.getattr("timezone")?;
        let utc = timezone.getattr("utc")?;
        let dt = datetime.call_method1("now", (utc,))?;
        Self::from_datetime(&dt)
    }

    #[staticmethod]
    pub fn from_datetime<'py>(dt: &Bound<'py, PyAny>) -> PyResult<Self> {
        let datetime = dt.py().import("datetime")?.getattr("datetime")?;
        if !dt.is_instance(&datetime)? {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected datetime.datetime instance",
            ));
        }
        let iso = dt.call_method0("isoformat")?.extract::<String>()?;
        let utc_dt = if iso.ends_with('Z') || iso.ends_with("+00:00") {
            dt.to_owned()
        } else {
            let timezone = dt.py().import("datetime")?.getattr("timezone")?;
            let utc = timezone.getattr("utc")?;
            dt.call_method1("astimezone", (utc,))?
        };
        let iso = utc_dt
            .call_method0("isoformat")?
            .extract::<String>()?
            .replace("+00:00", "Z");
        Ok(Self {
            inner: Epoch::from_iso(&iso, TimeSystem::UTC),
        })
    }

    pub fn to_dtg_20(&self) -> String {
        self.inner.to_dtg_20()
    }

    pub fn to_dtg_19(&self) -> String {
        self.inner.to_dtg_19()
    }

    pub fn to_dtg_17(&self) -> String {
        self.inner.to_dtg_17()
    }

    pub fn to_dtg_15(&self) -> String {
        self.inner.to_dtg_15()
    }

    pub fn to_time_components(&self) -> PyTimeComponents {
        let components = self.inner.to_time_components();
        PyTimeComponents::from(components)
    }

    #[getter]
    pub fn get_day_of_year(&self) -> f64 {
        self.inner.get_day_of_year()
    }

    pub fn to_fk4_greenwich_angle(&self) -> f64 {
        self.inner.to_fk4_greenwich_angle()
    }

    pub fn to_fk5_greenwich_angle(&self) -> f64 {
        self.inner.to_fk5_greenwich_angle()
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.inner.days_since_1950 > other.inner.days_since_1950
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.inner.days_since_1950 < other.inner.days_since_1950
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner.days_since_1950 != other.inner.days_since_1950
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.inner.days_since_1950 >= other.inner.days_since_1950
    }

    fn __le__(&self, other: &Self) -> bool {
        self.inner.days_since_1950 <= other.inner.days_since_1950
    }

    pub fn to_iso(&self) -> String {
        self.inner.to_iso()
    }

    pub fn to_datetime<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let iso = self.inner.to_iso();
        let iso = iso.replace('Z', "+00:00");
        let datetime = py.import("datetime")?.getattr("datetime")?;
        datetime.call_method1("fromisoformat", (iso,))
    }

    pub fn to_system(&self, time_system: PyTimeSystem) -> PyResult<Self> {
        self.inner
            .to_system(time_system.into())
            .map(|epoch| Self { inner: epoch })
            .map_err(PyRuntimeError::new_err)
    }
}
