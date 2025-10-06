use crate::bodies::Earth;
use crate::configs::{CONJUNCTION_STEP_MINUTES, DEFAULT_NORAD_ANALYST_ID, MAX_NEWTON_ITERATIONS, NEWTON_TOLERANCE};
use crate::elements::{CartesianState, CartesianVector, HorizonState};
use crate::enums::{ReferenceFrame, TimeSystem};
use crate::events::{CloseApproach, HorizonAccess};
use crate::saal::ext_ephem_interface;
use crate::time::{Epoch, TimeSpan};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct Ephemeris {
    key: i64,
    satellite_id: String,
    norad_id: i32,
}

impl Drop for Ephemeris {
    fn drop(&mut self) {
        ext_ephem_interface::remove_key(self.key);
    }
}

impl Ephemeris {
    pub fn get_number_of_states(&self) -> Result<i32, String> {
        ext_ephem_interface::get_number_of_states(self.key)
    }
}

#[pymethods]
impl Ephemeris {
    #[new]
    pub fn new(satellite_id: String, state: CartesianState) -> Self {
        let frame = match state.get_frame() {
            ReferenceFrame::TEME => ext_ephem_interface::COORD_ECI,
            ReferenceFrame::EFG => ext_ephem_interface::COORD_EFG,
            ReferenceFrame::ECR => ext_ephem_interface::COORD_ECR,
            ReferenceFrame::J2000 => ext_ephem_interface::COORD_J2K,
        };
        let key = ext_ephem_interface::add_satellite(
            DEFAULT_NORAD_ANALYST_ID,
            state.get_epoch().days_since_1950,
            Earth::get_equatorial_radius(),
            Earth::get_ke(),
            frame,
        );
        let ephem = Self {
            key,
            satellite_id,
            norad_id: DEFAULT_NORAD_ANALYST_ID,
        };
        ext_ephem_interface::add_satellite_state(
            key,
            state.get_epoch().days_since_1950,
            &state.get_position().into(),
            &state.get_velocity().into(),
            None,
        )
        .unwrap();
        ephem
    }

    #[getter("number_of_states")]
    pub fn py_get_number_of_states(&self) -> PyResult<i32> {
        self.get_number_of_states()
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    pub fn add_state(&self, state: CartesianState) {
        ext_ephem_interface::add_satellite_state(
            self.key,
            state.get_epoch().days_since_1950,
            &state.get_position().into(),
            &state.get_velocity().into(),
            None,
        )
        .unwrap();
    }

    #[getter]
    pub fn get_key(&self) -> i64 {
        self.key
    }

    pub fn get_state_at_epoch(&self, epoch: Epoch) -> Option<CartesianState> {
        let result = ext_ephem_interface::get_posvel_at_ds50(self.key, epoch.days_since_1950);
        match result {
            Ok((pos, vel)) => {
                let pos = CartesianVector::from(pos);
                let vel = CartesianVector::from(vel);
                Some(CartesianState::new(epoch, pos, vel, ReferenceFrame::TEME))
            }
            Err(e) => {
                println!("Error getting state at epoch {}: {}", epoch.to_iso(), e);
                None
            }
        }
    }

    pub fn get_horizon_accesses(
        &self,
        sensor: &Ephemeris,
        min_el: f64,
        min_duration: TimeSpan,
    ) -> Option<Vec<HorizonAccess>> {
        let (ds50_start, ds50_end) = ext_ephem_interface::get_ds50_utc_range(self.key).unwrap();
        let start_epoch = Epoch::from_days_since_1950(ds50_start, TimeSystem::UTC);
        let end_epoch = Epoch::from_days_since_1950(ds50_end, TimeSystem::UTC);
        let dt = min_duration * 0.5;

        let mut accesses = Vec::new();
        let mut next_epoch = start_epoch;
        let mut current_horizon = HorizonState::from_teme_states(
            sensor.get_state_at_epoch(next_epoch)?,
            self.get_state_at_epoch(next_epoch)?,
        );

        let mut always_visible = current_horizon.get_elevation() >= min_el;
        let mut last_entry = current_horizon;
        next_epoch += dt;

        while next_epoch <= end_epoch
            && sensor.get_state_at_epoch(next_epoch).is_some()
            && self.get_state_at_epoch(next_epoch).is_some()
        {
            let old_horizon = current_horizon;
            let old_el_sign = (old_horizon.get_elevation() - min_el).signum();

            current_horizon = HorizonState::from_teme_states(
                sensor.get_state_at_epoch(next_epoch).unwrap(),
                self.get_state_at_epoch(next_epoch).unwrap(),
            );

            let new_el_sign = (current_horizon.get_elevation() - min_el).signum();
            if old_el_sign != new_el_sign {
                always_visible = false;
                let t_guess = estimate_horizon_crossing_epoch(&old_horizon, min_el);
                if t_guess > start_epoch && t_guess < end_epoch {
                    if let Some(crossing) = refine_horizon_crossing(sensor, self, t_guess, min_el) {
                        if crossing.get_elevation_rate().unwrap() > 0.0 {
                            last_entry = crossing;
                        } else if crossing.epoch - last_entry.epoch >= min_duration {
                            accesses.push(HorizonAccess::new(self.get_satellite_id(), &last_entry, &crossing));
                        }
                        if crossing.epoch > next_epoch {
                            next_epoch = crossing.epoch;
                        }
                    }
                }
            }

            next_epoch += dt;
        }

        if accesses.is_empty() && always_visible && self.get_state_at_epoch(end_epoch).is_some() {
            accesses.push(HorizonAccess::new(
                self.get_satellite_id(),
                &HorizonState::from_teme_states(
                    sensor.get_state_at_epoch(start_epoch).unwrap(),
                    self.get_state_at_epoch(start_epoch).unwrap(),
                ),
                &HorizonState::from_teme_states(
                    sensor.get_state_at_epoch(end_epoch).unwrap(),
                    self.get_state_at_epoch(end_epoch).unwrap(),
                ),
            ));
        }
        Some(accesses)
    }

    pub fn get_close_approach(&self, other: &Ephemeris, distance_threshold: f64) -> Option<CloseApproach> {
        let (ds50_start, ds50_end) = ext_ephem_interface::get_ds50_utc_range(self.key).unwrap();
        let start_epoch = Epoch::from_days_since_1950(ds50_start, TimeSystem::UTC);
        let end_epoch = Epoch::from_days_since_1950(ds50_end, TimeSystem::UTC);

        let mut closest_epoch = start_epoch;
        let mut min_distance = f64::MAX;
        let mut current_epoch = start_epoch;
        let step = TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES);

        while current_epoch <= end_epoch {
            let state_1 = self.get_state_at_epoch(current_epoch);
            let state_2 = other.get_state_at_epoch(current_epoch);

            if state_1.is_none() || state_2.is_none() {
                break;
            }

            // Estimate the time of closest approach
            let t_guess = estimate_close_approach_epoch(&state_1?, &state_2?);

            match t_guess {
                Some(t) => {
                    let t_min = current_epoch;
                    let t_max = current_epoch + step;

                    if t < t_min || t > t_max {
                        current_epoch += step;
                        continue;
                    }
                    if let Some(ca) = refine_close_approach(self, other, t) {
                        if ca.get_distance() < min_distance && ca.get_epoch() >= t_min && ca.get_epoch() < t_max {
                            min_distance = ca.get_distance();
                            closest_epoch = ca.get_epoch();
                        }
                    }
                }
                None => {
                    break;
                }
            }

            current_epoch += step;
        }
        if min_distance < distance_threshold {
            Some(CloseApproach::new(
                self.norad_id,
                other.norad_id,
                closest_epoch,
                min_distance,
            ))
        } else {
            None
        }
    }

