use super::{CartesianState, KeplerianState};
use crate::saal::astro_func_interface;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
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

impl OrbitPlotState {
    pub fn from_keplerian_state(keplerian_state: &KeplerianState) -> Self {
        let eci = keplerian_state.to_cartesian().position;
        let lla = astro_func_interface::time_teme_to_lla(keplerian_state.get_epoch().days_since_1950, &eci.into());
        Self {
            epoch: keplerian_state.get_epoch(),
            latitude: lla[0],
            longitude: lla[1],
            altitude: lla[2],
            semi_major_axis: keplerian_state.get_semi_major_axis(),
            eccentricity: keplerian_state.get_eccentricity(),
            inclination: keplerian_state.get_inclination(),
            raan: keplerian_state.get_raan(),
            radius: eci.get_magnitude(),
            apogee_radius: keplerian_state.get_apoapsis(),
            perigee_radius: keplerian_state.get_periapsis(),
        }
    }

    pub fn from_cartesian_state(cartesian_state: &CartesianState) -> Self {
        let keplerian_state = cartesian_state.to_keplerian();
        Self::from_keplerian_state(&keplerian_state)
    }
}

#[pymethods]
impl OrbitPlotState {
    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    #[getter]
    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    #[getter]
    pub fn get_altitude(&self) -> f64 {
        self.altitude
    }

    #[getter]
    pub fn get_semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    #[getter]
    pub fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    #[getter]
    pub fn get_inclination(&self) -> f64 {
        self.inclination
    }

    #[getter]
    pub fn get_raan(&self) -> f64 {
        self.raan
    }

    #[getter]
    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    #[getter]
    pub fn get_apogee_radius(&self) -> f64 {
        self.apogee_radius
    }

    #[getter]
    pub fn get_perigee_radius(&self) -> f64 {
        self.perigee_radius
    }
}

#[pyclass]
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
}

#[pymethods]
impl OrbitPlotData {
    #[getter]
    pub fn get_satellite_id(&self) -> String {
        self.satellite_id.clone()
    }

    #[getter]
    pub fn get_epochs(&self) -> Vec<String> {
        self.epochs.clone()
    }

    #[getter]
    pub fn get_latitudes(&self) -> Vec<f64> {
        self.latitudes.clone()
    }

    #[getter]
    pub fn get_longitudes(&self) -> Vec<f64> {
        self.longitudes.clone()
    }

    #[getter]
    pub fn get_altitudes(&self) -> Vec<f64> {
        self.altitudes.clone()
    }

    #[getter]
    pub fn get_semi_major_axes(&self) -> Vec<f64> {
        self.semi_major_axes.clone()
    }

    #[getter]
    pub fn get_eccentricities(&self) -> Vec<f64> {
        self.eccentricities.clone()
    }

    #[getter]
    pub fn get_inclinations(&self) -> Vec<f64> {
        self.inclinations.clone()
    }

    #[getter]
    pub fn get_raans(&self) -> Vec<f64> {
        self.raans.clone()
    }

    #[getter]
    pub fn get_radii(&self) -> Vec<f64> {
        self.radii.clone()
    }

    #[getter]
    pub fn get_apogee_radii(&self) -> Vec<f64> {
        self.apogee_radii.clone()
    }

    #[getter]
    pub fn get_perigee_radii(&self) -> Vec<f64> {
        self.perigee_radii.clone()
    }
}
