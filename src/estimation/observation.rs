use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::vec;
use uuid::Uuid;

use super::{ObservationAssociation, ObservationResidual};
use crate::bodies::{Constellation, Satellite, Sensor};
use crate::elements::{CartesianState, CartesianVector, TopocentricElements};
use crate::enums::AssociationConfidence;
use crate::time::Epoch;
use saal::{astro, satellite};

#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    pub id: String,
    sensor: Sensor,
    epoch: Epoch,
    observed_teme_topocentric: TopocentricElements,
    observer_teme_position: CartesianVector,
    observer_lla: [f64; 3],
    observer_theta: f64,
    pub observed_satellite_id: Option<i32>,
}

impl Observation {
    pub fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>) {
        let mut m_vec = vec![self.get_right_ascension(), self.get_declination()];
        let mut w_vec = vec![
            1.0 / self.sensor.angular_noise.powi(2),
            1.0 / self.sensor.angular_noise.powi(2),
        ];
        if self.get_range().is_some() && self.sensor.range_noise.is_some() {
            m_vec.push(self.get_range().unwrap());
            w_vec.push(1.0 / self.sensor.range_noise.unwrap().powi(2));
        }
        if self.get_range_rate().is_some() && self.sensor.range_rate_noise.is_some() {
            m_vec.push(self.get_range_rate().unwrap());
            w_vec.push(1.0 / self.sensor.range_rate_noise.unwrap().powi(2));
        }
        if self.get_right_ascension_rate().is_some() && self.sensor.angular_rate_noise.is_some() {
            m_vec.push(self.get_right_ascension_rate().unwrap());
            w_vec.push(1.0 / self.sensor.angular_rate_noise.unwrap().powi(2));
        }
        if self.get_declination_rate().is_some() && self.sensor.angular_rate_noise.is_some() {
            m_vec.push(self.get_declination_rate().unwrap());
            w_vec.push(1.0 / self.sensor.angular_rate_noise.unwrap().powi(2));
        }
        (m_vec, w_vec)
    }

    pub fn get_predicted_vector(&self, satellite: &Satellite) -> Result<Vec<f64>, String> {
        let mut predicted = Vec::new();
        self.fill_predicted_vector(satellite, &mut predicted)?;
        Ok(predicted)
    }

    pub fn fill_predicted_vector(&self, satellite: &Satellite, out: &mut Vec<f64>) -> Result<(), String> {
        match satellite.get_state_at_epoch(self.get_epoch()) {
            Some(satellite_state) => self.fill_predicted_from_state(&satellite_state, out),
            None => Err(format!(
                "Error propagating satellite {} to {}",
                satellite.id,
                self.get_epoch().to_iso()
            )),
        }
    }

    pub fn fill_predicted_from_state(&self, state: &CartesianState, out: &mut Vec<f64>) -> Result<(), String> {
        let xa_topo = astro::teme_to_topo(
            self.observer_theta,
            self.observer_lla[0],
            &self.observer_teme_position.into(),
            &state.into(),
        )?;
        let has_range = self.get_range().is_some();
        let has_range_rate = self.get_range_rate().is_some();
        let has_ra_rate = self.get_right_ascension_rate().is_some();
        let has_dec_rate = self.get_declination_rate().is_some();
        out.clear();
        out.reserve(2 + has_range as usize + has_range_rate as usize + has_ra_rate as usize + has_dec_rate as usize);
        out.push(xa_topo[astro::XA_TOPO_RA]);
        out.push(xa_topo[astro::XA_TOPO_DEC]);
        if has_range {
            out.push(xa_topo[astro::XA_TOPO_RANGE]);
        }
        if has_range_rate {
            out.push(xa_topo[astro::XA_TOPO_RANGEDOT]);
        }
        if has_ra_rate {
            out.push(xa_topo[astro::XA_TOPO_RADOT]);
        }
        if has_dec_rate {
            out.push(xa_topo[astro::XA_TOPO_DECDOT]);
        }
        Ok(())
    }

    pub fn new(
        sensor: Sensor,
        epoch: Epoch,
        observed_teme_topocentric: TopocentricElements,
        observer_teme_position: CartesianVector,
    ) -> Self {
        let theta_g = epoch.to_fk5_greenwich_angle();
        let observer_lla = astro::gst_teme_to_lla(theta_g, &observer_teme_position.into());
        let observer_theta = theta_g + observer_lla[1].to_radians();
        Self {
            id: Uuid::new_v4().to_string(),
            sensor,
            epoch,
            observed_teme_topocentric,
            observer_teme_position,
            observer_lla,
            observer_theta,
            observed_satellite_id: None,
        }
    }

    pub fn get_sensor(&self) -> Sensor {
        self.sensor.clone()
    }

    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn get_range(&self) -> Option<f64> {
        self.observed_teme_topocentric.range
    }

    pub fn get_range_rate(&self) -> Option<f64> {
        self.observed_teme_topocentric.range_rate
    }

    pub fn get_right_ascension(&self) -> f64 {
        self.observed_teme_topocentric.right_ascension
    }

    pub fn get_declination(&self) -> f64 {
        self.observed_teme_topocentric.declination
    }

    pub fn get_right_ascension_rate(&self) -> Option<f64> {
        self.observed_teme_topocentric.right_ascension_rate
    }

    pub fn get_declination_rate(&self) -> Option<f64> {
        self.observed_teme_topocentric.declination_rate
    }

    pub fn set_range(&mut self, range: Option<f64>) {
        self.observed_teme_topocentric.range = range;
    }

    pub fn set_range_rate(&mut self, range_rate: Option<f64>) {
        self.observed_teme_topocentric.range_rate = range_rate;
    }

    pub fn set_right_ascension(&mut self, right_ascension: f64) {
        self.observed_teme_topocentric.right_ascension = right_ascension;
    }

    pub fn set_declination(&mut self, declination: f64) {
        self.observed_teme_topocentric.declination = declination;
    }

    pub fn get_associations(&self, constellation: &Constellation) -> Vec<ObservationAssociation> {
        let observed_teme_direction = self.observed_teme_topocentric.get_observed_direction();
        let sat_states = constellation.get_states_at_epoch(self.epoch);
        sat_states
            .par_iter()
            .filter_map(|(sat_id, sat_state_option)| match sat_state_option {
                Some(sat_state) => {
                    let sensor_to_satellite = sat_state.position - self.observer_teme_position;
                    let teme_estimate =
                        self.observer_teme_position + (*observed_teme_direction * sensor_to_satellite.get_magnitude());

                    let pos_vel_1 = [
                        sat_state.position[0],
                        sat_state.position[1],
                        sat_state.position[2],
                        sat_state.velocity[0],
                        sat_state.velocity[1],
                        sat_state.velocity[2],
                    ];
                    let pos_vel_2 = [
                        teme_estimate[0],
                        teme_estimate[1],
                        teme_estimate[2],
                        sat_state.velocity[0],
                        sat_state.velocity[1],
                        sat_state.velocity[2],
                    ];
                    let residual = ObservationResidual::from(satellite::get_relative_array(
                        &pos_vel_1,
                        &pos_vel_2,
                        self.epoch.days_since_1950,
                        1,
                    ));

                    if residual.get_range() < 1.0 {
                        Some(ObservationAssociation::new(
                            self.id.clone(),
                            sat_id.clone(),
                            residual,
                            AssociationConfidence::High,
                        ))
                    } else if residual.get_range() < 10.0 {
                        Some(ObservationAssociation::new(
                            self.id.clone(),
                            sat_id.clone(),
                            residual,
                            AssociationConfidence::Medium,
                        ))
                    } else if residual.get_range() < 100.0 {
                        Some(ObservationAssociation::new(
                            self.id.clone(),
                            sat_id.clone(),
                            residual,
                            AssociationConfidence::Low,
                        ))
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect()
    }
    pub fn get_residual(&self, satellite: &Satellite) -> Option<ObservationResidual> {
        match satellite.get_state_at_epoch(self.epoch) {
            Some(satellite_state) => {
                let sensor_to_satellite = satellite_state.position - self.observer_teme_position;
                let teme_estimate = self.observer_teme_position
                    + (*self.observed_teme_topocentric.get_observed_direction() * sensor_to_satellite.get_magnitude());

                let posvel_1 = [
                    satellite_state.position[0],
                    satellite_state.position[1],
                    satellite_state.position[2],
                    satellite_state.velocity[0],
                    satellite_state.velocity[1],
                    satellite_state.velocity[2],
                ];
                let posvel_2 = [
                    teme_estimate[0],
                    teme_estimate[1],
                    teme_estimate[2],
                    satellite_state.velocity[0],
                    satellite_state.velocity[1],
                    satellite_state.velocity[2],
                ];

                Some(ObservationResidual::from(satellite::get_relative_array(
                    &posvel_1,
                    &posvel_2,
                    self.epoch.days_since_1950,
                    1,
                )))
            }
            None => None,
        }
    }
}
