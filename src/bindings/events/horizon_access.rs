use crate::bindings::elements::PyHorizonState;
use crate::events::HorizonAccess;
use pyo3::prelude::*;

#[pyclass(name = "HorizonAccess")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyHorizonAccess {
    inner: HorizonAccess,
}

impl From<HorizonAccess> for PyHorizonAccess {
    fn from(inner: HorizonAccess) -> Self {
        Self { inner }
    }
}

impl From<PyHorizonAccess> for HorizonAccess {
    fn from(value: PyHorizonAccess) -> Self {
        value.inner
    }
}

impl PyHorizonAccess {
    pub fn new(satellite_id: String, observatory_id: String, start: &PyHorizonState, end: &PyHorizonState) -> Self {
        let start_core = (*start).into();
        let end_core = (*end).into();
        HorizonAccess::new(satellite_id, observatory_id, &start_core, &end_core).into()
    }
}

#[pymethods]
impl PyHorizonAccess {
    #[getter]
    pub fn get_satellite_id(&self) -> String {
        self.inner.get_satellite_id()
    }

    #[getter]
    pub fn get_observatory_id(&self) -> String {
        self.inner.get_observatory_id()
    }

    #[getter]
    pub fn get_start(&self) -> PyHorizonState {
        PyHorizonState::from(self.inner.get_start())
    }

    #[getter]
    pub fn get_end(&self) -> PyHorizonState {
        PyHorizonState::from(self.inner.get_end())
    }

    #[setter]
    pub fn set_start(&mut self, start: PyHorizonState) {
        self.inner.set_start(start.into());
    }

    #[setter]
    pub fn set_end(&mut self, end: PyHorizonState) {
        self.inner.set_end(end.into());
    }
}
