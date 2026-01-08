use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::obs::{self, ParsedB3};
use crate::DLL_VERSION;

#[pyclass]
pub struct ObsInterface {
    info: String,
}

#[pymethods]
impl ObsInterface {
    #[new]
    fn new() -> PyResult<Self> {
        let info = obs::get_dll_info();
        if !info.contains(DLL_VERSION) {
            return Err(PyRuntimeError::new_err(format!(
                "Expected DLL {} inconsistent with {}",
                DLL_VERSION, info
            )));
        }
        Ok(ObsInterface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    fn parse_line(&self, line: String) -> PyResult<PyParsedB3> {
        ParsedB3::from_line(&line)
            .map(PyParsedB3::from)
            .map_err(PyRuntimeError::new_err)
    }
}


#[pyclass(name = "ParsedB3")]
pub struct PyParsedB3 {
    inner: ParsedB3,
}

impl From<ParsedB3> for PyParsedB3 {
    fn from(value: ParsedB3) -> Self {
        PyParsedB3 { inner: value }
    }
}

#[pymethods]
impl PyParsedB3 {
    #[new]
    fn new() -> Self {
        PyParsedB3 {
            inner: ParsedB3::default(),
        }
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

    #[getter(norad_id)]
    fn get_norad_id(&self) -> PyResult<i32> {
        Ok(self.inner.norad_id)
    }

    #[setter(norad_id)]
    fn set_norad_id(&mut self, value: i32) -> PyResult<()> {
        self.inner.norad_id = value;
        Ok(())
    }

    #[getter(sensor_number)]
    fn get_sensor_number(&self) -> PyResult<i32> {
        Ok(self.inner.sensor_number)
    }

    #[setter(sensor_number)]
    fn set_sensor_number(&mut self, value: i32) -> PyResult<()> {
        self.inner.sensor_number = value;
        Ok(())
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

    #[getter(elevation)]
    fn get_elevation(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.elevation)
    }

    #[setter(elevation)]
    fn set_elevation(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.elevation = value;
        Ok(())
    }

    #[getter(declination)]
    fn get_declination(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.declination)
    }

    #[setter(declination)]
    fn set_declination(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.declination = value;
        Ok(())
    }

    #[getter(azimuth)]
    fn get_azimuth(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.azimuth)
    }

    #[setter(azimuth)]
    fn set_azimuth(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.azimuth = value;
        Ok(())
    }

    #[getter(right_ascension)]
    fn get_right_ascension(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.right_ascension)
    }

    #[setter(right_ascension)]
    fn set_right_ascension(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.right_ascension = value;
        Ok(())
    }

    #[getter(range)]
    fn get_range(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.range)
    }

    #[setter(range)]
    fn set_range(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.range = value;
        Ok(())
    }

    #[getter(range_rate)]
    fn get_range_rate(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.range_rate)
    }

    #[setter(range_rate)]
    fn set_range_rate(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.range_rate = value;
        Ok(())
    }

    #[getter(year_of_equinox)]
    fn get_year_of_equinox(&self) -> PyResult<Option<i32>> {
        Ok(self.inner.year_of_equinox)
    }

    #[setter(year_of_equinox)]
    fn set_year_of_equinox(&mut self, value: Option<i32>) -> PyResult<()> {
        self.inner.year_of_equinox = value;
        Ok(())
    }

    #[getter(elevation_rate)]
    fn get_elevation_rate(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.elevation_rate)
    }

    #[setter(elevation_rate)]
    fn set_elevation_rate(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.elevation_rate = value;
        Ok(())
    }

    #[getter(azimuth_rate)]
    fn get_azimuth_rate(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.azimuth_rate)
    }

    #[setter(azimuth_rate)]
    fn set_azimuth_rate(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.azimuth_rate = value;
        Ok(())
    }

    #[getter(range_acceleration)]
    fn get_range_acceleration(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.range_acceleration)
    }

    #[setter(range_acceleration)]
    fn set_range_acceleration(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.range_acceleration = value;
        Ok(())
    }

    #[getter(observation_type)]
    fn get_observation_type(&self) -> PyResult<i32> {
        Ok(self.inner.observation_type)
    }

    #[setter(observation_type)]
    fn set_observation_type(&mut self, value: i32) -> PyResult<()> {
        self.inner.observation_type = value;
        Ok(())
    }

    #[getter(track_position)]
    fn get_track_position(&self) -> PyResult<i32> {
        Ok(self.inner.track_position)
    }

    #[setter(track_position)]
    fn set_track_position(&mut self, value: i32) -> PyResult<()> {
        self.inner.track_position = value;
        Ok(())
    }

    #[getter(association_status)]
    fn get_association_status(&self) -> PyResult<i32> {
        Ok(self.inner.association_status)
    }

    #[setter(association_status)]
    fn set_association_status(&mut self, value: i32) -> PyResult<()> {
        self.inner.association_status = value;
        Ok(())
    }

    #[getter(site_tag)]
    fn get_site_tag(&self) -> PyResult<i32> {
        Ok(self.inner.site_tag)
    }

    #[setter(site_tag)]
    fn set_site_tag(&mut self, value: i32) -> PyResult<()> {
        self.inner.site_tag = value;
        Ok(())
    }

    #[getter(spadoc_tag)]
    fn get_spadoc_tag(&self) -> PyResult<i32> {
        Ok(self.inner.spadoc_tag)
    }

    #[setter(spadoc_tag)]
    fn set_spadoc_tag(&mut self, value: i32) -> PyResult<()> {
        self.inner.spadoc_tag = value;
        Ok(())
    }

    #[getter(position)]
    fn get_position(&self) -> PyResult<Option<[f64; 3]>> {
        Ok(self.inner.position)
    }

    #[setter(position)]
    fn set_position(&mut self, value: Option<[f64; 3]>) -> PyResult<()> {
        self.inner.position = value;
        Ok(())
    }

    fn get_line(&self) -> PyResult<String> {
        self.inner.get_line().map_err(PyRuntimeError::new_err)
    }
}

pub fn register_obs_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<ObsInterface>()?;
    parent_module.add_class::<PyParsedB3>()?;
    let class = parent_module.getattr("ObsInterface")?;
    class.setattr("EQUINOX_OBSTIME", obs::EQUINOX_OBSTIME)?;
    class.setattr("EQUINOX_OBSYEAR", obs::EQUINOX_OBSYEAR)?;
    class.setattr("EQUINOX_J2K", obs::EQUINOX_J2K)?;
    class.setattr("EQUINOX_B1950", obs::EQUINOX_B1950)?;
    class.setattr("OBSFORM_B3", obs::OBSFORM_B3)?;
    class.setattr("OBSFORM_TTY", obs::OBSFORM_TTY)?;
    class.setattr("OBSFORM_CSV", obs::OBSFORM_CSV)?;
    class.setattr("OBSFORM_RF", obs::OBSFORM_RF)?;
    class.setattr("BADOBSKEY", obs::BADOBSKEY)?;
    class.setattr("DUPOBSKEY", obs::DUPOBSKEY)?;
    class.setattr("OBS_KEYMODE_NODUP", obs::OBS_KEYMODE_NODUP)?;
    class.setattr("OBS_KEYMODE_DMA", obs::OBS_KEYMODE_DMA)?;
    Ok(())
}
