use crate::configs::{
    CONJUNCTION_STEP_MINUTES, DEFAULT_NORAD_ANALYST_ID, DEFAULT_STEP_MINUTES, MAX_NEWTON_ITERATIONS, NEWTON_TOLERANCE,
    ZERO_TOLERANCE,
};
use crate::elements::{CartesianState, CartesianVector, HorizonState};
use crate::enums::ReferenceFrame;
use crate::events::{CloseApproach, HorizonAccess, ManeuverEvent, ProximityEvent};
use crate::time::{Epoch, TimeSpan};
use saal::satellite;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Ephemeris {
    handle: Arc<EphemerisHandle>,
}

#[derive(Debug)]
pub struct EphemerisHandle {
    satellite_id: String,
    norad_id: i32,
    states: RwLock<Vec<CartesianState>>,
    uniform_grid: RwLock<UniformGrid>,
}

#[derive(Debug, Clone, Copy)]
struct UniformGrid {
    start_epoch: Epoch,
    step_seconds: Option<f64>,
    is_uniform: bool,
}

const UNIFORM_STEP_TOLERANCE_SECONDS: f64 = ZERO_TOLERANCE * 86_400.0;

impl Ephemeris {
    pub fn get_satellite_id(&self) -> String {
        self.handle.satellite_id.clone()
    }

    pub fn get_norad_id(&self) -> i32 {
        self.handle.norad_id
    }

    pub fn get_number_of_states(&self) -> Result<i32, String> {
        Ok(self.handle.states.read().unwrap().len() as i32)
    }
    pub fn add_state(&self, state: CartesianState) -> Result<(), String> {
        let mut states = self.handle.states.write().unwrap();
        let teme_state = state.to_frame(ReferenceFrame::TEME);
        let idx = insert_state(&mut states, teme_state);
        let mut uniform_grid = self.handle.uniform_grid.write().unwrap();
        update_uniform_grid(&mut uniform_grid, &states, idx);
        Ok(())
    }
    pub fn new(satellite_id: String, norad_id: Option<i32>, state: CartesianState) -> Result<Self, String> {
        let handle = EphemerisHandle {
            satellite_id: satellite_id.clone(),
            norad_id: norad_id.unwrap_or(DEFAULT_NORAD_ANALYST_ID),
            states: RwLock::new(vec![state.to_frame(ReferenceFrame::TEME)]),
            uniform_grid: RwLock::new(UniformGrid {
                start_epoch: state.epoch,
                step_seconds: None,
                is_uniform: true,
            }),
        };
        Ok(Self {
            handle: Arc::new(handle),
        })
    }

    pub fn get_state_at_epoch(&self, epoch: Epoch) -> Option<CartesianState> {
        let states = self.handle.states.read().ok()?;
        let uniform_grid = self.handle.uniform_grid.read().ok()?;
        interpolate_state_with_grid(&states, epoch, &uniform_grid)
    }

