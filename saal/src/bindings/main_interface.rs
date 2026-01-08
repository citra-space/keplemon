use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::{
    DLL_VERSION, get_dll_info, get_duplicate_key_mode, get_elset_key_mode, get_key_mode, get_last_error_message,
    get_last_info_message, load_from_file, reset_key_mode, set_duplicate_key_mode, set_elset_key_mode, set_key_mode,
};

#[pyclass]
pub struct MainInterface {
    info: String,
}

#[pymethods]
impl MainInterface {
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
        Ok(MainInterface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    #[getter]
    fn last_error_message(&self) -> PyResult<String> {
        Ok(get_last_error_message())
    }

    #[getter]
    fn last_info_message(&self) -> PyResult<String> {
        Ok(get_last_info_message())
    }

    #[getter]
    fn key_mode(&self) -> PyResult<i32> {
        get_key_mode().map_err(PyRuntimeError::new_err)
    }

    #[setter]
    fn set_key_mode(&self, mode: i32) -> PyResult<()> {
        set_key_mode(mode).map_err(PyRuntimeError::new_err)
    }

    fn reset_key_mode(&self) {
        reset_key_mode();
    }

    #[getter]
    fn elset_key_mode(&self) -> PyResult<i32> {
        get_elset_key_mode().map_err(PyRuntimeError::new_err)
    }

    #[setter]
    fn set_elset_key_mode(&self, mode: i32) -> PyResult<()> {
        set_elset_key_mode(mode).map_err(PyRuntimeError::new_err)
    }

    #[getter]
    fn duplicate_key_mode(&self) -> PyResult<i32> {
        get_duplicate_key_mode().map_err(PyRuntimeError::new_err)
    }

    #[setter]
    fn set_duplicate_key_mode(&self, mode: i32) -> PyResult<()> {
        set_duplicate_key_mode(mode).map_err(PyRuntimeError::new_err)
    }

    #[classattr]
    const DLL_VERSION: &'static str = DLL_VERSION;
}
pub fn register_main_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<MainInterface>()?;
    let class = parent_module.getattr("MainInterface")?;
    class.setattr("ALL_KEYMODE_NODUP", crate::ALL_KEYMODE_NODUP)?;
    class.setattr("ALL_KEYMODE_DMA", crate::ALL_KEYMODE_DMA)?;
    class.setattr("ELSET_KEYMODE_NODUP", crate::ELSET_KEYMODE_NODUP)?;
    class.setattr("ELSET_KEYMODE_DMA", crate::ELSET_KEYMODE_DMA)?;
    class.setattr("DUPKEY_ZERO", crate::DUPKEY_ZERO)?;
    class.setattr("DUPKEY_ACTUAL", crate::DUPKEY_ACTUAL)?;
    class.setattr("IDX_ORDER_ASC", crate::IDX_ORDER_ASC)?;
    class.setattr("IDX_ORDER_DES", crate::IDX_ORDER_DES)?;
    class.setattr("IDX_ORDER_READ", crate::IDX_ORDER_READ)?;
    class.setattr("IDX_ORDER_QUICK", crate::IDX_ORDER_QUICK)?;
    class.setattr("TIME_IS_MSE", crate::TIME_IS_MSE)?;
    class.setattr("TIME_IS_TAI", crate::TIME_IS_TAI)?;
    class.setattr("TIME_IS_UTC", crate::TIME_IS_UTC)?;
    Ok(())
}
