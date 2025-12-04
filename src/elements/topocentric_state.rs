use super::{HorizonState, TopocentricElements};
use crate::bodies::Observatory;
use crate::saal::astro_func_interface;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct TopocentricState {
    pub epoch: Epoch,
    pub elements: TopocentricElements,
}

impl Copy for TopocentricState {}

impl TopocentricState {
    pub fn new(epoch: Epoch, elements: TopocentricElements) -> Self {
        Self { epoch, elements }
    }

    pub fn from_horizon_state(state: &HorizonState, observer: &Observatory) -> Result<Self, String> {
        let theta = observer.get_theta(state.get_epoch());
        let sen_pos = observer.get_state_at_epoch(state.get_epoch()).position;
        let lat = observer.get_latitude();
        let teme = astro_func_interface::horizon_to_teme(theta, lat, &sen_pos.into(), &state.elements.get_xa_rae())?;

        let sat_pos = [teme[0], teme[1], teme[2]];
        let sat_vel = [teme[3], teme[4], teme[5]];
        let topo = astro_func_interface::teme_to_topocentric(theta, lat, &sen_pos.into(), &sat_pos, &sat_vel)?;

        let mut elements = TopocentricElements::new(
            topo[astro_func_interface::XA_TOPO_RA],
            topo[astro_func_interface::XA_TOPO_DEC],
        );

        elements.set_range(state.get_range());
        match state.get_range_rate() {
            Some(_) => elements.set_range_rate(Some(topo[astro_func_interface::XA_TOPO_RANGEDOT])),
            None => elements.set_range_rate(None),
        }
        match state.get_azimuth_rate() {
            Some(_) => elements.set_right_ascension_rate(Some(topo[astro_func_interface::XA_TOPO_RADOT])),
            None => elements.set_right_ascension_rate(None),
        }
        match state.get_elevation_rate() {
            Some(_) => elements.set_declination_rate(Some(topo[astro_func_interface::XA_TOPO_DECDOT])),
            None => elements.set_declination_rate(None),
        }

        Ok(Self {
            epoch: state.get_epoch(),
            elements,
        })
    }
}

#[pymethods]
impl TopocentricState {
    #[new]
    pub fn __init__(epoch: Epoch, elements: TopocentricElements) -> Self {
        Self::new(epoch, elements)
    }

    #[staticmethod]
    #[pyo3(name = "from_horizon_state")]
    pub fn py_from_horizon_state(state: &HorizonState, observer: &Observatory) -> PyResult<Self> {
        Self::from_horizon_state(state, observer).map_err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>)
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_elements(&self) -> TopocentricElements {
        self.elements
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.elements.get_range()
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.elements.get_range_rate()
    }

    #[getter]
    pub fn get_right_ascension_rate(&self) -> Option<f64> {
        self.elements.get_right_ascension_rate()
    }

    #[getter]
    pub fn get_declination_rate(&self) -> Option<f64> {
        self.elements.get_declination_rate()
    }

    #[getter]
    pub fn get_right_ascension(&self) -> f64 {
        self.elements.get_right_ascension()
    }
    #[getter]
    pub fn get_declination(&self) -> f64 {
        self.elements.get_declination()
    }
}
