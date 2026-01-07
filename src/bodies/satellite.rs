use crate::bodies::Observatory;
use crate::configs::{CONJUNCTION_STEP_MINUTES, DEFAULT_NORAD_ANALYST_ID, MIN_EPHEMERIS_POINTS};
use crate::elements::{
    BoreToBodyAngles, CartesianState, CartesianVector, Ephemeris, GeodeticPosition, KeplerianState, OrbitPlotData,
    OrbitPlotState, RelativeState, TLE, construct_ephemeris_id,
};
use crate::enums::{Classification, KeplerianType, ReferenceFrame};
use crate::estimation::Observation;
use crate::events::{CloseApproach, HorizonAccessReport};
use crate::propagation::{ForceProperties, InertialPropagator};
use crate::time::{Epoch, TimeSpan};
use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;
use saal::{astro, satellite};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Satellite {
    pub id: String,
    pub norad_id: i32,
    pub name: Option<String>,
    force_properties: ForceProperties,
    keplerian_state: Option<KeplerianState>,
    inertial_propagator: Option<InertialPropagator>,
    ephemeris_cache: Option<Ephemeris>,
    pub ephemeris_id: Option<String>,
}

impl Default for Satellite {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Satellite> for TLE {
    fn from(satellite: Satellite) -> TLE {
        let state = satellite.get_keplerian_state().unwrap();
        TLE::new(
            satellite.id.clone(),
            satellite.norad_id,
            satellite.name.clone(),
            Classification::Unclassified,
            "".to_string(),
            state,
            satellite.force_properties,
        )
        .unwrap()
    }
}

impl From<TLE> for Satellite {
    fn from(tle: TLE) -> Self {
        Self {
            id: tle.satellite_id.clone(),
            norad_id: tle.norad_id,
            name: tle.get_name(),
            force_properties: tle.get_force_properties(),
            keplerian_state: Some(tle.get_keplerian_state()),
            inertial_propagator: Some(InertialPropagator::from(tle)),
            ephemeris_cache: None,
            ephemeris_id: None,
        }
    }
}

impl Satellite {
    pub fn get_jacobian(&self, ob: &Observation, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        match self.inertial_propagator {
            Some(ref propagator) => propagator.get_jacobian(ob, use_drag, use_srp),
            None => Err("Inertial propagator is not set".to_string()),
        }
    }

    pub fn get_jacobian_with_ref(
        &self,
        ob: &Observation,
        use_drag: bool,
        use_srp: bool,
        h_ref: &[f64],
    ) -> Result<DMatrix<f64>, String> {
        match self.inertial_propagator {
            Some(ref propagator) => propagator.get_jacobian_with_ref(ob, use_drag, use_srp, h_ref),
            None => Err("Inertial propagator is not set".to_string()),
        }
    }

    pub fn build_perturbed_satellites(&self, use_drag: bool, use_srp: bool) -> Result<Vec<(Satellite, f64)>, String> {
        match self.inertial_propagator {
            Some(ref propagator) => propagator.build_perturbed_satellites(use_drag, use_srp),
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
                // Get keplerian state and force properties from the new propagator
                new_satellite.keplerian_state = Some(
                    new_satellite
                        .inertial_propagator
                        .as_ref()
                        .unwrap()
                        .get_keplerian_state()
                        .unwrap(),
                );
                new_satellite.force_properties = new_satellite
                    .inertial_propagator
                    .as_ref()
                    .unwrap()
                    .get_force_properties()
                    .unwrap();
            }
            None => return Err("Inertial propagator is not set".to_string()),
        };

        Ok(new_satellite)
    }

    pub fn step_to_epoch(&mut self, epoch: Epoch) -> Result<(), String> {
        match self.inertial_propagator {
            Some(ref mut propagator) => {
                propagator.step_to_epoch(epoch)?;
                self.keplerian_state = Some(propagator.get_keplerian_state().unwrap());
                Ok(())
            }
            None => Err("Inertial propagator is not set".to_string()),
        }
    }

