use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::environment::{
    get_dll_info, get_earth_flattening, get_earth_mu, get_earth_radius, get_earth_rotation_acceleration,
    get_earth_rotation_rate, get_fundamental_catalog, get_j2, get_j3, get_j4, get_j5, load_from_file,
    set_fundamental_catalog,
};
use crate::DLL_VERSION;

#[pyclass]
pub struct EnvironmentInterface {
    info: String,
}

#[pymethods]
impl EnvironmentInterface {
    #[new]
    #[pyo3(signature = (file_name=None))]
    fn new(file_name: Option<String>) -> PyResult<Self> {
        let info = get_dll_info();
        if !info.contains(DLL_VERSION) {
            return Err(PyRuntimeError::new_err(format!(
                "Expected DLL {} inconsistent with {}",
                DLL_VERSION, info
            )));
        }
        if let Some(file) = file_name {
            load_from_file(&file).map_err(PyRuntimeError::new_err)?;
        }
        Ok(EnvironmentInterface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    #[getter]
    fn earth_radius(&self) -> PyResult<f64> {
        Ok(get_earth_radius())
    }

    #[getter]
    fn earth_rotation_rate(&self) -> PyResult<f64> {
        Ok(get_earth_rotation_rate())
    }

    #[getter]
    fn earth_rotation_acceleration(&self) -> PyResult<f64> {
        Ok(get_earth_rotation_acceleration())
    }

    #[getter]
    fn earth_mu(&self) -> PyResult<f64> {
        Ok(get_earth_mu())
    }

    #[getter]
    fn earth_flattening(&self) -> PyResult<f64> {
        Ok(get_earth_flattening())
    }

    #[getter]
    fn j2(&self) -> PyResult<f64> {
        Ok(get_j2())
    }

    #[getter]
    fn j3(&self) -> PyResult<f64> {
        Ok(get_j3())
    }

    #[getter]
    fn j4(&self) -> PyResult<f64> {
        Ok(get_j4())
    }

    #[getter]
    fn j5(&self) -> PyResult<f64> {
        Ok(get_j5())
    }

    #[getter]
    fn fundamental_catalog(&self) -> PyResult<i32> {
        get_fundamental_catalog().map_err(PyRuntimeError::new_err)
    }

    #[setter]
    fn set_fundamental_catalog(&self, catalog: i32) -> PyResult<()> {
        set_fundamental_catalog(catalog);
        Ok(())
    }
}

pub fn register_environment_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<EnvironmentInterface>()?;
    let class = parent_module.getattr("EnvironmentInterface")?;
    class.setattr("XF_FKMOD_4", crate::environment::XF_FKMOD_4)?;
    class.setattr("XF_FKMOD_5", crate::environment::XF_FKMOD_5)?;
    Ok(())
}
