use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::DLL_VERSION;
use crate::sgp4::{self, XA_SGP4OUT_SIZE};
use crate::tle;

#[pyclass]
pub struct SGP4Interface {
    info: String,
}

#[pymethods]
impl SGP4Interface {
    #[new]
    fn new() -> PyResult<Self> {
        let info = sgp4::get_dll_info();
        if !info.contains(DLL_VERSION) {
            return Err(PyRuntimeError::new_err(format!(
                "Expected DLL {} inconsistent with {}",
                DLL_VERSION, info
            )));
        }
        Ok(SGP4Interface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    fn load(&self, sat_key: i64) -> PyResult<()> {
        sgp4::load(sat_key).map_err(PyRuntimeError::new_err)
    }

    fn remove(&self, sat_key: i64) -> PyResult<()> {
        sgp4::remove(sat_key).map_err(PyRuntimeError::new_err)
    }

    fn clear(&self) -> PyResult<()> {
        sgp4::clear().map_err(PyRuntimeError::new_err)
    }

    fn get_count(&self) -> PyResult<i32> {
        Ok(sgp4::get_count())
    }

    fn get_position_velocity_lla(&self, sat_key: i64, ds50_utc: f64) -> PyResult<(f64, [f64; 3], [f64; 3], [f64; 3])> {
        sgp4::get_position_velocity_lla(sat_key, ds50_utc).map_err(PyRuntimeError::new_err)
    }

    fn get_position_velocity(&self, sat_key: i64, ds50_utc: f64) -> PyResult<([f64; 3], [f64; 3])> {
        sgp4::get_position_velocity(sat_key, ds50_utc).map_err(PyRuntimeError::new_err)
    }

    fn get_lla(&self, sat_key: i64, ds50_utc: f64) -> PyResult<[f64; 3]> {
        sgp4::get_lla(sat_key, ds50_utc).map_err(PyRuntimeError::new_err)
    }

    fn get_position(&self, sat_key: i64, ds50_utc: f64) -> PyResult<[f64; 3]> {
        sgp4::get_position(sat_key, ds50_utc).map_err(PyRuntimeError::new_err)
    }

    fn get_full_state(&self, sat_key: i64, ds50_utc: f64) -> PyResult<[f64; XA_SGP4OUT_SIZE]> {
        sgp4::get_full_state(sat_key, ds50_utc).map_err(PyRuntimeError::new_err)
    }

    fn get_equinoctial(&self, sat_key: i64, ds50_utc: f64) -> PyResult<[f64; 6]> {
        sgp4::get_equinoctial(sat_key, ds50_utc).map_err(PyRuntimeError::new_err)
    }

    fn get_ephemeris(
        &self,
        sat_key: i64,
        start: f64,
        stop: f64,
        step: f64,
        frame: i32,
    ) -> PyResult<Vec<f64>> {
        sgp4::get_ephemeris(sat_key, start, stop, step, frame).map_err(PyRuntimeError::new_err)
    }

    fn array_to_ephemeris(
        &self,
        xa_tle: [f64; tle::XA_TLE_SIZE],
        start: f64,
        stop: f64,
        step: f64,
        frame: i32,
    ) -> PyResult<Vec<f64>> {
        sgp4::array_to_ephemeris(&xa_tle, start, stop, step, frame).map_err(PyRuntimeError::new_err)
    }

    fn fit_xp_array(
        &self,
        epoch: f64,
        posvel: [f64; 6],
        ballistic_coefficient: Option<f64>,
        srp_coefficient: Option<f64>,
    ) -> PyResult<[f64; tle::XA_TLE_SIZE]> {
        sgp4::fit_xp_array(epoch, &posvel, ballistic_coefficient, srp_coefficient).map_err(PyRuntimeError::new_err)
    }

    fn fit_sgp4_array(
        &self,
        epoch: f64,
        posvel: [f64; 6],
        b_star: Option<f64>,
    ) -> PyResult<[f64; tle::XA_TLE_SIZE]> {
        sgp4::fit_sgp4_array(epoch, &posvel, b_star).map_err(PyRuntimeError::new_err)
    }

    fn get_positions_velocities(&self, sat_keys: Vec<i64>, ds50_utc: f64) -> PyResult<Vec<f64>> {
        sgp4::get_positions_velocities(&sat_keys, ds50_utc).map_err(PyRuntimeError::new_err)
    }

    fn set_license_directory(&self, lic_file_path: String) -> PyResult<()> {
        sgp4::set_license_directory(&lic_file_path);
        Ok(())
    }

    fn get_license_directory(&self) -> PyResult<String> {
        Ok(sgp4::get_license_directory())
    }

    fn reepoch_tle(&self, sat_key: i64, re_epoch_ds50_utc: f64) -> PyResult<(String, String)> {
        sgp4::reepoch_tle(sat_key, re_epoch_ds50_utc).map_err(PyRuntimeError::new_err)
    }
}

pub fn register_sgp4_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<SGP4Interface>()?;
    let class = parent_module.getattr("SGP4Interface")?;
    class.setattr("SGP4_EPHEM_ECI", sgp4::SGP4_EPHEM_ECI)?;
    class.setattr("SGP4_EPHEM_J2K", sgp4::SGP4_EPHEM_J2K)?;
    class.setattr("SGP4_TIMETYPE_MSE", sgp4::SGP4_TIMETYPE_MSE)?;
    class.setattr("SGP4_TIMETYPE_DS50UTC", sgp4::SGP4_TIMETYPE_DS50UTC)?;
    class.setattr("DYN_SS_BASIC", sgp4::DYN_SS_BASIC)?;
    class.setattr("GP_ERR_NONE", sgp4::GP_ERR_NONE)?;
    class.setattr("GP_ERR_BADFK", sgp4::GP_ERR_BADFK)?;
    class.setattr("GP_ERR_ANEGATIVE", sgp4::GP_ERR_ANEGATIVE)?;
    class.setattr("GP_ERR_ATOOLARGE", sgp4::GP_ERR_ATOOLARGE)?;
    class.setattr("GP_ERR_EHYPERPOLIC", sgp4::GP_ERR_EHYPERPOLIC)?;
    class.setattr("GP_ERR_ENEGATIVE", sgp4::GP_ERR_ENEGATIVE)?;
    class.setattr("GP_ERR_MATOOLARGE", sgp4::GP_ERR_MATOOLARGE)?;
    class.setattr("GP_ERR_E2TOOLARGE", sgp4::GP_ERR_E2TOOLARGE)?;
    Ok(())
}
