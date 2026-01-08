use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::astro::{
    brouwer_to_kozai, cartesian_to_keplerian, efg_to_lla, equinoctial_to_keplerian, get_dll_info,
    gst_teme_to_lla, keplerian_to_cartesian, keplerian_to_equinoctial, kozai_to_brouwer,
    mean_motion_to_sma, osculating_to_mean, sma_to_mean_motion,
};
use crate::DLL_VERSION;

#[pyclass]
pub struct AstroInterface {
    info: String,
}

#[pymethods]
impl AstroInterface {
    #[new]
    fn new() -> PyResult<Self> {
        let info = get_dll_info();
        if !info.contains(DLL_VERSION) {
            return Err(PyRuntimeError::new_err(format!(
                "Expected DLL {} inconsistent with {}",
                DLL_VERSION, info
            )));
        }
        Ok(AstroInterface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    fn keplerian_to_equinoctial(&self, kep: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(keplerian_to_equinoctial(&kep))
    }

    fn equinoctial_to_keplerian(&self, eqnx: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(equinoctial_to_keplerian(&eqnx))
    }

    fn keplerian_to_cartesian(&self, kep: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(keplerian_to_cartesian(&kep))
    }

    fn cartesian_to_keplerian(&self, posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(cartesian_to_keplerian(&posvel))
    }

    fn mean_motion_to_sma(&self, mean_motion: f64) -> PyResult<f64> {
        Ok(mean_motion_to_sma(mean_motion))
    }

    fn sma_to_mean_motion(&self, semi_major_axis: f64) -> PyResult<f64> {
        Ok(sma_to_mean_motion(semi_major_axis))
    }

    fn kozai_to_brouwer(&self, eccentricity: f64, inclination: f64, mean_motion: f64) -> PyResult<f64> {
        Ok(kozai_to_brouwer(eccentricity, inclination, mean_motion))
    }

    fn brouwer_to_kozai(&self, eccentricity: f64, inclination: f64, mean_motion: f64) -> PyResult<f64> {
        Ok(brouwer_to_kozai(eccentricity, inclination, mean_motion))
    }

    fn osculating_to_mean(&self, osc: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(osculating_to_mean(&osc))
    }

    fn gst_teme_to_lla(&self, gst: f64, teme_pos: [f64; 3]) -> PyResult<[f64; 3]> {
        Ok(gst_teme_to_lla(gst, &teme_pos))
    }

    fn efg_to_lla(&self, efg_pos: [f64; 3]) -> PyResult<[f64; 3]> {
        efg_to_lla(&efg_pos).map_err(PyRuntimeError::new_err)
    }
}

pub fn register_astro_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<AstroInterface>()?;
    let class = parent_module.getattr("AstroInterface")?;
    class.setattr("XF_CONV_SGP42SGP", crate::astro::XF_CONV_SGP42SGP)?;
    Ok(())
}