    #[getter]
    pub fn get_satellite_id(&self) -> String {
        self.satellite_id.clone()
    }
}

fn estimate_close_approach_epoch(state_1: &CartesianState, state_2: &CartesianState) -> Option<Epoch> {
    if state_1.epoch != state_2.epoch {
        None
    } else {
        let t0 = state_1.epoch;

        // Calculate the relative position and velocity
        let dx0 = state_1.position - state_2.position;
        let dv0 = state_1.velocity - state_2.velocity;

        // Quadratic minimization: d(t)^2 = |dx0 + dv0*(t-t0)|^2
        // d/dt set to zero gives t = t0 - (dx0 . dv0)/(dv0 . dv0)
        let numerator = dx0.dot(&dv0);
        let denominator = dv0.dot(&dv0);

        if denominator.abs() < 1e-12 {
            Some(t0)
        } else {
            Some(t0 - TimeSpan::from_seconds(numerator / denominator))
        }
    }
}

fn estimate_horizon_crossing_epoch(state_1: &HorizonState, min_elevation: f64) -> Epoch {
    let t0 = state_1.epoch;

    // Linear interpolation to find the time when the elevation crosses the minimum
    let delta_t = (min_elevation - state_1.get_elevation()) / state_1.get_elevation_rate().unwrap();
    t0 + TimeSpan::from_seconds(delta_t)
}

fn refine_horizon_crossing(
    sensor_ephem: &Ephemeris,
    sat_ephem: &Ephemeris,
    t_guess: Epoch,
    min_el: f64,
) -> Option<HorizonState> {
    // Use Newton's method to refine the time of horizon crossing
    let mut t = t_guess;

    for _ in 0..MAX_NEWTON_ITERATIONS {
        // Propagate both satellites to time t and get their horizon states
        let sensor_teme = sensor_ephem.get_state_at_epoch(t)?;
        let target_teme = sat_ephem.get_state_at_epoch(t)?;

        let horizon = HorizonState::from_teme_states(sensor_teme, target_teme);

        let elevation = horizon.get_elevation();
        let elevation_rate = horizon.get_elevation_rate().unwrap();
        let dt = (min_el - elevation) / elevation_rate;
        t += TimeSpan::from_seconds(dt);
        if dt.abs() < NEWTON_TOLERANCE {
            break;
        }
    }

    Some(HorizonState::from_teme_states(
        sensor_ephem.get_state_at_epoch(t)?,
        sat_ephem.get_state_at_epoch(t)?,
    ))
}

fn refine_close_approach(ephem_1: &Ephemeris, ephem_2: &Ephemeris, t_guess: Epoch) -> Option<CloseApproach> {
    // Use Newton's method to refine the time of closest approach
    let mut t = t_guess;

    for _ in 0..MAX_NEWTON_ITERATIONS {
        // Propagate both satellites to time t and get their positions and velocities
        let state_1 = ephem_1.get_state_at_epoch(t)?;
        let state_2 = ephem_2.get_state_at_epoch(t)?;

        let dr = state_1.position - state_2.position;
        let dv = state_1.velocity - state_2.velocity;
        let drdv = dr.dot(&dv);
        let dvdv = dv.dot(&dv);

        // Newton-Raphson step
        let dt = -drdv / dvdv;
        t += TimeSpan::from_seconds(dt);

        if dt.abs() < NEWTON_TOLERANCE {
            break;
        }
    }

    // At final t, compute range
    let state_1 = ephem_1.get_state_at_epoch(t)?;
    let state_2 = ephem_2.get_state_at_epoch(t)?;
    let range = (state_1.position - state_2.position).get_magnitude();

    Some(CloseApproach::new(ephem_1.norad_id, ephem_2.norad_id, t, range))
}
