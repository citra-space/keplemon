use super::CartesianVector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphericalVector {
    pub range: f64,
    pub right_ascension: f64,
    pub declination: f64,
}

impl SphericalVector {
    pub fn new(range: f64, right_ascension: f64, declination: f64) -> Self {
        Self {
            range,
            right_ascension,
            declination,
        }
    }
}

impl From<SphericalVector> for CartesianVector {
    fn from(sph: SphericalVector) -> Self {
        let right_ascension_rad = sph.right_ascension.to_radians();
        let declination_rad = sph.declination.to_radians();
        let cos_dec = declination_rad.cos();
        let x = sph.range * right_ascension_rad.cos() * cos_dec;
        let y = sph.range * right_ascension_rad.sin() * cos_dec;
        let z = sph.range * declination_rad.sin();

        CartesianVector::new(x, y, z)
    }
}
