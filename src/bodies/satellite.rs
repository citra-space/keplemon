use crate::bodies::Observatory;
use crate::configs::{CONJUNCTION_STEP_MINUTES, DEFAULT_NORAD_ANALYST_ID, MIN_EPHEMERIS_POINTS};
use crate::elements::{
    BoreToBodyAngles, CartesianState, CartesianVector, Ephemeris, GeodeticPosition, KeplerianState, OrbitPlotData,
    OrbitPlotState, RelativeState, TLE,
};
use crate::enums::{Classification, KeplerianType, ReferenceFrame};
use crate::estimation::{Observation, ObservationAssociation, ObservationCollection, ObservationResidual};
use crate::events::{CloseApproach, HorizonAccessReport, ManeuverEvent, ProximityReport};
use crate::propagation::{ForceProperties, InertialPropagator};
use crate::time::{Epoch, TimeSpan};
use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;
use saal::{astro, satellite};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Satellite {
    pub id: String,
    pub norad_id: i32,
    pub name: Option<String>,
    force_properties: ForceProperties,
    keplerian_state: Option<KeplerianState>,
    inertial_propagator: Option<InertialPropagator>,
    ephemeris_cache: Option<Ephemeris>,
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

    pub fn get_rms(&self, obs: &[Observation]) -> Result<f64, String> {
        let squared_residuals: Vec<f64> = obs
            .iter()
            .filter_map(|ob| {
                let state = self.interpolate_state_at_epoch(ob.get_epoch())?;
                let residual = ob.get_residual_from_state(&state)?;
                Some(residual.get_range().powi(2))
            })
            .collect();

        if squared_residuals.is_empty() {
            return Err("No valid residuals computed".to_string());
        }

        let sum: f64 = squared_residuals.iter().sum();
        let rms = (sum / squared_residuals.len() as f64).sqrt();
        Ok(rms)
    }

    pub fn get_residuals(&self, obs: &[Observation]) -> Result<Vec<ObservationResidual>, String> {
        let residuals: Vec<ObservationResidual> = obs
            .iter()
            .filter_map(|ob| {
                let state = self.interpolate_state_at_epoch(ob.get_epoch())?;
                ob.get_residual_from_state(&state)
            })
            .collect();

        if residuals.is_empty() {
            return Err("No valid residuals computed".to_string());
        }

        Ok(residuals)
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
        if self.ephemeris_cache.is_some()
            && self
                .ephemeris_cache
                .as_ref()
                .unwrap()
                .covers_range(start_epoch, end_epoch)
        {
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
                self.inertial_propagator.as_mut().unwrap().reload().ok()?;
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

    pub fn interpolate_state_at_epoch(&self, epoch: Epoch) -> Option<CartesianState> {
        // Check if ephemeris is cached and covers the requested epoch
        if self.ephemeris_cache.is_some() && self.ephemeris_cache.as_ref().unwrap().covers_epoch(epoch) {
            return self.ephemeris_cache.as_ref().unwrap().get_state_at_epoch(epoch);
        }
        // Fall back to propagator-based state computation
        self.get_state_at_epoch(epoch)
    }

    pub fn get_associations(&self, collections: &Vec<ObservationCollection>) -> Vec<ObservationAssociation> {
        let mut associations: Vec<ObservationAssociation> = Vec::new();
        for collection in collections {
            if let Some(association) = collection.get_association(self) {
                associations.push(association);
            }
        }
        associations
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

    pub fn get_proximity_report(
        &mut self,
        other: &mut Satellite,
        start_epoch: Epoch,
        end_epoch: Epoch,
        distance_threshold: f64,
    ) -> Option<ProximityReport> {
        if self.keplerian_state.is_none() || other.keplerian_state.is_none() {
            return None;
        }

        let ephemeris = self.get_ephemeris(start_epoch, end_epoch, TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES))?;
        let other_ephemeris =
            other.get_ephemeris(start_epoch, end_epoch, TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES))?;

        let mut report = ProximityReport::new(start_epoch, end_epoch, distance_threshold);
        if let Some(event) = ephemeris.get_proximity_event(&other_ephemeris, distance_threshold) {
            report.set_events(vec![event]);
        }
        Some(report)
    }

    pub fn get_maneuver_event(
        &mut self,
        future_sat: &mut Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: f64,
        velocity_threshold: f64,
    ) -> Option<ManeuverEvent> {
        if self.keplerian_state.is_none() || future_sat.keplerian_state.is_none() {
            return None;
        }

        let ephemeris = self.get_ephemeris(start, end, TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES))?;
        let future_ephemeris =
            future_sat.get_ephemeris(start, end, TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES))?;

        ephemeris.get_maneuver_event(&future_ephemeris, distance_threshold, velocity_threshold)
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

#[cfg(test)]
mod tests {
    use super::Satellite;
    use crate::bodies::Observatory;
    use crate::elements::TLE;
    use crate::enums::TimeSystem;
    use crate::time::{Epoch, TimeSpan};
    use approx::assert_abs_diff_eq;

    fn make_satellite(line_1: &str, line_2: &str) -> Satellite {
        let tle = TLE::from_lines(line_1, line_2, None).unwrap();
        Satellite::from(tle)
    }

    #[test]
    fn test_from_tle() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let sat_1 = make_satellite(
            "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999",
            "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660",
        );
        let sat_2 = make_satellite(
            "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
            "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950",
        );
        let sat_3 = make_satellite(
            "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
            "2 37605   2.1234  87.2060 0006091 220.8721 161.7206  1.00271635 50950",
        );

        assert_eq!(sat_1.norad_id, 25544);
        let pos_2 = sat_2.get_geodetic_position().expect("missing geodetic position");
        let _pos_3 = sat_3.get_geodetic_position().expect("missing geodetic position");
        assert_abs_diff_eq!(pos_2.latitude, 0.3938497796549098, epsilon = 0.1);
        assert_abs_diff_eq!(pos_2.longitude, 55.074384090833696, epsilon = 0.1);
        assert_abs_diff_eq!(pos_2.altitude, 35808.08113476326, epsilon = 0.1);
    }

    #[test]
    fn test_get_close_approach() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let mut sat_2 = make_satellite(
            "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
            "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950",
        );
        let mut sat_3 = make_satellite(
            "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
            "2 37605   2.1234  87.2060 0006091 220.8721 161.7206  1.00271635 50950",
        );
        let start = Epoch::from_iso("2025-04-15T12:00:00.000000Z", TimeSystem::UTC);
        let end = Epoch::from_iso("2025-04-16T12:00:00.000000Z", TimeSystem::UTC);
        let ca = sat_2
            .get_close_approach(&mut sat_3, start, end, 25.0)
            .expect("missing close approach");
        assert_eq!(ca.get_epoch().to_iso(), "2025-04-15T12:32:28.532");
        assert_abs_diff_eq!(ca.get_distance(), 6.088, epsilon = 0.1);
    }

    #[test]
    fn test_get_relative_state_at_epoch() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let sat_2 = make_satellite(
            "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
            "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950",
        );
        let sat_3 = make_satellite(
            "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
            "2 37605   2.1234  87.2060 0006091 220.8721 161.7206  1.00271635 50950",
        );
        let epoch = Epoch::from_iso("2025-04-15T12:32:28.532000Z", TimeSystem::UTC);
        let state = sat_2
            .get_relative_state_at_epoch(&sat_3, epoch)
            .expect("missing relative state");
        assert_abs_diff_eq!(state.position.get_magnitude(), 6.088, epsilon = 0.1);
        assert_abs_diff_eq!(state.position.get_x(), -3.166, epsilon = 1e-3);
        assert_abs_diff_eq!(state.position.get_y(), -5.2, epsilon = 1e-3);
        assert_abs_diff_eq!(state.position.get_z(), 0.0196, epsilon = 1e-3);
        assert_abs_diff_eq!(state.velocity.get_x(), 0.001, epsilon = 1e-3);
        assert_abs_diff_eq!(state.velocity.get_y(), -0.0003, epsilon = 1e-3);
        assert_abs_diff_eq!(state.velocity.get_z(), -0.059, epsilon = 1e-3);
    }

    #[test]
    fn test_get_body_angles_at_epoch() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let sat_2 = make_satellite(
            "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
            "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950",
        );
        let sat_3 = make_satellite(
            "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
            "2 37605   2.1234  87.2060 0006091 220.8721 161.7206  1.00271635 50950",
        );
        let epoch = Epoch::from_iso("2025-04-15T12:32:28.532000Z", TimeSystem::UTC);
        let angles = sat_2
            .get_body_angles_at_epoch(&sat_3, epoch)
            .expect("missing body angles");
        assert_abs_diff_eq!(angles.get_earth_angle(), 121.3, epsilon = 0.1);
        assert_abs_diff_eq!(angles.get_sun_angle(), 121.0, epsilon = 0.1);
        assert_abs_diff_eq!(angles.get_moon_angle(), 88.0, epsilon = 0.1);
    }

    #[test]
    fn test_get_observatory_access_report() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let line_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999";
        let line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660";
        let tle = TLE::from_lines("ISS", line_1, Some(line_2)).unwrap();
        let mut satellite = Satellite::from(tle);

        let mut obs1 = Observatory::new(34.0, -118.0, 100.0);
        obs1.name = Some("LA Observatory".to_string());
        let mut obs2 = Observatory::new(51.5, -0.1, 50.0);
        obs2.name = Some("London Observatory".to_string());
        let mut obs3 = Observatory::new(-33.9, 18.4, 20.0);
        obs3.name = Some("Cape Town Observatory".to_string());

        let observatories = vec![obs1.clone(), obs2.clone(), obs3.clone()];
        let start = Epoch::from_iso("2025-04-18T04:00:00.000000Z", TimeSystem::UTC);
        let end = Epoch::from_iso("2025-04-18T08:00:00.000000Z", TimeSystem::UTC);
        let min_elevation = 10.0;
        let min_duration = TimeSpan::from_minutes(1.0);

        let report = satellite
            .get_observatory_access_report(observatories, start, end, min_elevation, min_duration)
            .expect("missing access report");

        assert_eq!(report.get_start(), start);
        assert_eq!(report.get_end(), end);
        assert_abs_diff_eq!(report.get_elevation_threshold(), min_elevation, epsilon = 1e-6);
        assert_abs_diff_eq!(
            report.get_duration_threshold().in_minutes(),
            min_duration.in_minutes(),
            epsilon = 1e-6
        );

        let accesses = report.get_accesses();
        assert_eq!(accesses.len(), 3);

        let la_accesses: Vec<_> = accesses.iter().filter(|a| a.get_observatory_id() == obs1.id).collect();
        let london_accesses: Vec<_> = accesses.iter().filter(|a| a.get_observatory_id() == obs2.id).collect();
        let cape_town_accesses: Vec<_> = accesses.iter().filter(|a| a.get_observatory_id() == obs3.id).collect();

        assert_eq!(la_accesses.len(), 1);
        assert_eq!(london_accesses.len(), 0);
        assert_eq!(cape_town_accesses.len(), 2);

        for access in accesses {
            let start_state = access.get_start();
            let end_state = access.get_end();
            assert!(
                start_state.elements.elevation >= min_elevation
                    || (start_state.elements.elevation - min_elevation).abs() <= 0.1
            );
            assert!(
                end_state.elements.elevation >= min_elevation
                    || (end_state.elements.elevation - min_elevation).abs() <= 0.1
            );
            let duration = end_state.epoch - start_state.epoch;
            assert!(
                duration.in_minutes() >= min_duration.in_minutes()
                    || (duration.in_minutes() - min_duration.in_minutes()).abs() <= 0.1
            );
        }
    }

    #[test]
    fn test_interpolate_state_at_epoch_accuracy() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let line_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999";
        let line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660";
        let tle = TLE::from_lines("ISS", line_1, Some(line_2)).unwrap();
        let mut sat_with_cache = Satellite::from(tle.clone());
        let sat_reference = Satellite::from(tle);

        // Cache ephemeris covering a 1-hour window with 60-second steps
        let start = Epoch::from_iso("2020-07-18T12:00:00.000000Z", TimeSystem::UTC);
        let end = Epoch::from_iso("2020-07-18T13:00:00.000000Z", TimeSystem::UTC);
        let step = TimeSpan::from_seconds(60.0);
        sat_with_cache
            .get_ephemeris(start, end, step)
            .expect("failed to cache ephemeris");

        // Test at multiple query epochs within the cached range
        let test_epochs = vec![
            Epoch::from_iso("2020-07-18T12:00:05.000000Z", TimeSystem::UTC), // Between grid points
            Epoch::from_iso("2020-07-18T12:15:00.000000Z", TimeSystem::UTC), // On a grid point
            Epoch::from_iso("2020-07-18T12:30:03.500000Z", TimeSystem::UTC), // Between grid points
            Epoch::from_iso("2020-07-18T12:45:00.000000Z", TimeSystem::UTC), // On a grid point
            Epoch::from_iso("2020-07-18T12:59:55.000000Z", TimeSystem::UTC), // Near end
        ];

        // Position tolerance in km (Hermite interpolation should be very accurate)
        let pos_tolerance_km = 1e-3;
        // Velocity tolerance in km/s (looser due to interpolation between grid points)
        let vel_tolerance_km_s = 1e-4;

        for epoch in test_epochs {
            let interpolated = sat_with_cache
                .interpolate_state_at_epoch(epoch)
                .expect("interpolate_state_at_epoch failed");
            let propagated = sat_reference
                .get_state_at_epoch(epoch)
                .expect("get_state_at_epoch failed");

            // Compare positions
            assert_abs_diff_eq!(
                interpolated.position.get_x(),
                propagated.position.get_x(),
                epsilon = pos_tolerance_km
            );
            assert_abs_diff_eq!(
                interpolated.position.get_y(),
                propagated.position.get_y(),
                epsilon = pos_tolerance_km
            );
            assert_abs_diff_eq!(
                interpolated.position.get_z(),
                propagated.position.get_z(),
                epsilon = pos_tolerance_km
            );

            // Compare velocities
            assert_abs_diff_eq!(
                interpolated.velocity.get_x(),
                propagated.velocity.get_x(),
                epsilon = vel_tolerance_km_s
            );
            assert_abs_diff_eq!(
                interpolated.velocity.get_y(),
                propagated.velocity.get_y(),
                epsilon = vel_tolerance_km_s
            );
            assert_abs_diff_eq!(
                interpolated.velocity.get_z(),
                propagated.velocity.get_z(),
                epsilon = vel_tolerance_km_s
            );
        }
    }
}
