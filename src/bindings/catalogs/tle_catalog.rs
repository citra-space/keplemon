use crate::bindings::elements::{PyOrbitPlotData, PyTLE};
use crate::catalogs::TLECatalog;
use crate::elements::TLE;
use pyo3::prelude::*;

#[pyclass(name = "TLECatalog")]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PyTLECatalog {
    inner: TLECatalog,
}

impl From<TLECatalog> for PyTLECatalog {
    fn from(inner: TLECatalog) -> Self {
        Self { inner }
    }
}

impl From<PyTLECatalog> for TLECatalog {
    fn from(value: PyTLECatalog) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyTLECatalog {
    #[new]
    pub fn new() -> Self {
        TLECatalog::new().into()
    }

    pub fn add(&mut self, tle: PyTLE) {
        let tle: TLE = tle.into();
        self.inner.add(tle);
    }

    pub fn keys(&self) -> Vec<String> {
        self.inner.keys()
    }

    pub fn get(&self, satellite_id: String) -> Option<PyTLE> {
        self.inner.get(satellite_id).map(PyTLE::from)
    }

    pub fn remove(&mut self, satellite_id: String) {
        self.inner.remove(satellite_id);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    fn __getitem__(&self, satellite_id: String) -> PyResult<PyTLE> {
        match self.inner.get(satellite_id.clone()) {
            Some(tle) => Ok(PyTLE::from(tle)),
            None => Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "Invalid key: {}",
                satellite_id
            ))),
        }
    }

    #[getter]
    pub fn get_count(&self) -> usize {
        self.inner.get_count()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.inner.name = Some(name);
    }

    #[staticmethod]
    pub fn from_tle_file(file_path: &str) -> PyResult<PyTLECatalog> {
        TLECatalog::from_tle_file(file_path)
            .map(PyTLECatalog::from)
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    pub fn get_plot_data(&self) -> PyOrbitPlotData {
        PyOrbitPlotData::from(self.inner.get_plot_data())
    }
}
