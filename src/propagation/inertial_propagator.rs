use super::ForceProperties;
use crate::bodies::Satellite;
use crate::elements::{CartesianState, CartesianVector, KeplerianState, TLE};
use crate::enums::{ReferenceFrame, TimeSystem};
use crate::estimation::Observation;
use crate::saal::{sat_state_interface, sgp4_prop_interface};
use crate::time::Epoch;
use nalgebra::{DMatrix, DVector};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct InertialPropagator {
    tle: Option<TLE>,
}

impl Drop for InertialPropagator {
    fn drop(&mut self) {
        if let Some(tle) = &self.tle {
            sgp4_prop_interface::remove_key(tle.get_key()).unwrap();
        }
    }
}

impl Clone for InertialPropagator {
    fn clone(&self) -> Self {
        match &self.tle {
            Some(tle) => {
                let new_tle = tle.clone();
                sgp4_prop_interface::load_key(new_tle.get_key()).unwrap();
                Self { tle: Some(new_tle) }
            }
            None => Self { tle: None },
        }
    }
}

impl InertialPropagator {
    pub fn step_to_epoch(&mut self, epoch: Epoch) -> Result<(), String> {
        match self.tle {
            Some(ref mut tle) => {
                let lines = sgp4_prop_interface::reepoch_tle(tle.get_key(), epoch.days_since_1950)?;
                let new_tle = TLE::from_two_lines(&lines.0, &lines.1)?;
                self.tle = Some(new_tle);
                Ok(())
            }
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }
}

#[pymethods]
impl InertialPropagator {
    #[staticmethod]
    pub fn from_tle(tle: TLE) -> Self {
        sgp4_prop_interface::load_key(tle.get_key()).unwrap();
        Self { tle: Some(tle) }
    }

    pub fn get_cartesian_state_at_epoch(&self, epoch: Epoch) -> Option<CartesianState> {
        match &self.tle {
            Some(tle) => {
                let result = sgp4_prop_interface::get_posvel_at_ds50(tle.get_key(), epoch.days_since_1950);
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
                let result = sgp4_prop_interface::get_all_at_ds50(tle.get_key(), epoch.days_since_1950);
                match result {
                    Ok(all) => {
                        let start_idx = sgp4_prop_interface::XA_SGP4OUT_MN_A;
                        let mut elements = tle.get_keplerian_state().get_elements();
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

    #[getter]
    pub fn get_keplerian_state(&self) -> PyResult<KeplerianState> {
        match &self.tle {
            Some(tle) => Ok(tle.get_keplerian_state()),
            None => Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Propagation of osculating elements has not been implemented",
            )),
        }
    }

    #[getter]
    pub fn get_force_properties(&self) -> PyResult<ForceProperties> {
        match &self.tle {
            Some(tle) => Ok(tle.get_force_properties()),
            None => Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Propagation of osculating elements has not been implemented",
            )),
        }
    }
}

impl InertialPropagator {
    pub fn get_prior_node(&self, epoch: Epoch) -> Result<Epoch, String> {
        match &self.tle {
            Some(tle) => {
                let utc_ds50 = sat_state_interface::get_prior_nodal_crossing(
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

    pub fn get_jacobian_with_ref(
        &self,
        ob: &Observation,
        use_drag: bool,
        use_srp: bool,
        h_ref: &[f64],
    ) -> Result<DMatrix<f64>, String> {
        match &self.tle {
            Some(tle) => tle.get_jacobian_with_ref(ob, use_drag, use_srp, h_ref),
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn build_perturbed_satellites(
        &self,
        use_drag: bool,
        use_srp: bool,
    ) -> Result<Vec<(Satellite, f64)>, String> {
        match &self.tle {
            Some(tle) => tle.build_perturbed_satellites(use_drag, use_srp),
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn new_with_delta_x(&self, delta_x: &DVector<f64>, use_drag: bool, use_srp: bool) -> Result<Self, String> {
        match &self.tle {
            Some(tle) => {
                let new_tle = tle.new_with_delta_x(delta_x, use_drag, use_srp)?;
                Ok(Self::from_tle(new_tle))
            }
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }

    pub fn clone_at_epoch(&self, epoch: Epoch) -> Result<Self, String> {
        match &self.tle {
            Some(tle) => {
                let el_start_idx = sgp4_prop_interface::XA_SGP4OUT_MN_A;
                let el_end_idx = sgp4_prop_interface::XA_SGP4OUT_MN_OMEGA + 1;
                let sgp4_out = sgp4_prop_interface::get_all_at_ds50(tle.get_key(), epoch.days_since_1950)?;
                let new_els = &sgp4_out[el_start_idx..el_end_idx];
                let mut elements = tle.get_keplerian_state().get_elements();
                for i in 0..new_els.len() {
                    elements[i] = new_els[i];
                }
                let state = KeplerianState::new(epoch, elements, ReferenceFrame::TEME, tle.get_type());
                Ok(Self::from_tle(TLE::new(
                    tle.get_id(),
                    tle.get_norad_id(),
                    tle.get_name(),
                    tle.get_classification(),
                    tle.get_designator(),
                    state,
                    tle.get_force_properties(),
                )?))
            }
            None => Err("Propagation of osculating elements has not been implemented".to_string()),
        }
    }
}
