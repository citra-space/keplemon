use crate::configs::{
    CONJUNCTION_STEP_MINUTES, DEFAULT_NORAD_ANALYST_ID, MAX_NEWTON_ITERATIONS, NEWTON_TOLERANCE, ZERO_TOLERANCE,
};
use crate::elements::{CartesianState, CartesianVector, HorizonState};
use crate::enums::ReferenceFrame;
use crate::events::{CloseApproach, HorizonAccess};
use crate::time::{Epoch, TimeSpan};
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Ephemeris {
    handle: Arc<EphemerisHandle>,
}

#[derive(Debug)]
pub struct EphemerisHandle {
    id: String,
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

impl PartialEq for Ephemeris {
    fn eq(&self, other: &Self) -> bool {
        self.handle.id == other.handle.id
    }
}

impl Ephemeris {
    pub fn get_id(&self) -> String {
        self.handle.id.clone()
    }

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
            id: Uuid::new_v4().to_string(),
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
        let dt = min_duration * 0.5;

        let mut accesses = Vec::new();
        let mut next_epoch = start_epoch;
        let mut current_horizon = HorizonState::from((
            interpolate_state_with_grid(&sensor_states, next_epoch, &sensor_grid)?,
            interpolate_state_with_grid(&sat_states, next_epoch, &sat_grid)?,
        ));

        let mut always_visible = current_horizon.elements.elevation >= min_el;
        let mut last_entry = current_horizon;
        next_epoch += dt;

        while next_epoch <= end_epoch
            && interpolate_state_with_grid(&sensor_states, next_epoch, &sensor_grid).is_some()
            && interpolate_state_with_grid(&sat_states, next_epoch, &sat_grid).is_some()
        {
            let old_horizon = current_horizon;
            let old_el_sign = (old_horizon.elements.elevation - min_el).signum();

            current_horizon = HorizonState::from((
                interpolate_state_with_grid(&sensor_states, next_epoch, &sensor_grid).unwrap(),
                interpolate_state_with_grid(&sat_states, next_epoch, &sat_grid).unwrap(),
            ));

            let new_el_sign = (current_horizon.elements.elevation - min_el).signum();
            if old_el_sign != new_el_sign {
                always_visible = false;
                let t_guess = estimate_horizon_crossing_epoch(&old_horizon, min_el);
                if t_guess > start_epoch
                    && t_guess < end_epoch
                    && let Some(crossing) =
                        refine_horizon_crossing(&sensor_states, &sat_states, &sensor_grid, &sat_grid, t_guess, min_el)
                {
                    if crossing.elements.elevation_rate.unwrap() > 0.0 {
                        last_entry = crossing;
                    } else if crossing.epoch - last_entry.epoch >= min_duration {
                        accesses.push(HorizonAccess::new(
                            sat_id.clone(),
                            sensor_id.clone(),
                            &last_entry,
                            &crossing,
                        ));
                    }
                    if crossing.epoch > next_epoch {
                        next_epoch = crossing.epoch;
                    }
                }
            }

            next_epoch += dt;
        }

        if accesses.is_empty()
            && always_visible
            && interpolate_state_with_grid(&sat_states, end_epoch, &sat_grid).is_some()
        {
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
        }
        Some(accesses)
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
    let delta_t = (min_elevation - state_1.elements.elevation) / state_1.elements.elevation_rate.unwrap();
    t0 + TimeSpan::from_seconds(delta_t)
}

fn refine_horizon_crossing(
    sensor_states: &[CartesianState],
    sat_states: &[CartesianState],
    sensor_grid: &UniformGrid,
    sat_grid: &UniformGrid,
    t_guess: Epoch,
    min_el: f64,
) -> Option<HorizonState> {
    // Use Newton's method to refine the time of horizon crossing
    let mut t = t_guess;

    for _ in 0..MAX_NEWTON_ITERATIONS {
        // Propagate both satellites to time t and get their horizon states
        let sensor_teme = interpolate_state_with_grid(sensor_states, t, sensor_grid)?;
        let target_teme = interpolate_state_with_grid(sat_states, t, sat_grid)?;

        let horizon = HorizonState::from((sensor_teme, target_teme));

        let elevation = horizon.elements.elevation;
        let elevation_rate = horizon.elements.elevation_rate.unwrap();
        let dt = (min_el - elevation) / elevation_rate;
        t += TimeSpan::from_seconds(dt);
        if dt.abs() < NEWTON_TOLERANCE {
            break;
        }
    }

    Some(HorizonState::from((
        interpolate_state_with_grid(sensor_states, t, sensor_grid)?,
        interpolate_state_with_grid(sat_states, t, sat_grid)?,
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

pub fn construct_ephemeris_id(start: Epoch, end: Epoch, step: TimeSpan) -> String {
    format!("{}_{}_{:04}", start.to_iso(), end.to_iso(), step.in_seconds())
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

impl Ephemeris {
    fn get_epoch_range(&self) -> Option<(Epoch, Epoch)> {
        let states = self.handle.states.read().ok()?;
        let start = states.first()?.epoch;
        let end = states.last()?.epoch;
        Some((start, end))
    }
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
