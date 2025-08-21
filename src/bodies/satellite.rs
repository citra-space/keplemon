use crate::configs::{CONJUNCTION_STEP_MINUTES, DEFAULT_NORAD_ANALYST_ID};
use crate::elements::{CartesianState, Ephemeris, KeplerianState, TLE};
use crate::enums::{Classification, KeplerianType};
use crate::estimation::Observation;
use crate::events::CloseApproach;
use crate::propagation::{ForceProperties, InertialPropagator};
use crate::time::{Epoch, TimeSpan};
use nalgebra::{DMatrix, DVector};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use uuid::Uuid;

#[pyclass(subclass)]
#[derive(Debug, Clone, PartialEq)]
pub struct Satellite {
    id: String,
    norad_id: i32,
    name: Option<String>,
    force_properties: ForceProperties,
    keplerian_state: Option<KeplerianState>,
    inertial_propagator: Option<InertialPropagator>,
}

impl Satellite {
    pub fn get_jacobian(&self, ob: &Observation, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        match self.inertial_propagator {
            Some(ref propagator) => propagator.get_jacobian(ob, use_drag, use_srp),
            None => Err("Inertial propagator is not set".to_string()),
        }
    }

    pub fn clone_at_epoch(&self, epoch: Epoch) -> Result<Self, String> {
        let mut new_satellite = self.clone();
        match self.inertial_propagator {
            Some(ref propagator) => {
                new_satellite.inertial_propagator = Some(propagator.clone_at_epoch(epoch)?);
                new_satellite.keplerian_state = Some(propagator.get_keplerian_state_at_epoch(epoch).unwrap());
            }
            None => return Err("Inertial propagator is not set".to_string()),
        };

        Ok(new_satellite)
    }

    pub fn get_prior_node(&self, epoch: Epoch) -> Result<Epoch, String> {
        match self.inertial_propagator {
            Some(ref propagator) => propagator.get_prior_node(epoch),
            None => Err("Inertial propagator is not set".to_string()),
        }
    }

    pub fn new_with_delta_x(&self, delta_x: &DVector<f64>, use_drag: bool, use_srp: bool) -> Result<Self, String> {
        let mut new_satellite = self.clone();
        match self.inertial_propagator {
            Some(ref propagator) => {
                new_satellite.inertial_propagator = Some(propagator.new_with_delta_x(delta_x, use_drag, use_srp)?);
                new_satellite.keplerian_state = Some(propagator.get_keplerian_state().unwrap());
                new_satellite.force_properties = propagator.get_force_properties().unwrap();
            }
            None => return Err("Inertial propagator is not set".to_string()),
        };

        Ok(new_satellite)
    }
}

impl Default for Satellite {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl Satellite {
    #[new]
    pub fn new() -> Self {
        Self {
            norad_id: DEFAULT_NORAD_ANALYST_ID,
            id: Uuid::new_v4().to_string(),
            name: None,
            force_properties: ForceProperties::default(),
            keplerian_state: None,
            inertial_propagator: None,
        }
    }

    pub fn to_tle(&self) -> PyResult<Option<TLE>> {
        match self.get_keplerian_state() {
            Some(state) => match TLE::new(
                self.id.clone(),
                self.norad_id,
                self.name.clone(),
                Classification::Unclassified,
                "".to_string(),
                state,
                self.force_properties,
            ) {
                Ok(tle) => Ok(Some(tle)),
                Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
            },
            None => Ok(None),
        }
    }