    pub fn get_ephemeris(&mut self, start_epoch: Epoch, end_epoch: Epoch, step: TimeSpan) -> Option<Ephemeris> {
        // exit early if we have a cached ephemeris that matches the request
        if self.ephemeris_id == Some(construct_ephemeris_id(start_epoch, end_epoch, step)) {
            return self.ephemeris_cache.clone();
        }

        match self.get_state_at_epoch(start_epoch) {
            Some(state) => {
                let ephemeris = Ephemeris::new(self.id.clone(), Some(self.norad_id), state).unwrap();
                let diff = end_epoch - start_epoch;
                let max_step = TimeSpan::from_minutes(diff.in_minutes() / MIN_EPHEMERIS_POINTS as f64);
                let dt = if step < max_step { step } else { max_step };
                let mut next_epoch: Epoch = start_epoch + dt;
                while next_epoch <= end_epoch {
                    match self.get_state_at_epoch(next_epoch) {
                        Some(state) => {
                            ephemeris.add_state(state).unwrap();
                            next_epoch += dt;
                        }
                        None => return None,
                    }
                }
                self.ephemeris_cache = Some(ephemeris.clone());
                self.ephemeris_id = Some(construct_ephemeris_id(start_epoch, end_epoch, step));
                Some(ephemeris)
            }
            None => None,
        }
    }

    pub fn new() -> Self {
        Self {
            norad_id: DEFAULT_NORAD_ANALYST_ID,
            id: Uuid::new_v4().to_string(),
            name: None,
            force_properties: ForceProperties::default(),
            keplerian_state: None,
            inertial_propagator: None,
            ephemeris_cache: None,
            ephemeris_id: None,
        }
    }

    pub fn get_relative_state_at_epoch(&self, origin: &Satellite, epoch: Epoch) -> Option<RelativeState> {
        let state_1 = self.get_state_at_epoch(epoch)?;
        let state_2 = origin.get_state_at_epoch(epoch)?;

        let teme_1 = [
            state_1.position[0],
            state_1.position[1],
            state_1.position[2],
            state_1.velocity[0],
            state_1.velocity[1],
            state_1.velocity[2],
        ];
        let teme_2 = [
            state_2.position[0],
            state_2.position[1],
            state_2.position[2],
            state_2.velocity[0],
            state_2.velocity[1],
            state_2.velocity[2],
        ];
        let xa_delta = satellite::get_relative_array(&teme_2, &teme_1, epoch.days_since_1950, 1);
        let pos = [
            xa_delta[satellite::XA_DELTA_PRADIAL],
            xa_delta[satellite::XA_DELTA_PINTRCK],
            xa_delta[satellite::XA_DELTA_PCRSSTRCK],
        ];
        let vel = [
            xa_delta[satellite::XA_DELTA_VRADIAL],
            xa_delta[satellite::XA_DELTA_VINTRCK],
            xa_delta[satellite::XA_DELTA_VCRSSTRCK],
        ];
        Some(RelativeState {
            epoch,
            position: CartesianVector::from(pos),
            velocity: CartesianVector::from(vel),
            origin_satellite_id: origin.id.clone(),
            secondary_satellite_id: self.id.clone(),
        })
    }

    pub fn get_body_angles_at_epoch(&self, other: &Satellite, epoch: Epoch) -> Option<BoreToBodyAngles> {
        let self_state = self.get_state_at_epoch(epoch)?;
        let other_state = other.get_state_at_epoch(epoch)?;
        let self_to_other = other_state.position - self_state.position;
        let self_to_earth = self_state.position * -1.0;
        let (sun, moon) = astro::get_jpl_sun_and_moon_position(epoch.days_since_1950);
        let self_to_sun = CartesianVector::from(sun) - self_state.position;
        let self_to_moon = CartesianVector::from(moon) - self_state.position;
        let sun_angle = self_to_other.angle(&self_to_sun);
        let moon_angle = self_to_other.angle(&self_to_moon);
        let earth_angle = self_to_other.angle(&self_to_earth);
        Some(BoreToBodyAngles::new(
            earth_angle.to_degrees(),
            sun_angle.to_degrees(),
            moon_angle.to_degrees(),
        ))
    }

