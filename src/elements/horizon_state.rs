use super::{CartesianState, HorizonElements, TopocentricState};
use crate::bodies::Observatory;
use crate::saal::astro_func_interface;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct HorizonState {
    pub epoch: Epoch,
    pub elements: HorizonElements,
}

impl Copy for HorizonState {}

impl HorizonState {
    pub fn new(epoch: Epoch, elements: HorizonElements) -> Self {
        Self { epoch, elements }
    }

    pub fn from_topocentric_state(state: &TopocentricState, observer: &Observatory) -> Result<Self, String> {
        let az_el = astro_func_interface::ra_dec_to_az_el_time(
            state.get_epoch().days_since_1950,
            observer.get_latitude(),
            observer.get_longitude(),
            state.get_right_ascension(),
            state.get_declination(),
        );
        let mut elements = HorizonElements::new(az_el[0], az_el[1]);
        elements.set_range(state.get_range());

        Ok(Self {
            epoch: state.get_epoch(),
            elements,
        })
    }
}

#[pymethods]
impl HorizonState {
    #[new]
    pub fn py_new(epoch: Epoch, elements: HorizonElements) -> Self {
        Self::new(epoch, elements)
    }

    #[staticmethod]
    #[pyo3(name = "from_topocentric_state")]
    pub fn py_from_topocentric_state(state: &TopocentricState, observer: &Observatory) -> PyResult<Self> {
        Self::from_topocentric_state(state, observer).map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
    }

    #[staticmethod]
    pub fn from_teme_states(sensor_teme: CartesianState, target_teme: CartesianState) -> Self {
        let theta_g = sensor_teme.get_epoch().to_fk5_greenwich_angle();
        let lla = astro_func_interface::theta_teme_to_lla(theta_g, &sensor_teme.position.into());
        let topo = astro_func_interface::teme_to_topocentric(
            theta_g + lla[1].to_radians(),
            lla[0],
            &sensor_teme.position.into(),
            &target_teme.position.into(),
            &target_teme.velocity.into(),
        )
        .unwrap();
        let elements = HorizonElements {
            azimuth: topo[astro_func_interface::XA_TOPO_AZ],
            elevation: topo[astro_func_interface::XA_TOPO_EL],
            range: Some(topo[astro_func_interface::XA_TOPO_RANGE]),
            range_rate: Some(topo[astro_func_interface::XA_TOPO_RANGEDOT]),
            azimuth_rate: Some(topo[astro_func_interface::XA_TOPO_AZDOT]),
            elevation_rate: Some(topo[astro_func_interface::XA_TOPO_ELDOT]),
        };
        Self {
            epoch: sensor_teme.get_epoch(),
            elements,
        }
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_elements(&self) -> HorizonElements {
        self.elements
    }

    #[getter]
    pub fn get_azimuth(&self) -> f64 {
        self.elements.azimuth
    }

    #[getter]
    pub fn get_elevation(&self) -> f64 {
        self.elements.elevation
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.elements.range
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.elements.range_rate
    }

    #[getter]
    pub fn get_azimuth_rate(&self) -> Option<f64> {
        self.elements.azimuth_rate
    }

    #[getter]
    pub fn get_elevation_rate(&self) -> Option<f64> {
        self.elements.elevation_rate
    }

    #[setter]
    pub fn set_elements(&mut self, elements: HorizonElements) {
        self.elements = elements;
    }

    #[setter]
    pub fn set_epoch(&mut self, epoch: Epoch) {
        self.epoch = epoch;
    }

    #[setter]
    pub fn set_azimuth(&mut self, azimuth: f64) {
        self.elements.azimuth = azimuth;
    }

    #[setter]
    pub fn set_elevation(&mut self, elevation: f64) {
        self.elements.elevation = elevation;
    }

    #[setter]
    pub fn set_range(&mut self, range: Option<f64>) {
        self.elements.range = range;
    }

    #[setter]
    pub fn set_range_rate(&mut self, range_rate: Option<f64>) {
        self.elements.range_rate = range_rate;
    }

    #[setter]
    pub fn set_azimuth_rate(&mut self, azimuth_rate: Option<f64>) {
        self.elements.azimuth_rate = azimuth_rate;
    }

    #[setter]
    pub fn set_elevation_rate(&mut self, elevation_rate: Option<f64>) {
        self.elements.elevation_rate = elevation_rate;
    }
}
