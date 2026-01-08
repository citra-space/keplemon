use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::DLL_VERSION;
use crate::tle::{self, ParsedTLE, XA_TLE_SIZE};

#[pyclass]
pub struct TLEInterface {
    info: String,
}

#[pymethods]
impl TLEInterface {
    #[new]
    fn new() -> PyResult<Self> {
        let info = tle::get_dll_info();
        if !info.contains(DLL_VERSION) {
            return Err(PyRuntimeError::new_err(format!(
                "Expected DLL {} inconsistent with {}",
                DLL_VERSION, info
            )));
        }
        Ok(TLEInterface { info })
    }

    #[getter(info)]
    fn get_info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    fn fix_blank_exponent_sign(&self, mut line_1: String) -> PyResult<String> {
        tle::fix_blank_exponent_sign(&mut line_1);
        Ok(line_1)
    }

    fn add_check_sums(&self, mut line_1: String, mut line_2: String) -> PyResult<(String, String)> {
        tle::add_check_sums(&mut line_1, &mut line_2).map_err(PyRuntimeError::new_err)?;
        Ok((line_1, line_2))
    }

    fn lines_to_arrays(&self, line_1: String, line_2: String) -> PyResult<([f64; XA_TLE_SIZE], String)> {
        tle::lines_to_arrays(&line_1, &line_2).map_err(PyRuntimeError::new_err)
    }

    fn arrays_to_lines(&self, xa_tle: [f64; XA_TLE_SIZE], xs_tle: String) -> PyResult<(String, String)> {
        tle::arrays_to_lines(xa_tle, &xs_tle).map_err(PyRuntimeError::new_err)
    }

    fn get_check_sums(&self, line_1: String, line_2: String) -> PyResult<(i32, i32)> {
        tle::get_check_sums(&line_1, &line_2).map_err(PyRuntimeError::new_err)
    }

    fn load_lines(&self, line_1: String, line_2: String) -> PyResult<i64> {
        Ok(tle::load_lines(&line_1, &line_2))
    }

    fn load_arrays(&self, xa_tle: [f64; XA_TLE_SIZE], xs_tle: String) -> PyResult<i64> {
        tle::load_arrays(xa_tle, &xs_tle).map_err(PyRuntimeError::new_err)
    }

    fn load_file(&self, file_path: String) -> PyResult<i32> {
        tle::load_file(&file_path).map_err(PyRuntimeError::new_err)
    }

    fn clear(&self) -> PyResult<()> {
        tle::clear().map_err(PyRuntimeError::new_err)
    }

    fn remove(&self, sat_key: i64) -> PyResult<()> {
        tle::remove(sat_key);
        Ok(())
    }

    fn get_count(&self) -> PyResult<i32> {
        Ok(tle::get_count())
    }

    fn get_keys(&self, order: i32) -> PyResult<Vec<i64>> {
        Ok(tle::get_keys(order))
    }

    fn get_lines(&self, sat_key: i64) -> PyResult<(String, String)> {
        tle::get_lines(sat_key).map_err(PyRuntimeError::new_err)
    }

    fn get_arrays(&self, sat_key: i64) -> PyResult<([f64; XA_TLE_SIZE], String)> {
        tle::get_arrays(sat_key).map_err(PyRuntimeError::new_err)
    }

    fn parse_lines(&self, line_1: String, line_2: String) -> PyResult<PyParsedTLE> {
        let parsed = tle::parse_lines(&line_1, &line_2).map_err(PyRuntimeError::new_err)?;
        Ok(parsed.into())
    }
}

#[pyclass(name = "ParsedTLE")]
#[derive(Clone)]
pub struct PyParsedTLE {
    inner: ParsedTLE,
}

impl From<ParsedTLE> for PyParsedTLE {
    fn from(value: ParsedTLE) -> Self {
        PyParsedTLE { inner: value }
    }
}

#[pymethods]
impl PyParsedTLE {
    #[new]
    fn new() -> Self {
        PyParsedTLE {
            inner: ParsedTLE::default(),
        }
    }

    #[getter(epoch)]
    fn get_epoch(&self) -> PyResult<f64> {
        Ok(self.inner.epoch)
    }

    #[setter(epoch)]
    fn set_epoch(&mut self, value: f64) -> PyResult<()> {
        self.inner.epoch = value;
        Ok(())
    }

    #[getter(norad_id)]
    fn get_norad_id(&self) -> PyResult<i32> {
        Ok(self.inner.norad_id)
    }

    #[setter(norad_id)]
    fn set_norad_id(&mut self, value: i32) -> PyResult<()> {
        self.inner.norad_id = value;
        Ok(())
    }

    #[getter(inclination)]
    fn get_inclination(&self) -> PyResult<f64> {
        Ok(self.inner.inclination)
    }

    #[setter(inclination)]
    fn set_inclination(&mut self, value: f64) -> PyResult<()> {
        self.inner.inclination = value;
        Ok(())
    }

    #[getter(raan)]
    fn get_raan(&self) -> PyResult<f64> {
        Ok(self.inner.raan)
    }

    #[setter(raan)]
    fn set_raan(&mut self, value: f64) -> PyResult<()> {
        self.inner.raan = value;
        Ok(())
    }