    pub fn get_next_horizon_crossing(
        &self,
        sensor: &Ephemeris,
        min_epoch: Epoch,
        max_epoch: Epoch,
        min_el: f64,
        step: TimeSpan,
    ) -> Option<HorizonState> {
        let sensor_states = sensor.handle.states.read().ok()?;
        let sat_states = self.handle.states.read().ok()?;
        let sensor_grid = sensor.handle.uniform_grid.read().ok()?;
        let sat_grid = self.handle.uniform_grid.read().ok()?;

        let mut next_epoch = min_epoch;
        let mut current_horizon = HorizonState::from((
            interpolate_state_with_grid(&sensor_states, next_epoch, &sensor_grid)?,
            interpolate_state_with_grid(&sat_states, next_epoch, &sat_grid)?,
        ));
        next_epoch += step;

        log::debug!(
            "Searching for horizon crossings between {} and {} for satellite {} and sensor {}",
            min_epoch.to_iso(),
            max_epoch.to_iso(),
            self.get_satellite_id(),
            sensor.get_satellite_id()
        );

        log::debug!(
            "At {} satellite {} is {:3} deg elevation to sensor {} traveling at {:.3} deg/s",
            current_horizon.epoch.to_iso(),
            self.get_satellite_id(),
            current_horizon.elements.elevation,
            sensor.get_satellite_id(),
            current_horizon.elements.elevation_rate.unwrap()
        );

        while next_epoch <= max_epoch
            && interpolate_state_with_grid(&sensor_states, next_epoch, &sensor_grid).is_some()
            && interpolate_state_with_grid(&sat_states, next_epoch, &sat_grid).is_some()
        {
            let old_horizon = current_horizon;

            current_horizon = HorizonState::from((
                interpolate_state_with_grid(&sensor_states, next_epoch, &sensor_grid).unwrap(),
                interpolate_state_with_grid(&sat_states, next_epoch, &sat_grid).unwrap(),
            ));
            let old_sign = (old_horizon.elements.elevation - min_el).signum();
            let current_sign = (current_horizon.elements.elevation - min_el).signum();

            if old_sign != current_sign {
                log::debug!(
                    "Detected {:.2} deg elevation crossing from {:.2} deg to {:.2} deg between {} and {} for satellite {} to sensor {}",
                    min_el,
                    old_horizon.elements.elevation,
                    current_horizon.elements.elevation,
                    old_horizon.epoch.to_iso(),
                    current_horizon.epoch.to_iso(),
                    self.get_satellite_id(),
                    sensor.get_satellite_id()
                );
                let t_guess = estimate_horizon_crossing_epoch(&old_horizon, &current_horizon, min_el);
                log::debug!(
                    "Satellite {} estimated horizon crossing to sensor {} at {}",
                    self.get_satellite_id(),
                    sensor.get_satellite_id(),
                    t_guess.to_iso()
                );
                if t_guess < min_epoch || t_guess > max_epoch {
                    log::debug!(
                        "Satellite {} estimated horizon crossing to sensor {} at {} is out of bounds ({} - {})",
                        self.get_satellite_id(),
                        sensor.get_satellite_id(),
                        t_guess.to_iso(),
                        min_epoch.to_iso(),
                        max_epoch.to_iso()
                    );
                } else if let Some(crossing) =
                    refine_horizon_crossing(sensor, self, &old_horizon, &current_horizon, t_guess, min_el)
                {
                    if crossing.epoch < min_epoch || crossing.epoch > max_epoch {
                        log::debug!(
                            "Satellite {} refined horizon crossing to sensor {} at {} is out of bounds ({} - {})",
                            self.get_satellite_id(),
                            sensor.get_satellite_id(),
                            crossing.epoch.to_iso(),
                            min_epoch.to_iso(),
                            max_epoch.to_iso()
                        );
                    } else {
                        log::debug!(
                            "Satellite {} refined horizon crossing to sensor {} at {}",
                            self.get_satellite_id(),
                            sensor.get_satellite_id(),
                            crossing.epoch.to_iso()
                        );
                        return Some(crossing);
                    }
                }
            }

            next_epoch += step;
        }
        None
    }

