use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::sensor::{self, ParsedSensor, XA_SEN_SIZE};
use crate::DLL_VERSION;

#[pyclass]
pub struct SensorInterface {
    info: String,
}

#[pymethods]
impl SensorInterface {
    #[new]
    fn new() -> PyResult<Self> {
        let info = sensor::get_dll_info();
        if !info.contains(DLL_VERSION) {
            return Err(PyRuntimeError::new_err(format!(
                "Expected DLL {} inconsistent with {}",
                DLL_VERSION, info
            )));
        }
        Ok(SensorInterface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    fn parse_key(&self, sen_key: i64) -> PyResult<PyParsedSensor> {
        sensor::parse_key(sen_key)
            .map(PyParsedSensor::from)
            .map_err(PyRuntimeError::new_err)
    }

    fn parse_all(&self) -> PyResult<Vec<PyParsedSensor>> {
        let sensors = sensor::parse_all().map_err(PyRuntimeError::new_err)?;
        Ok(sensors.into_iter().map(PyParsedSensor::from).collect())
    }

    fn prune_missing_locations(&self) -> PyResult<()> {
        sensor::prune_missing_locations().map_err(PyRuntimeError::new_err)
    }

    fn get_astronomical_ll(&self, sen_key: i64) -> PyResult<[f64; 2]> {
        sensor::get_astronomical_ll(sen_key).map_err(PyRuntimeError::new_err)
    }

    fn get_lla(&self, sen_key: i64) -> PyResult<Option<[f64; 3]>> {
        sensor::get_lla(sen_key).map_err(PyRuntimeError::new_err)
    }

    fn get_keys(&self, order: i32) -> PyResult<Vec<i64>> {
        Ok(sensor::get_keys(order))
    }

    fn load_card(&self, card: String) -> PyResult<()> {
        sensor::load_card(&card).map_err(PyRuntimeError::new_err)
    }

    fn remove(&self, sen_key: i64) -> PyResult<()> {
        sensor::remove(sen_key).map_err(PyRuntimeError::new_err)
    }

    fn get_count(&self) -> PyResult<i32> {
        Ok(sensor::count_loaded())
    }

    fn load_file(&self, file_path: String) -> PyResult<()> {
        sensor::load_file(&file_path).map_err(PyRuntimeError::new_err)
    }

    fn clear(&self) -> PyResult<()> {
        sensor::clear().map_err(PyRuntimeError::new_err)
    }

    fn get_arrays(&self, sen_key: i64) -> PyResult<([f64; XA_SEN_SIZE], String)> {
        sensor::get_arrays(sen_key).map_err(PyRuntimeError::new_err)
    }
}

#[pyclass(name = "ParsedSensor")]
pub struct PyParsedSensor {
    inner: ParsedSensor,
}

impl From<ParsedSensor> for PyParsedSensor {
    fn from(value: ParsedSensor) -> Self {
        PyParsedSensor { inner: value }
    }
}

#[pymethods]
impl PyParsedSensor {
    #[getter(key)]
    fn get_key(&self) -> PyResult<i64> {
        Ok(self.inner.key)
    }

    #[getter(number)]
    fn get_number(&self) -> PyResult<i32> {
        Ok(self.inner.number)
    }

    #[getter(minimum_range)]
    fn get_minimum_range(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.minimum_range)
    }

    #[getter(maximum_range)]
    fn get_maximum_range(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.maximum_range)
    }

    #[getter(range_rate_limit)]
    fn get_range_rate_limit(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.range_rate_limit)
    }

    #[getter(apply_range_limits)]
    fn get_apply_range_limits(&self) -> PyResult<bool> {
        Ok(self.inner.apply_range_limits)
    }

    #[getter(mobile)]
    fn get_mobile(&self) -> PyResult<bool> {
        Ok(self.inner.mobile)
    }

    #[getter(latitude)]
    fn get_latitude(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.latitude)
    }

    #[getter(longitude)]
    fn get_longitude(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.longitude)
    }

    #[getter(altitude)]
    fn get_altitude(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.altitude)
    }

    #[getter(astronomical_latitude)]
    fn get_astronomical_latitude(&self) -> PyResult<f64> {
        Ok(self.inner.astronomical_latitude)
    }

    #[getter(astronomical_longitude)]
    fn get_astronomical_longitude(&self) -> PyResult<f64> {
        Ok(self.inner.astronomical_longitude)
    }

    #[getter(azimuth_noise)]
    fn get_azimuth_noise(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.azimuth_noise)
    }

    #[getter(elevation_noise)]
    fn get_elevation_noise(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.elevation_noise)
    }

    #[getter(range_noise)]
    fn get_range_noise(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.range_noise)
    }

    #[getter(range_rate_noise)]
    fn get_range_rate_noise(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.range_rate_noise)
    }

    #[getter(azimuth_rate_noise)]
    fn get_azimuth_rate_noise(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.azimuth_rate_noise)
    }

    #[getter(elevation_rate_noise)]
    fn get_elevation_rate_noise(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.elevation_rate_noise)
    }

    #[getter(description)]
    fn get_description(&self) -> PyResult<Option<String>> {
        Ok(self.inner.description.clone())
    }
}

pub fn register_sensor_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<SensorInterface>()?;
    parent_module.add_class::<PyParsedSensor>()?;
    let class = parent_module.getattr("SensorInterface")?;
    class.setattr("SEN_KEYMODE_NODUP", sensor::SEN_KEYMODE_NODUP)?;
    class.setattr("SEN_KEYMODE_DMA", sensor::SEN_KEYMODE_DMA)?;
    class.setattr("BADSENKEY", sensor::BADSENKEY)?;
    class.setattr("DUPSENKEY", sensor::DUPSENKEY)?;
    class.setattr("SENLOC_TYPE_ECR", sensor::SENLOC_TYPE_ECR)?;
    class.setattr("SENLOC_TYPE_EFG", sensor::SENLOC_TYPE_EFG)?;
    class.setattr("SENLOC_TYPE_LLH", sensor::SENLOC_TYPE_LLH)?;
    class.setattr("SENLOC_TYPE_ECI", sensor::SENLOC_TYPE_ECI)?;
    Ok(())
}
