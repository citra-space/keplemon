#[derive(Debug, Clone, PartialEq)]
pub struct BoreToBodyAngles {
    earth_angle: f64,
    sun_angle: f64,
    moon_angle: f64,
}
impl BoreToBodyAngles {
    pub fn new(earth_angle: f64, sun_angle: f64, moon_angle: f64) -> Self {
        Self {
            earth_angle,
            sun_angle,
            moon_angle,
        }
    }

    pub fn get_earth_angle(&self) -> f64 {
        self.earth_angle
    }

    pub fn get_sun_angle(&self) -> f64 {
        self.sun_angle
    }

    pub fn get_moon_angle(&self) -> f64 {
        self.moon_angle
    }
}