    pub fn get_horizon_accesses(
        &self,
        sensor: &Ephemeris,
        min_el: f64,
        min_duration: TimeSpan,
    ) -> Option<Vec<HorizonAccess>> {
        let (start_epoch, end_epoch) = self.get_epoch_range()?;
        let sensor_states = sensor.handle.states.read().ok()?;
        let sat_states = self.handle.states.read().ok()?;
        let sensor_grid = sensor.handle.uniform_grid.read().ok()?;
        let sat_grid = self.handle.uniform_grid.read().ok()?;
        let sensor_id = sensor.get_satellite_id();
        let sat_id = self.get_satellite_id();
        let dt = TimeSpan::from_minutes(DEFAULT_STEP_MINUTES).min(min_duration * 0.5);

        let mut accesses = Vec::new();
        let mut next_epoch = start_epoch;
        let current_horizon = HorizonState::from((
            interpolate_state_with_grid(&sensor_states, next_epoch, &sensor_grid)?,
            interpolate_state_with_grid(&sat_states, next_epoch, &sat_grid)?,
        ));

        let mut last_entry: Option<HorizonState>;
        let mut last_exit: Option<HorizonState>;
        if current_horizon.elements.elevation > min_el {
            last_entry = Some(current_horizon);
            last_exit = self.get_next_horizon_crossing(sensor, next_epoch, end_epoch, min_el, dt);
            if last_entry.is_some() && last_exit.is_none() {
                log::debug!(
                    "Satellite {} is always above sensor {} horizon between {} and {}",
                    sat_id,
                    sensor_id,
                    start_epoch.to_iso(),
                    end_epoch.to_iso()
                );
                accesses.push(HorizonAccess::new(
                    sat_id,
                    sensor_id,
                    &HorizonState::from((
                        interpolate_state_with_grid(&sensor_states, start_epoch, &sensor_grid).unwrap(),
                        interpolate_state_with_grid(&sat_states, start_epoch, &sat_grid).unwrap(),
                    )),
                    &HorizonState::from((
                        interpolate_state_with_grid(&sensor_states, end_epoch, &sensor_grid).unwrap(),
                        interpolate_state_with_grid(&sat_states, end_epoch, &sat_grid).unwrap(),
                    )),
                ));
                return Some(accesses);
            }
        } else {
            last_entry = self.get_next_horizon_crossing(sensor, next_epoch, end_epoch, min_el, dt);
            last_exit = None;
        }

        if last_entry.is_none() {
            log::debug!(
                "Satellite {} is never above sensor {} horizon between {} and {}",
                sat_id,
                sensor_id,
                start_epoch.to_iso(),
                end_epoch.to_iso()
            );
            return Some(accesses);
        } else if let Some(exit) = last_exit {
            next_epoch = exit.epoch + dt;
        }

        while next_epoch <= end_epoch {
            if last_entry.is_some() && last_exit.is_some() {
                let entry = last_entry.take().unwrap();
                let exit = last_exit.take().unwrap();
                let duration = exit.epoch - entry.epoch;
                if duration >= min_duration {
                    log::debug!(
                        "Saved satellite {} {:.1}s access to sensor {} from {} to {}",
                        sat_id,
                        duration.in_seconds(),
                        sensor_id,
                        entry.epoch.to_iso(),
                        exit.epoch.to_iso()
                    );
                    accesses.push(HorizonAccess::new(sat_id.clone(), sensor_id.clone(), &entry, &exit));
                } else {
                    log::debug!(
                        "Skipped satellite {} {:.1}s access to sensor {} from {} to {}",
                        sat_id,
                        duration.in_seconds(),
                        sensor_id,
                        entry.epoch.to_iso(),
                        exit.epoch.to_iso()
                    );
                    last_entry = None;
                    last_exit = None;
                }
            }

            if let Some(crossing) = self.get_next_horizon_crossing(sensor, next_epoch, end_epoch, min_el, dt) {
                if crossing.elements.elevation_rate.unwrap() > 0.0 {
                    log::debug!(
                        "Found satellite {} entry to sensor {} horizon at {}",
                        sat_id,
                        sensor_id,
                        crossing.epoch.to_iso()
                    );
                    last_entry = Some(crossing);
                    last_exit = None;
                } else if crossing.elements.elevation_rate.unwrap() < 0.0 {
                    log::debug!(
                        "Found satellite {} exit from sensor {} horizon at {}",
                        sat_id,
                        sensor_id,
                        crossing.epoch.to_iso()
                    );
                    last_exit = Some(crossing);
                }
                next_epoch = crossing.epoch + dt;
            } else {
                break;
            }
        }

        Some(accesses)
    }

