use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct BoreToBodyAngles {
    pub earth_angle: f64,
    pub sun_angle: f64,
    pub moon_angle: f64,
}

#[pymethods]
impl BoreToBodyAngles {
    #[new]
    pub fn new(earth_angle: f64, sun_angle: f64, moon_angle: f64) -> Self {
        Self {
            earth_angle,
            sun_angle,
            moon_angle,
        }
    }

    #[getter]
    pub fn get_earth_angle(&self) -> f64 {
        self.earth_angle
    }

    #[getter]
    pub fn get_sun_angle(&self) -> f64 {
        self.sun_angle
    }

    #[getter]
    pub fn get_moon_angle(&self) -> f64 {
        self.moon_angle
    }
}
