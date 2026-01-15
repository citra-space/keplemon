use super::ForceProperties;
use crate::bodies::Satellite;
use crate::elements::{CartesianState, KeplerianState, TLE};
use crate::enums::TimeSystem;
use crate::estimation::Observation;
use crate::time::Epoch;
use nalgebra::{DMatrix, DVector};
use saal::{satellite, sgp4};

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
    pub fn reload(&mut self) -> Result<(), String> {
        match &mut self.tle {
            Some(tle) => {
                tle.reload()?;
                Ok(())
            }
            None => Err("No TLE data to reload".to_string()),
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
            Some(tle) => tle.get_cartesian_state_at_epoch(epoch).ok(),
            None => panic!("Propagation of osculating elements has not been implemented"),
        }
    }

    pub fn get_keplerian_state_at_epoch(&self, epoch: Epoch) -> Option<KeplerianState> {
        match &self.tle {
            Some(tle) => tle.get_keplerian_state_at_epoch(epoch).ok(),
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
            Some(tle) => Ok(Self::from(TLE::new(
                tle.satellite_id.clone(),
                tle.norad_id,
                tle.name.clone(),
                tle.classification,
                tle.designator.clone(),
                tle.get_keplerian_state_at_epoch(epoch)?,
                tle.force_properties,
            )?)),
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }
}