    #[staticmethod]
    pub fn from_tle(tle: TLE) -> Self {
        Self {
            id: tle.get_id(),
            norad_id: tle.get_norad_id(),
            name: tle.get_name(),
            force_properties: tle.get_force_properties(),
            keplerian_state: Some(tle.get_keplerian_state()),
            inertial_propagator: Some(InertialPropagator::from_tle(tle)),
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

    #[setter]
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    #[getter]
    pub fn get_periapsis(&self) -> Option<f64> {
        self.keplerian_state.as_ref().map(|state| state.get_periapsis())
    }

    #[getter]
    pub fn get_apoapsis(&self) -> Option<f64> {
        self.keplerian_state.as_ref().map(|state| state.get_apoapsis())
    }

    #[setter]
    pub fn set_id(&mut self, satellite_id: String) {
        self.id = satellite_id;
    }

    #[getter]
    pub fn get_norad_id(&self) -> i32 {
        self.norad_id
    }

    #[setter]
    pub fn set_norad_id(&mut self, norad_id: i32) {
        self.norad_id = norad_id;
    }

    pub fn get_state_at_epoch(&self, epoch: Epoch) -> Option<CartesianState> {
        self.inertial_propagator
            .as_ref()
            .map(|propagator| propagator.get_cartesian_state_at_epoch(epoch))?
    }

    #[setter]
    pub fn set_keplerian_state(&mut self, keplerian_state: KeplerianState) -> PyResult<()> {
        self.keplerian_state = Some(keplerian_state);
        match keplerian_state.get_type() {
            KeplerianType::Osculating => Err(PyErr::new::<PyValueError, _>(
                "Osculating elements not implemented".to_string(),
            )),
            _ => {
                let tle = TLE::new(
                    self.id.clone(),
                    self.norad_id,
                    self.name.clone(),
                    Classification::Unclassified,
                    "".to_string(),
                    keplerian_state,
                    self.force_properties,
                )
                .unwrap();
                self.inertial_propagator = Some(InertialPropagator::from_tle(tle));
                Ok(())
            }
        }
    }

    #[setter]
    pub fn set_force_properties(&mut self, force_properties: ForceProperties) {
        self.force_properties = force_properties;
        if let Some(state) = self.get_keplerian_state() {
            if state.get_type() != KeplerianType::Osculating {
                let tle = TLE::new(
                    self.id.clone(),
                    self.norad_id,
                    self.name.clone(),
                    Classification::Unclassified,
                    "".to_string(),
                    state,
                    force_properties,
                )
                .unwrap();
                self.inertial_propagator = Some(InertialPropagator::from_tle(tle));
            }
        }
    }

    #[getter]
    pub fn get_force_properties(&self) -> ForceProperties {
        self.force_properties
    }

    pub fn get_ephemeris(&self, start_epoch: Epoch, end_epoch: Epoch, step: TimeSpan) -> Option<Ephemeris> {
        match self.get_state_at_epoch(start_epoch) {
            Some(state) => {
                let ephemeris = Ephemeris::new(self.id.clone(), state);
                let mut next_epoch: Epoch = start_epoch + step;
                while next_epoch <= end_epoch {
                    match self.get_state_at_epoch(next_epoch) {
                        Some(state) => {
                            ephemeris.add_state(state);
                            next_epoch += step;
                        }
                        None => {
                            return None;
                        }
                    }
                }
                Some(ephemeris)
            }
            None => None,
        }
    }

    #[getter]
    pub fn get_keplerian_state(&self) -> Option<KeplerianState> {
        self.keplerian_state
    }

    pub fn get_close_approach(
        &self,
        other: &Satellite,
        start_epoch: Epoch,
        end_epoch: Epoch,
        distance_threshold: f64,
    ) -> Option<CloseApproach> {
        if (self.keplerian_state.is_none() || other.keplerian_state.is_none())
            || self.get_apoapsis()? < other.get_periapsis()? - distance_threshold
            || other.get_apoapsis()? < self.get_periapsis()? - distance_threshold
            || self.get_periapsis()? > other.get_apoapsis()? + distance_threshold
            || other.get_periapsis()? > self.get_apoapsis()? + distance_threshold
        {
            return None;
        }

        match self.get_ephemeris(start_epoch, end_epoch, TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES)) {
            Some(ephemeris) => {
                match other.get_ephemeris(start_epoch, end_epoch, TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES)) {
                    Some(other_ephemeris) => ephemeris.get_close_approach(&other_ephemeris, distance_threshold),
                    None => None,
                }
            }
            None => None,
        }
    }
}