    pub fn get_maneuver_event(
        &self,
        future_ephem: &Ephemeris,
        distance_threshold: f64,
        velocity_threshold: f64,
    ) -> Option<ManeuverEvent> {
        let close_approach = self.get_close_approach(future_ephem, distance_threshold)?;
        let epoch = close_approach.get_epoch();
        let state_1 = future_ephem.get_state_at_epoch(epoch)?;
        let state_2 = self.get_state_at_epoch(epoch)?;

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
        if xa_delta[satellite::XA_DELTA_VEL] * 1e3 < velocity_threshold {
            return None;
        }
        let vel = [
            xa_delta[satellite::XA_DELTA_VRADIAL],
            xa_delta[satellite::XA_DELTA_VINTRCK],
            xa_delta[satellite::XA_DELTA_VCRSSTRCK],
        ];
        Some(ManeuverEvent::new(
            self.get_satellite_id(),
            close_approach.get_epoch(),
            CartesianVector::new(vel[0], vel[1], vel[2]) * 1e3,
        ))
    }

    pub fn get_close_approach(&self, other: &Ephemeris, distance_threshold: f64) -> Option<CloseApproach> {
        let (start_epoch, end_epoch) = self.get_epoch_range()?;
        let self_states = self.handle.states.read().ok()?;
        let other_states = other.handle.states.read().ok()?;
        let self_grid = self.handle.uniform_grid.read().ok()?;
        let other_grid = other.handle.uniform_grid.read().ok()?;
        let self_id = self.get_satellite_id();
        let other_id = other.get_satellite_id();

        let mut closest_epoch = start_epoch;
        let mut min_distance = f64::MAX;
        let mut current_epoch = start_epoch;
        let step = TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES);

