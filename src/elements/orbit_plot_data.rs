use super::{CartesianState, KeplerianState};
use crate::time::Epoch;
use saal::astro;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitPlotState {
    epoch: Epoch,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    semi_major_axis: f64,
    eccentricity: f64,
    inclination: f64,
    raan: f64,
    radius: f64,
    apogee_radius: f64,
    perigee_radius: f64,
}

impl From<&KeplerianState> for OrbitPlotState {
    fn from(keplerian_state: &KeplerianState) -> Self {
        let eci: CartesianState = keplerian_state.into();
        let lla = astro::time_teme_to_lla(keplerian_state.epoch.days_since_1950, &eci.position.into());
        Self {
            epoch: keplerian_state.epoch,
            latitude: lla[0],
            longitude: lla[1],
            altitude: lla[2],
            semi_major_axis: keplerian_state.get_semi_major_axis(),
            eccentricity: keplerian_state.get_eccentricity(),
            inclination: keplerian_state.get_inclination(),
            raan: keplerian_state.get_raan(),
            radius: eci.position.get_magnitude(),
            apogee_radius: keplerian_state.get_apoapsis(),
            perigee_radius: keplerian_state.get_periapsis(),
        }
    }
}

impl From<CartesianState> for OrbitPlotState {
    fn from(cartesian_state: CartesianState) -> Self {
        let keplerian_state: KeplerianState = cartesian_state.into();
        Self::from(&keplerian_state)
    }
}

impl OrbitPlotState {
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn get_altitude(&self) -> f64 {
        self.altitude
    }

    pub fn get_semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    pub fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    pub fn get_inclination(&self) -> f64 {
        self.inclination
    }

    pub fn get_raan(&self) -> f64 {
        self.raan
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_apogee_radius(&self) -> f64 {
        self.apogee_radius
    }

    pub fn get_perigee_radius(&self) -> f64 {
        self.perigee_radius
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct OrbitPlotData {
    satellite_id: String,
    epochs: Vec<String>,
    latitudes: Vec<f64>,
    longitudes: Vec<f64>,
    altitudes: Vec<f64>,
    semi_major_axes: Vec<f64>,
    eccentricities: Vec<f64>,
    inclinations: Vec<f64>,
    raans: Vec<f64>,
    radii: Vec<f64>,
    apogee_radii: Vec<f64>,
    perigee_radii: Vec<f64>,
}

impl OrbitPlotData {
    pub fn new(satellite_id: String) -> Self {
        Self {
            satellite_id,
            epochs: Vec::new(),
            latitudes: Vec::new(),
            longitudes: Vec::new(),
            altitudes: Vec::new(),
            semi_major_axes: Vec::new(),
            eccentricities: Vec::new(),
            inclinations: Vec::new(),
            raans: Vec::new(),
            radii: Vec::new(),
            apogee_radii: Vec::new(),
            perigee_radii: Vec::new(),
        }
    }

    pub fn add_state(&mut self, plot_state: OrbitPlotState) {
        self.epochs.push(plot_state.get_epoch().to_iso());
        self.latitudes.push(plot_state.get_latitude());
        self.longitudes.push(plot_state.get_longitude());
        self.altitudes.push(plot_state.get_altitude());
        self.semi_major_axes.push(plot_state.get_semi_major_axis());
        self.eccentricities.push(plot_state.get_eccentricity());
        self.inclinations.push(plot_state.get_inclination());
        self.raans.push(plot_state.get_raan());
        self.radii.push(plot_state.get_radius());
        self.apogee_radii.push(plot_state.get_apogee_radius());
        self.perigee_radii.push(plot_state.get_perigee_radius());
    }

    pub fn get_satellite_id(&self) -> String {
        self.satellite_id.clone()
    }

    pub fn get_epochs(&self) -> Vec<String> {
        self.epochs.clone()
    }

    pub fn get_latitudes(&self) -> Vec<f64> {
        self.latitudes.clone()
    }

    pub fn get_longitudes(&self) -> Vec<f64> {
        self.longitudes.clone()
    }

    pub fn get_altitudes(&self) -> Vec<f64> {
        self.altitudes.clone()
    }

    pub fn get_semi_major_axes(&self) -> Vec<f64> {
        self.semi_major_axes.clone()
    }

    pub fn get_eccentricities(&self) -> Vec<f64> {
        self.eccentricities.clone()
    }

    pub fn get_inclinations(&self) -> Vec<f64> {
        self.inclinations.clone()
    }

    pub fn get_raans(&self) -> Vec<f64> {
        self.raans.clone()
    }

    pub fn get_radii(&self) -> Vec<f64> {
        self.radii.clone()
    }

    pub fn get_apogee_radii(&self) -> Vec<f64> {
        self.apogee_radii.clone()
    }

    pub fn get_perigee_radii(&self) -> Vec<f64> {
        self.perigee_radii.clone()
    }
}
