use super::ForceProperties;
use crate::bodies::Satellite;
use crate::elements::{CartesianState, CartesianVector, KeplerianState, TLE};
use crate::enums::{ReferenceFrame, TimeSystem};
use crate::estimation::Observation;
use crate::time::Epoch;
use nalgebra::{DMatrix, DVector};
use saal::{get_last_error_message, satellite, sgp4};
use std::thread;

#[derive(Debug, PartialEq)]
pub struct InertialPropagator {
    tle: Option<TLE>,
}

impl Clone for InertialPropagator {
    fn clone(&self) -> Self {
        match &self.tle {
            Some(tle) => {
                let new_tle = tle.clone();
                new_tle.into()
            }
            None => Self { tle: None },
        }
    }
}

impl From<TLE> for InertialPropagator {
    fn from(tle: TLE) -> Self {
        Self { tle: Some(tle) }
    }
}

impl InertialPropagator {
    fn sgp4_log_enabled() -> bool {
        std::env::var("KEPLEMON_SGP4_LOG").is_ok()
    }

    fn log_sgp4(message: &str, key: i64, epoch_ds50: f64) {
        if Self::sgp4_log_enabled() {
            eprintln!(
                "[tid={:?}] {} key={} epoch_ds50={}",
                thread::current().id(),
                message,
                key,
                epoch_ds50
            );
        }
    }

    fn log_tle_state(context: &str, tle: &TLE, epoch_ds50: f64) {
        if Self::sgp4_log_enabled() {
            let state = tle.get_keplerian_state();
            let mean_motion = state.elements.get_mean_motion(state.get_type());
            eprintln!(
                "[tid={:?}] {} key={} epoch_ds50={} type={:?} mean_motion={} elements={:?} b_star={} b_term={} agom={}",
                thread::current().id(),
                context,
                tle.get_key(),
                epoch_ds50,
                state.get_type(),
                mean_motion,
                state.elements,
                tle.get_b_star(),
                tle.get_b_term(),
                tle.get_agom()
            );
        }
    }

    fn log_sgp4_load_ok(context: &str, key: i64, epoch_ds50: f64) {
        if Self::sgp4_log_enabled() {
            eprintln!(
                "[tid={:?}] {} sgp4::load ok key={} epoch_ds50={}",
                thread::current().id(),
                context,
                key,
                epoch_ds50
            );
        }
    }

    fn log_sgp4_load_err(context: &str, key: i64) {
        if Self::sgp4_log_enabled() {
            eprintln!(
                "[tid={:?}] {} sgp4::load err key={} err={}",
                thread::current().id(),
                context,
                key,
                get_last_error_message()
            );
        }
    }