    #[getter(eccentricity)]
    fn get_eccentricity(&self) -> PyResult<f64> {
        Ok(self.inner.eccentricity)
    }

    #[setter(eccentricity)]
    fn set_eccentricity(&mut self, value: f64) -> PyResult<()> {
        self.inner.eccentricity = value;
        Ok(())
    }

    #[getter(argument_of_perigee)]
    fn get_argument_of_perigee(&self) -> PyResult<f64> {
        Ok(self.inner.argument_of_perigee)
    }

    #[setter(argument_of_perigee)]
    fn set_argument_of_perigee(&mut self, value: f64) -> PyResult<()> {
        self.inner.argument_of_perigee = value;
        Ok(())
    }

    #[getter(mean_anomaly)]
    fn get_mean_anomaly(&self) -> PyResult<f64> {
        Ok(self.inner.mean_anomaly)
    }

    #[setter(mean_anomaly)]
    fn set_mean_anomaly(&mut self, value: f64) -> PyResult<()> {
        self.inner.mean_anomaly = value;
        Ok(())
    }

    #[getter(mean_motion)]
    fn get_mean_motion(&self) -> PyResult<f64> {
        Ok(self.inner.mean_motion)
    }

    #[setter(mean_motion)]
    fn set_mean_motion(&mut self, value: f64) -> PyResult<()> {
        self.inner.mean_motion = value;
        Ok(())
    }

    #[getter(ephemeris_type)]
    fn get_ephemeris_type(&self) -> PyResult<i32> {
        Ok(self.inner.get_ephemeris_type())
    }

    #[setter(ephemeris_type)]
    fn set_ephemeris_type(&mut self, value: i32) -> PyResult<()> {
        self.inner.set_ephemeris_type(value);
        Ok(())
    }

    #[getter(element_set_number)]
    fn get_element_set_number(&self) -> PyResult<i32> {
        Ok(self.inner.element_set_number)
    }

    #[setter(element_set_number)]
    fn set_element_set_number(&mut self, value: i32) -> PyResult<()> {
        self.inner.element_set_number = value;
        Ok(())
    }

    #[getter(revolution_number)]
    fn get_revolution_number(&self) -> PyResult<i32> {
        Ok(self.inner.revolution_number)
    }

    #[setter(revolution_number)]
    fn set_revolution_number(&mut self, value: i32) -> PyResult<()> {
        self.inner.revolution_number = value;
        Ok(())
    }

    #[getter(designator)]
    fn get_designator(&self) -> PyResult<Option<String>> {
        Ok(self.inner.designator.clone())
    }

    #[setter(designator)]
    fn set_designator(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.designator = value;
        Ok(())
    }

    #[getter(classification)]
    fn get_classification(&self) -> PyResult<String> {
        Ok(self.inner.classification.clone())
    }

    #[setter(classification)]
    fn set_classification(&mut self, value: String) -> PyResult<()> {
        self.inner.classification = value;
        Ok(())
    }

    #[getter(mean_motion_1st_derivative)]
    fn get_mean_motion_1st_derivative(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.get_mean_motion_1st_derivative())
    }

    #[setter(mean_motion_1st_derivative)]
    fn set_mean_motion_1st_derivative(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.set_mean_motion_1st_derivative(value);
        Ok(())
    }

    #[getter(mean_motion_2nd_derivative)]
    fn get_mean_motion_2nd_derivative(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.get_mean_motion_2nd_derivative())
    }

    #[setter(mean_motion_2nd_derivative)]
    fn set_mean_motion_2nd_derivative(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.set_mean_motion_2nd_derivative(value);
        Ok(())
    }

    #[getter(b_star)]
    fn get_b_star(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.get_b_star())
    }

    #[setter(b_star)]
    fn set_b_star(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.set_b_star(value);
        Ok(())
    }

    #[getter(ballistic_coefficient)]
    fn get_ballistic_coefficient(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.get_ballistic_coefficient())
    }

    #[setter(ballistic_coefficient)]
    fn set_ballistic_coefficient(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.set_ballistic_coefficient(value);
        Ok(())
    }

    #[getter(srp_coefficient)]
    fn get_srp_coefficient(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.get_srp_coefficient())
    }

    #[setter(srp_coefficient)]
    fn set_srp_coefficient(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.set_srp_coefficient(value);
        Ok(())
    }

    #[pyo3(signature = (remove_nulls=false))]
    fn get_lines(&self, remove_nulls: bool) -> PyResult<(String, String)> {
        self.inner.get_lines(remove_nulls).map_err(PyRuntimeError::new_err)
    }
}

pub fn register_tle_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<TLEInterface>()?;
    parent_module.add_class::<PyParsedTLE>()?;
    let class = parent_module.getattr("TLEInterface")?;
    class.setattr("TLETYPE_SGP", tle::TLETYPE_SGP)?;
    class.setattr("TLETYPE_SGP4", tle::TLETYPE_SGP4)?;
    class.setattr("TLETYPE_XP", tle::TLETYPE_XP)?;
    class.setattr("TLETYPE_SP", tle::TLETYPE_SP)?;
    Ok(())
}