        while current_epoch <= end_epoch {
            let state_1 = interpolate_state_with_grid(&self_states, current_epoch, &self_grid);
            let state_2 = interpolate_state_with_grid(&other_states, current_epoch, &other_grid);

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
                    if let Some(ca) = refine_close_approach(
                        &self_states,
                        &other_states,
                        &self_grid,
                        &other_grid,
                        self_id.clone(),
                        other_id.clone(),
                        t,
                    ) && ca.get_distance() < min_distance
                        && ca.get_epoch() >= t_min
                        && ca.get_epoch() < t_max
                    {
                        min_distance = ca.get_distance();
                        closest_epoch = ca.get_epoch();
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
                self.get_satellite_id(),
                other.get_satellite_id(),
                closest_epoch,
                min_distance,
            ))
        } else {
            None
        }
    }

    pub fn get_proximity_event(&self, other: &Ephemeris, distance_threshold: f64) -> Option<ProximityEvent> {
        let (start_epoch, end_epoch) = self.get_epoch_range()?;
        let self_states = self.handle.states.read().ok()?;
        let other_states = other.handle.states.read().ok()?;
        let self_grid = self.handle.uniform_grid.read().ok()?;
        let other_grid = other.handle.uniform_grid.read().ok()?;
        let self_id = self.get_satellite_id();
        let other_id = other.get_satellite_id();

        let mut current_epoch = start_epoch;
        let step = TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES);

        let mut global_min_distance = f64::MAX;
        let mut global_max_distance = 0.0_f64;

        // Check distance at start epoch
        let state_1 = interpolate_state_with_grid(&self_states, start_epoch, &self_grid)?;
        let state_2 = interpolate_state_with_grid(&other_states, start_epoch, &other_grid)?;
        let start_distance = (state_1.position - state_2.position).get_magnitude();
        global_min_distance = global_min_distance.min(start_distance);
        global_max_distance = global_max_distance.max(start_distance);

        // Check distance at end epoch
        let state_1 = interpolate_state_with_grid(&self_states, end_epoch, &self_grid)?;
        let state_2 = interpolate_state_with_grid(&other_states, end_epoch, &other_grid)?;
        let end_distance = (state_1.position - state_2.position).get_magnitude();
        global_min_distance = global_min_distance.min(end_distance);
        global_max_distance = global_max_distance.max(end_distance);

        // Early exit if boundary distances exceed threshold
        if global_max_distance > distance_threshold {
            return None;
        }

        while current_epoch <= end_epoch {
            let state_1 = interpolate_state_with_grid(&self_states, current_epoch, &self_grid)?;
            let state_2 = interpolate_state_with_grid(&other_states, current_epoch, &other_grid)?;

            // Estimate the time of the extremum (could be min or max)
            if let Some(t_extremum) = estimate_close_approach_epoch(&state_1, &state_2) {
                let t_min = current_epoch;
                let t_max = current_epoch + step;

                // Only refine if the estimated extremum falls within this interval
                if t_extremum >= t_min
                    && t_extremum <= t_max
                    && let Some(refined_distance) =
                        refine_extremum_distance(&self_states, &other_states, &self_grid, &other_grid, t_extremum)
                {
                    global_min_distance = global_min_distance.min(refined_distance);
                    global_max_distance = global_max_distance.max(refined_distance);

                    // Early exit if max exceeds threshold
                    if global_max_distance > distance_threshold {
                        return None;
                    }
                }
            }

            current_epoch += step;
        }

        // All extrema are within threshold, return a single proximity event
        Some(ProximityEvent::new(
            self_id,
            other_id,
            start_epoch,
            end_epoch,
            global_min_distance,
            global_max_distance,
        ))
    }

    pub fn get_epoch_range(&self) -> Option<(Epoch, Epoch)> {
        let states = self.handle.states.read().ok()?;
        let start = states.first()?.epoch;
        let end = states.last()?.epoch;
        Some((start, end))
    }

    pub fn covers_range(&self, start: Epoch, end: Epoch) -> bool {
        let Ok(states) = self.handle.states.read() else {
            return false;
        };
        let (Some(first), Some(last)) = (states.first(), states.last()) else {
            return false;
        };
        start >= first.epoch && end <= last.epoch
    }

    pub fn covers_epoch(&self, epoch: Epoch) -> bool {
        let Ok(states) = self.handle.states.read() else {
            return false;
        };
        let (Some(first), Some(last)) = (states.first(), states.last()) else {
            return false;
        };
        epoch >= first.epoch && epoch <= last.epoch
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

fn estimate_horizon_crossing_epoch(state_1: &HorizonState, state_2: &HorizonState, min_elevation: f64) -> Epoch {
    let y0 = state_1.elements.elevation;
    let t1 = (state_2.epoch - state_1.epoch).in_seconds();
    let y1 = state_2.elements.elevation;
    let m = (y1 - y0) / t1;

    // Linear interpolation to find the time when the elevation crosses the minimum
    let delta_t = (min_elevation - y0) / m;
    let mut guess = state_1.epoch + TimeSpan::from_seconds(delta_t);
    let (t_min, t_max) = if state_1.epoch <= state_2.epoch {
        (state_1.epoch, state_2.epoch)
    } else {
        (state_2.epoch, state_1.epoch)
    };
    if guess < t_min {
        guess = t_min;
    } else if guess > t_max {
        guess = t_max;
    }
    guess
}

fn refine_horizon_crossing(
    sensor: &Ephemeris,
    sat: &Ephemeris,
    state_1: &HorizonState,
    state_2: &HorizonState,
    t_guess: Epoch,
    min_el: f64,
) -> Option<HorizonState> {
    let sensor_states = sensor.handle.states.read().ok()?;
    let sat_states = sat.handle.states.read().ok()?;
    let sensor_grid = sensor.handle.uniform_grid.read().ok()?;
    let sat_grid = sat.handle.uniform_grid.read().ok()?;
    // Use Newton's method to refine the time of horizon crossing
    let (low_state, high_state) = if state_1.epoch <= state_2.epoch {
        (state_1, state_2)
    } else {
        (state_2, state_1)
    };
    let mut t_lo = low_state.epoch;
    let mut t_hi = high_state.epoch;
    let mut f_lo = low_state.elements.elevation - min_el;
    let f_hi = high_state.elements.elevation - min_el;

    if f_lo.abs() < ZERO_TOLERANCE {
        return Some(*low_state);
    }
    if f_hi.abs() < ZERO_TOLERANCE {
        return Some(*high_state);
    }
    if f_lo.signum() == f_hi.signum() {
        return None;
    }

    let mut t = t_guess;
    if t < t_lo {
        t = t_lo;
    } else if t > t_hi {
        t = t_hi;
    }

    for _ in 0..MAX_NEWTON_ITERATIONS {
        // Propagate both satellites to time t and get their horizon states
        let sensor_teme = interpolate_state_with_grid(&sensor_states, t, &sensor_grid)?;
        let target_teme = interpolate_state_with_grid(&sat_states, t, &sat_grid)?;

        let horizon = HorizonState::from((sensor_teme, target_teme));

        let elevation = horizon.elements.elevation;
        let elevation_rate = horizon.elements.elevation_rate.unwrap();
        let f = elevation - min_el;
        if f.abs() < ZERO_TOLERANCE {
            return Some(horizon);
        }

        let mut t_new = if elevation_rate.abs() > ZERO_TOLERANCE {
            t + TimeSpan::from_seconds(-f / elevation_rate)
        } else {
            t
        };

        if t_new <= t_lo || t_new >= t_hi {
            t_new = t_lo + (t_hi - t_lo) * 0.5;
        }

        let sensor_new = interpolate_state_with_grid(&sensor_states, t_new, &sensor_grid)?;
        let target_new = interpolate_state_with_grid(&sat_states, t_new, &sat_grid)?;
        let horizon_new = HorizonState::from((sensor_new, target_new));
        let f_new = horizon_new.elements.elevation - min_el;

        if f_new.abs() < ZERO_TOLERANCE {
            return Some(horizon_new);
        }

        if (t_hi - t_lo).in_seconds().abs() < NEWTON_TOLERANCE {
            return Some(horizon_new);
        }

        if f_lo.signum() == f_new.signum() {
            t_lo = t_new;
            f_lo = f_new;
        } else {
            t_hi = t_new;
        }
        t = t_new;
    }

    Some(HorizonState::from((
        interpolate_state_with_grid(&sensor_states, t, &sensor_grid)?,
        interpolate_state_with_grid(&sat_states, t, &sat_grid)?,
    )))
}

fn refine_close_approach(
    ephem_1_states: &[CartesianState],
    ephem_2_states: &[CartesianState],
    ephem_1_grid: &UniformGrid,
    ephem_2_grid: &UniformGrid,
    ephem_1_satellite_id: String,
    ephem_2_satellite_id: String,
    t_guess: Epoch,
) -> Option<CloseApproach> {
    // Use Newton's method to refine the time of closest approach
    let mut t = t_guess;

    for _ in 0..MAX_NEWTON_ITERATIONS {
        // Propagate both satellites to time t and get their positions and velocities
        let state_1 = interpolate_state_with_grid(ephem_1_states, t, ephem_1_grid)?;
        let state_2 = interpolate_state_with_grid(ephem_2_states, t, ephem_2_grid)?;

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
    let state_1 = interpolate_state_with_grid(ephem_1_states, t, ephem_1_grid)?;
    let state_2 = interpolate_state_with_grid(ephem_2_states, t, ephem_2_grid)?;
    let range = (state_1.position - state_2.position).get_magnitude();

    Some(CloseApproach::new(ephem_1_satellite_id, ephem_2_satellite_id, t, range))
}

fn refine_extremum_distance(
    ephem_1_states: &[CartesianState],
    ephem_2_states: &[CartesianState],
    ephem_1_grid: &UniformGrid,
    ephem_2_grid: &UniformGrid,
    t_guess: Epoch,
) -> Option<f64> {
    let mut t = t_guess;

    for _ in 0..MAX_NEWTON_ITERATIONS {
        let state_1 = interpolate_state_with_grid(ephem_1_states, t, ephem_1_grid)?;
        let state_2 = interpolate_state_with_grid(ephem_2_states, t, ephem_2_grid)?;

        let dr = state_1.position - state_2.position;
        let dv = state_1.velocity - state_2.velocity;
        let drdv = dr.dot(&dv);
        let dvdv = dv.dot(&dv);

        let dt = -drdv / dvdv;
        t += TimeSpan::from_seconds(dt);

        if dt.abs() < NEWTON_TOLERANCE {
            break;
        }
    }

    let state_1 = interpolate_state_with_grid(ephem_1_states, t, ephem_1_grid)?;
    let state_2 = interpolate_state_with_grid(ephem_2_states, t, ephem_2_grid)?;
    Some((state_1.position - state_2.position).get_magnitude())
}

fn insert_state(states: &mut Vec<CartesianState>, state: CartesianState) -> usize {
    match states.binary_search_by(|s| s.epoch.cmp(&state.epoch)) {
        Ok(idx) => {
            states[idx] = state;
            idx
        }
        Err(idx) => {
            states.insert(idx, state);
            idx
        }
    }
}

fn interpolate_state_with_grid(
    states: &[CartesianState],
    epoch: Epoch,
    uniform_grid: &UniformGrid,
) -> Option<CartesianState> {
    if states.is_empty() {
        return None;
    }
    if states.len() == 1 {
        return Some(states[0]);
    }
    if epoch <= states.first()?.epoch {
        return Some(states.first()?.to_owned());
    }
    if epoch >= states.last()?.epoch {
        return Some(states.last()?.to_owned());
    }

    if uniform_grid.is_uniform
        && let Some(step_seconds) = uniform_grid.step_seconds
        && let Some(step_days) = Some(step_seconds / 86_400.0)
        && step_days > 0.0
    {
        let offset_days = epoch.days_since_1950 - uniform_grid.start_epoch.days_since_1950;
        let raw_idx = offset_days / step_days;
        let idx_rounded = raw_idx.round();
        if (raw_idx - idx_rounded).abs() * step_seconds <= UNIFORM_STEP_TOLERANCE_SECONDS {
            let idx = idx_rounded as isize;
            if idx >= 0 && (idx as usize) < states.len() {
                return Some(states[idx as usize]);
            }
        }
        let lower_idx = raw_idx.floor() as isize;
        if lower_idx < 0 {
            return Some(states.first()?.to_owned());
        }
        let upper_idx = lower_idx + 1;
        if (upper_idx as usize) >= states.len() {
            return Some(states.last()?.to_owned());
        }
        let a = &states[lower_idx as usize];
        let b = &states[upper_idx as usize];
        return Some(hermite_interpolate(a, b, epoch));
    }

    match states.binary_search_by(|s| s.epoch.cmp(&epoch)) {
        Ok(idx) => Some(states[idx]),
        Err(idx) => {
            let upper_idx = idx;
            let lower_idx = idx - 1;
            let a = &states[lower_idx];
            let b = &states[upper_idx];
            Some(hermite_interpolate(a, b, epoch))
        }
    }
}

fn hermite_interpolate(a: &CartesianState, b: &CartesianState, t: Epoch) -> CartesianState {
    let dt_days = b.epoch.days_since_1950 - a.epoch.days_since_1950;
    if dt_days.abs() < f64::EPSILON {
        return *a;
    }
    let dt_seconds = dt_days * 86_400.0;
    let tau = (t.days_since_1950 - a.epoch.days_since_1950) / dt_days;

    let tau2 = tau * tau;
    let tau3 = tau2 * tau;

    let h00 = 2.0 * tau3 - 3.0 * tau2 + 1.0;
    let h10 = tau3 - 2.0 * tau2 + tau;
    let h01 = -2.0 * tau3 + 3.0 * tau2;
    let h11 = tau3 - tau2;

    let dh00 = 6.0 * tau2 - 6.0 * tau;
    let dh10 = 3.0 * tau2 - 4.0 * tau + 1.0;
    let dh01 = -dh00;
    let dh11 = 3.0 * tau2 - 2.0 * tau;

    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    for i in 0..3 {
        pos[i] = h00 * a.position[i]
            + h10 * dt_seconds * a.velocity[i]
            + h01 * b.position[i]
            + h11 * dt_seconds * b.velocity[i];
        vel[i] = (dh00 * a.position[i]
            + dh10 * dt_seconds * a.velocity[i]
            + dh01 * b.position[i]
            + dh11 * dt_seconds * b.velocity[i])
            / dt_seconds;
    }

    CartesianState::new(
        t,
        CartesianVector::from(pos),
        CartesianVector::from(vel),
        ReferenceFrame::TEME,
    )
}

fn update_uniform_grid(grid: &mut UniformGrid, states: &[CartesianState], idx: usize) {
    if !grid.is_uniform {
        return;
    }
    if states.is_empty() {
        return;
    }

    grid.start_epoch = states[0].epoch;
    if states.len() == 1 {
        grid.step_seconds = None;
        return;
    }

    let step_seconds = match grid.step_seconds {
        Some(step_seconds) => step_seconds,
        None => {
            let step = (states[1].epoch - states[0].epoch).in_seconds();
            if step <= 0.0 {
                grid.is_uniform = false;
                return;
            }
            grid.step_seconds = Some(step);
            step
        }
    };

    if step_seconds <= 0.0 {
        grid.is_uniform = false;
        return;
    }

    let mut valid = true;
    if idx > 0 {
        let diff = (states[idx].epoch - states[idx - 1].epoch).in_seconds();
        if (diff - step_seconds).abs() > UNIFORM_STEP_TOLERANCE_SECONDS {
            valid = false;
        }
    }
    if idx + 1 < states.len() {
        let diff = (states[idx + 1].epoch - states[idx].epoch).in_seconds();
        if (diff - step_seconds).abs() > UNIFORM_STEP_TOLERANCE_SECONDS {
            valid = false;
        }
    }

    if !valid {
        grid.is_uniform = false;
    }
}

#[cfg(test)]
mod tests {
    use super::estimate_horizon_crossing_epoch;
    use crate::elements::{HorizonElements, HorizonState};
    use crate::enums::TimeSystem;
    use crate::time::Epoch;

    #[test]
    fn test_estimate_horizon_crossing_epoch_clamps_with_reversed_epochs() {
        let mut elements_1 = HorizonElements::new(0.0, 1.0);
        elements_1.elevation_rate = Some(0.1);
        let mut elements_2 = HorizonElements::new(0.0, -1.0);
        elements_2.elevation_rate = Some(-0.1);
        let state_1 = HorizonState::new(
            Epoch::from_iso("2026-01-01T00:10:00.000000Z", TimeSystem::UTC),
            elements_1,
        );
        let state_2 = HorizonState::new(
            Epoch::from_iso("2026-01-01T00:00:00.000000Z", TimeSystem::UTC),
            elements_2,
        );

        let guess = estimate_horizon_crossing_epoch(&state_1, &state_2, 0.0);
        let min_epoch = if state_1.epoch <= state_2.epoch {
            state_1.epoch
        } else {
            state_2.epoch
        };
        let max_epoch = if state_1.epoch >= state_2.epoch {
            state_1.epoch
        } else {
            state_2.epoch
        };

        assert!(guess >= min_epoch);
        assert!(guess <= max_epoch);
    }
}
