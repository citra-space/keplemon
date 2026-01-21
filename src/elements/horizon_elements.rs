use saal::astro;

#[derive(Debug, Clone, PartialEq)]
pub struct HorizonElements {
    pub range: Option<f64>,
    pub range_rate: Option<f64>,
    pub azimuth: f64,
    pub elevation: f64,
    pub azimuth_rate: Option<f64>,
    pub elevation_rate: Option<f64>,
}
impl Copy for HorizonElements {}

impl HorizonElements {
    pub fn new(azimuth: f64, elevation: f64) -> Self {
        Self {
            range: None,
            range_rate: None,
            azimuth,
            elevation,
            azimuth_rate: None,
            elevation_rate: None,
        }
    }
}

impl From<HorizonElements> for [f64; astro::XA_RAE_SIZE] {
    fn from(horizon: HorizonElements) -> Self {
        let mut xa_rae = [0.0; astro::XA_RAE_SIZE];
        xa_rae[astro::XA_RAE_AZ] = horizon.azimuth;
        xa_rae[astro::XA_RAE_EL] = horizon.elevation;
        match horizon.range {
            Some(r) => xa_rae[astro::XA_RAE_RANGE] = r,
            None => xa_rae[astro::XA_RAE_RANGE] = 1.0,
        }
        match horizon.range_rate {
            Some(rr) => xa_rae[astro::XA_RAE_RANGEDOT] = rr,
            None => xa_rae[astro::XA_RAE_RANGEDOT] = 0.0,
        }
        match horizon.azimuth_rate {
            Some(az) => xa_rae[astro::XA_RAE_AZDOT] = az,
            None => xa_rae[astro::XA_RAE_AZDOT] = 0.0,
        }
        match horizon.elevation_rate {
            Some(el) => xa_rae[astro::XA_RAE_ELDOT] = el,
            None => xa_rae[astro::XA_RAE_ELDOT] = 0.0,
        }
        xa_rae
    }
}