    pub fn get_geodetic_position(&self) -> Option<GeodeticPosition> {
        match self.keplerian_state {
            Some(ref state) => {
                let teme: CartesianState = state.into();
                let teme = teme.to_frame(ReferenceFrame::TEME).position;
                let lla = astro::time_teme_to_lla(state.epoch.days_since_1950, &teme.into());
                Some(GeodeticPosition::new(lla[0], lla[1], lla[2]))
            }
            None => None,
        }
    }

    pub fn get_periapsis(&self) -> Option<f64> {
        self.keplerian_state.as_ref().map(|state| state.get_periapsis())
    }

    pub fn get_apoapsis(&self) -> Option<f64> {
        self.keplerian_state.as_ref().map(|state| state.get_apoapsis())
    }

    pub fn get_state_at_epoch(&self, epoch: Epoch) -> Option<CartesianState> {
        self.inertial_propagator
            .as_ref()
            .map(|propagator| propagator.get_cartesian_state_at_epoch(epoch))?
    }

    pub fn set_keplerian_state(&mut self, keplerian_state: KeplerianState) -> Result<(), String> {
        self.keplerian_state = Some(keplerian_state);
        match keplerian_state.get_type() {
            KeplerianType::Osculating => Err("Cannot set osculating elements directly; use TLE instead".to_string()),
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
                self.inertial_propagator = Some(InertialPropagator::from(tle));
                Ok(())
            }
        }
    }

    pub fn set_force_properties(&mut self, force_properties: ForceProperties) {
        self.force_properties = force_properties;
        if let Some(state) = self.get_keplerian_state()
            && state.get_type() != KeplerianType::Osculating
        {
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
            self.inertial_propagator = Some(InertialPropagator::from(tle));
        }
    }

    pub fn get_force_properties(&self) -> ForceProperties {
        self.force_properties
    }

    pub fn get_plot_data(&self, start: Epoch, end: Epoch, step: TimeSpan) -> Option<OrbitPlotData> {
        match self.get_state_at_epoch(start) {
            Some(state) => {
                let mut plot_data = OrbitPlotData::new(self.id.clone());
                plot_data.add_state(OrbitPlotState::from(state));
                let mut next_epoch: Epoch = start + step;
                while next_epoch <= end {
                    match self.get_state_at_epoch(next_epoch) {
                        Some(state) => {
                            plot_data.add_state(OrbitPlotState::from(state));
                            next_epoch += step;
                        }
                        None => {
                            return None;
                        }
                    }
                }
                Some(plot_data)
            }
            None => None,
        }
    }

    pub fn get_keplerian_state(&self) -> Option<KeplerianState> {
        self.keplerian_state
    }

    pub fn get_close_approach(
        &mut self,
        other: &mut Satellite,
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

    pub fn get_observatory_access_report(
        &mut self,
        observatories: Vec<Observatory>,
        start: Epoch,
        end: Epoch,
        min_el: f64,
        min_duration: TimeSpan,
    ) -> Option<HorizonAccessReport> {
        // Get TEME states for this satellite
        let sat_ephem = self.get_ephemeris(start, end, min_duration)?;

        // Create empty report
        let mut report = HorizonAccessReport::new(start, end, min_el, min_duration);

        // Parallelize the access report generation across observatories
        let accesses = observatories
            .par_iter()
            .filter_map(|obs| {
                let obs_ephem = obs.get_ephemeris(start, end, min_duration);
                sat_ephem.get_horizon_accesses(&obs_ephem, min_el, min_duration)
            })
            .collect::<Vec<_>>();

        report.set_accesses(accesses.into_iter().flatten().collect());
        Some(report)
    }
}
