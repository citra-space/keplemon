use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::DLL_VERSION;
use crate::time::{
    constants_loaded, ds50_to_dtg15, ds50_to_dtg17, ds50_to_dtg19, ds50_to_dtg20,
    ds50_to_ymd_components, ds50_to_year_doy, dtg_to_ds50, get_dll_info,
    get_fk4_greenwich_angle, get_fk5_greenwich_angle, load_constants, tai_to_utc, tai_to_ut1,
    utc_to_tai, utc_to_tt, utc_to_ut1, year_doy_to_ds50, ymd_components_to_ds50,
};

#[pyclass]
pub struct TimeInterface {
    info: String,
}

#[pymethods]
impl TimeInterface {
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
            load_constants(&file).map_err(PyRuntimeError::new_err)?;
        }
        Ok(TimeInterface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    fn ymd_components_to_ds50(
        &self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: f64,
    ) -> PyResult<f64> {
        Ok(ymd_components_to_ds50(year, month, day, hour, minute, second))
    }

    fn utc_to_ut1(&self, ds50_utc: f64) -> PyResult<f64> {
        Ok(utc_to_ut1(ds50_utc))
    }

    fn utc_to_tai(&self, ds50_utc: f64) -> PyResult<f64> {
        Ok(utc_to_tai(ds50_utc))
    }

    fn tai_to_utc(&self, ds50_tai: f64) -> PyResult<f64> {
        Ok(tai_to_utc(ds50_tai))
    }

    fn utc_to_tt(&self, ds50_utc: f64) -> PyResult<f64> {
        Ok(utc_to_tt(ds50_utc))
    }

    fn tai_to_ut1(&self, ds50_tai: f64) -> PyResult<f64> {
        Ok(tai_to_ut1(ds50_tai))
    }

    fn ds50_to_ymd_components(
        &self,
        ds50_utc: f64,
    ) -> PyResult<(i32, i32, i32, i32, i32, f64)> {
        Ok(ds50_to_ymd_components(ds50_utc))
    }

    fn dtg_to_ds50(&self, dtg: String) -> PyResult<f64> {
        Ok(dtg_to_ds50(&dtg))
    }

    fn ds50_to_dtg20(&self, ds50_utc: f64) -> PyResult<String> {
        Ok(ds50_to_dtg20(ds50_utc))
    }

    fn ds50_to_dtg19(&self, ds50_utc: f64) -> PyResult<String> {
        Ok(ds50_to_dtg19(ds50_utc))
    }

    fn ds50_to_dtg17(&self, ds50_utc: f64) -> PyResult<String> {
        Ok(ds50_to_dtg17(ds50_utc))
    }

    fn ds50_to_dtg15(&self, ds50_utc: f64) -> PyResult<String> {
        Ok(ds50_to_dtg15(ds50_utc))
    }

    fn year_doy_to_ds50(&self, year: i32, doy: f64) -> PyResult<f64> {
        Ok(year_doy_to_ds50(year, doy))
    }

    fn ds50_to_year_doy(&self, ds50_utc: f64) -> PyResult<(i32, f64)> {
        Ok(ds50_to_year_doy(ds50_utc))
    }

    fn load_constants(&self, path: String) -> PyResult<()> {
        load_constants(&path).map_err(PyRuntimeError::new_err)
    }

    fn load_time_constants(&self, path: String) -> PyResult<()> {
        load_constants(&path).map_err(PyRuntimeError::new_err)
    }

    fn get_fk4_greenwich_angle(&self, ds50_ut1: f64) -> PyResult<f64> {
        Ok(get_fk4_greenwich_angle(ds50_ut1))
    }

    fn get_fk5_greenwich_angle(&self, ds50_ut1: f64) -> PyResult<f64> {
        Ok(get_fk5_greenwich_angle(ds50_ut1))
    }

    #[getter]
    fn constants_loaded(&self) -> PyResult<bool> {
        Ok(constants_loaded())
    }

    fn time_constants_loaded(&self) -> PyResult<bool> {
        Ok(constants_loaded())
    }
}

pub fn register_time_func_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<TimeInterface>()?;
    Ok(())
}