    pub fn step_to_epoch(&mut self, epoch: Epoch) -> Result<(), String> {
        match self.tle {
            Some(ref mut tle) => {
                let lines = sgp4::reepoch_tle(tle.get_key(), epoch.days_since_1950)?;
                let new_tle = TLE::from_two_lines(&lines.0, &lines.1)?;
                self.tle = Some(new_tle);
                Ok(())
            }
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn get_cartesian_state_at_epoch(&self, epoch: Epoch) -> Option<CartesianState> {
        match &self.tle {
            Some(tle) => {
                Self::log_sgp4(
                    "get_cartesian_state_at_epoch: start",
                    tle.get_key(),
                    epoch.days_since_1950,
                );
                let mut result = sgp4::get_position_velocity(tle.get_key(), epoch.days_since_1950);
                if result.is_err() {
                    Self::log_tle_state(
                        "get_cartesian_state_at_epoch: tle state",
                        tle,
                        epoch.days_since_1950,
                    );
                    Self::log_sgp4(
                        "get_cartesian_state_at_epoch: sgp4 call failed",
                        tle.get_key(),
                        epoch.days_since_1950,
                    );
                    if Self::sgp4_log_enabled() {
                        eprintln!(
                            "[tid={:?}] get_cartesian_state_at_epoch: sgp4 call failed err={}",
                            thread::current().id(),
                            get_last_error_message()
                        );
                    }
                    let load_result = sgp4::load(tle.get_key());
                    if load_result.is_err() {
                        Self::log_sgp4_load_err("get_cartesian_state_at_epoch:", tle.get_key());
                    } else {
                        Self::log_sgp4_load_ok(
                            "get_cartesian_state_at_epoch:",
                            tle.get_key(),
                            epoch.days_since_1950,
                        );
                    }
                    result = sgp4::get_position_velocity(tle.get_key(), epoch.days_since_1950);
                    if result.is_err() && Self::sgp4_log_enabled() {
                        eprintln!(
                            "[tid={:?}] get_cartesian_state_at_epoch: retry sgp4 call failed key={} epoch_ds50={} err={}",
                            thread::current().id(),
                            tle.get_key(),
                            epoch.days_since_1950,
                            get_last_error_message()
                        );
                    }
                }
                match result {
                    Ok((pos, vel)) => {
                        let pos = CartesianVector::from(pos);
                        let vel = CartesianVector::from(vel);
                        Some(CartesianState::new(epoch, pos, vel, ReferenceFrame::TEME))
                    }
                    Err(_) => None,
                }
            }
            None => panic!("Propagation of osculating elements has not been implemented"),
        }
    }

    pub fn get_keplerian_state_at_epoch(&self, epoch: Epoch) -> Option<KeplerianState> {
        match &self.tle {
            Some(tle) => {
                Self::log_sgp4(
                    "get_keplerian_state_at_epoch: start",
                    tle.get_key(),
                    epoch.days_since_1950,
                );
                let mut result = sgp4::get_full_state(tle.get_key(), epoch.days_since_1950);
                if result.is_err() {
                    Self::log_tle_state(
                        "get_keplerian_state_at_epoch: tle state",
                        tle,
                        epoch.days_since_1950,
                    );
                    Self::log_sgp4(
                        "get_keplerian_state_at_epoch: sgp4 call failed",
                        tle.get_key(),
                        epoch.days_since_1950,
                    );
                    if Self::sgp4_log_enabled() {
                        eprintln!(
                            "[tid={:?}] get_keplerian_state_at_epoch: sgp4 call failed err={}",
                            thread::current().id(),
                            get_last_error_message()
                        );
                    }
                    let load_result = sgp4::load(tle.get_key());
                    if load_result.is_err() {
                        Self::log_sgp4_load_err("get_keplerian_state_at_epoch:", tle.get_key());
                    } else {
                        Self::log_sgp4_load_ok(
                            "get_keplerian_state_at_epoch:",
                            tle.get_key(),
                            epoch.days_since_1950,
                        );
                    }
                    result = sgp4::get_full_state(tle.get_key(), epoch.days_since_1950);
                    if result.is_err() && Self::sgp4_log_enabled() {
                        eprintln!(
                            "[tid={:?}] get_keplerian_state_at_epoch: retry sgp4 call failed key={} epoch_ds50={} err={}",
                            thread::current().id(),
                            tle.get_key(),
                            epoch.days_since_1950,
                            get_last_error_message()
                        );
                    }
                }
                match result {
                    Ok(all) => {
                        let start_idx = sgp4::XA_SGP4OUT_MN_A;
                        let mut elements = tle.get_keplerian_state().elements;
                        for i in 0..6 {
                            elements[i] = all[start_idx + i];
                        }

                        Some(KeplerianState::new(
                            epoch,
                            elements,
                            ReferenceFrame::TEME,
                            tle.get_type(),
                        ))
                    }
                    Err(_) => None,
                }
            }
            None => panic!("Propagation of osculating elements has not been implemented"),
        }
    }

    pub fn get_keplerian_state(&self) -> Result<KeplerianState, String> {
        match &self.tle {
            Some(tle) => Ok(tle.get_keplerian_state()),
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn get_force_properties(&self) -> Result<ForceProperties, String> {
        match &self.tle {
            Some(tle) => Ok(tle.get_force_properties()),
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn get_prior_node(&self, epoch: Epoch) -> Result<Epoch, String> {
        match &self.tle {
            Some(tle) => {
                let utc_ds50 = satellite::get_prior_nodal_crossing(
                    tle.get_key(),
                    epoch.to_system(TimeSystem::TAI).unwrap().days_since_1950,
                );
                Ok(Epoch::from_days_since_1950(utc_ds50, TimeSystem::UTC))
            }
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }
    pub fn get_stm(&self, epoch: Epoch, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        match &self.tle {
            Some(tle) => tle.get_stm(epoch, use_drag, use_srp),
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn get_jacobian(&self, ob: &Observation, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        match &self.tle {
            Some(tle) => tle.get_jacobian(ob, use_drag, use_srp),
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn build_perturbed_satellites(&self, use_drag: bool, use_srp: bool) -> Result<Vec<(Satellite, f64)>, String> {
        match &self.tle {
            Some(tle) => tle.build_perturbed_satellites(use_drag, use_srp),
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn new_with_delta_x(&self, delta_x: &DVector<f64>, use_drag: bool, use_srp: bool) -> Result<Self, String> {
        match &self.tle {
            Some(tle) => {
                let new_tle = tle.new_with_delta_x(delta_x, use_drag, use_srp)?;
                Ok(Self::from(new_tle))
            }
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn clone_at_epoch(&self, epoch: Epoch) -> Result<Self, String> {
        match &self.tle {
            Some(tle) => {
                let el_start_idx = sgp4::XA_SGP4OUT_MN_A;
                let el_end_idx = sgp4::XA_SGP4OUT_MN_OMEGA + 1;
                let sgp4_out = sgp4::get_full_state(tle.get_key(), epoch.days_since_1950)?;
                let new_els = &sgp4_out[el_start_idx..el_end_idx];
                let mut elements = tle.get_keplerian_state().elements;
                for i in 0..new_els.len() {
                    elements[i] = new_els[i];
                }
                let state = KeplerianState::new(epoch, elements, ReferenceFrame::TEME, tle.get_type());
                Ok(Self::from(TLE::new(
                    tle.satellite_id.clone(),
                    tle.norad_id,
                    tle.name.clone(),
                    tle.classification,
                    tle.designator.clone(),
                    state,
                    tle.force_properties,
                )?))
            }
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }
}
