use pyo3::prelude::*;
use uuid::Uuid;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    id: String,
    name: Option<String>,
    angular_noise: f64,
    range_noise: Option<f64>,
    range_rate_noise: Option<f64>,
    angular_rate_noise: Option<f64>,
    tdoa_noise: Option<f64>,
    fdoa_noise: Option<f64>,
}

#[pymethods]
impl Sensor {
    #[new]
    pub fn new(angular_noise: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            angular_noise,
            range_noise: None,
            range_rate_noise: None,
            angular_rate_noise: None,
            tdoa_noise: None,
            fdoa_noise: None,
        }
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    #[getter]
    pub fn get_angular_noise(&self) -> f64 {
        self.angular_noise
    }

    #[getter]
    pub fn get_range_noise(&self) -> Option<f64> {
        self.range_noise
    }

    #[getter]
    pub fn get_range_rate_noise(&self) -> Option<f64> {
        self.range_rate_noise
    }

    #[getter]
    pub fn get_angular_rate_noise(&self) -> Option<f64> {
        self.angular_rate_noise
    }

    #[setter]
    pub fn set_range_noise(&mut self, range_noise: f64) {
        self.range_noise = Some(range_noise);
    }

    #[setter]
    pub fn set_range_rate_noise(&mut self, range_rate_noise: f64) {
        self.range_rate_noise = Some(range_rate_noise);
    }

    #[setter]
    pub fn set_angular_rate_noise(&mut self, angular_rate_noise: f64) {
        self.angular_rate_noise = Some(angular_rate_noise);
    }

    #[getter]
    pub fn get_tdoa_noise(&self) -> Option<f64> {
        self.tdoa_noise
    }

    #[setter]
    pub fn set_tdoa_noise(&mut self, tdoa_noise: f64) {
        self.tdoa_noise = Some(tdoa_noise);
    }

    #[getter]
    pub fn get_fdoa_noise(&self) -> Option<f64> {
        self.fdoa_noise
    }

    #[setter]
    pub fn set_fdoa_noise(&mut self, fdoa_noise: f64) {
        self.fdoa_noise = Some(fdoa_noise);
    }
}
