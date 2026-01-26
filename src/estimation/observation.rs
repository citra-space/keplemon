use super::{ObservationAssociation, ObservationResidual};
use crate::bodies::{Constellation, Observatory, Satellite, Sensor};
use crate::configs::{
    self, DEFAULT_ANGULAR_RATE_NOISE, HIGH_ASSOCIATION_CLOS_RANGE, LOW_ASSOCIATION_CLOS_RANGE,
    MEDIUM_ASSOCIATION_CLOS_RANGE,
};
use crate::elements::{CartesianState, CartesianVector, TopocentricElements};
use crate::enums::{AssociationConfidence, TimeSystem};
use crate::time::Epoch;
use log;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use saal::{astro, get_last_error_message, obs, satellite, sensor};
use std::vec;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    pub id: String,
    sensor: Sensor,
    epoch: Epoch,
    observed_teme_topocentric: TopocentricElements,
    observer_teme_position: CartesianVector,
    observer_lla: [f64; 3],
    observer_theta: f64,
    pub observed_satellite_id: Option<String>,
}

impl Observation {
    pub fn from_saal_files(sensor_file: &str, observation_file: &str) -> Result<Vec<Observation>, String> {
        sensor::load_file(sensor_file)?;
        obs::load_file(observation_file)?;
        let saal_obs = obs::parse_all()?;
        let saal_sensors = sensor::parse_all()?;
        let sensor_map: std::collections::HashMap<i32, sensor::ParsedSensor> =
            saal_sensors.into_iter().map(|s| (s.number, s)).collect();

        let mut observations: Vec<Observation> = Vec::new();
        for saal_ob in saal_obs {
            if !sensor_map.contains_key(&saal_ob.sensor_number) {
                log::warn!("Skipping observation from tracked sensor {}", saal_ob.sensor_number);
                continue;
            } else if saal_ob.position.is_none() {
                log::warn!(
                    "Skipping observation from sensor {} with missing position",
                    saal_ob.sensor_number
                );
                continue;
            } else {
                log::debug!(
                    "Processing type {} observation from sensor {}",
                    saal_ob.observation_type,
                    saal_ob.sensor_number
                );
            }

            let lla = astro::efg_to_lla(&saal_ob.position.unwrap())?;
            let saal_sensor = sensor_map.get(&saal_ob.sensor_number).unwrap();
            let observatory = Observatory::new(lla[0], lla[1], lla[2]);

            let epoch = Epoch::from_days_since_1950(saal_ob.epoch, TimeSystem::UTC);
            let range: Option<f64> = saal_ob.range;
            let range_rate: Option<f64> = saal_ob.range_rate;
            let mut right_ascension: f64 = 0.0;
            let mut declination: f64 = 0.0;
            let mut right_ascension_rate: Option<f64> = None;
            let mut declination_rate: Option<f64> = None;
            let mut angular_noise: f64 = configs::DEFAULT_ANGULAR_NOISE;
            let range_noise: Option<f64> = saal_sensor.range_noise;
            let range_rate_noise: Option<f64> = saal_sensor.range_rate_noise;
            let mut angular_rate_noise: Option<f64> = None;

            if saal_ob.right_ascension.is_some() && saal_ob.declination.is_some() {
                if saal_ob.year_of_equinox.unwrap() > 0 {
                    (right_ascension, declination) = astro::topo_meme_to_teme(
                        saal_ob.year_of_equinox.unwrap(),
                        saal_ob.epoch,
                        saal_ob.right_ascension.unwrap(),
                        saal_ob.declination.unwrap(),
                    );
                } else {
                    let mut out_ra = 0.0;
                    let mut out_dec = 0.0;
                    unsafe {
                        astro::RotRADecl(
                            106,
                            2,
                            epoch.days_since_1950,
                            saal_ob.right_ascension.unwrap(),
                            saal_ob.declination.unwrap(),
                            epoch.days_since_1950,
                            &mut out_ra,
                            &mut out_dec,
                        );
                    }
                    right_ascension = out_ra;
                    declination = out_dec;
                }
            } else if saal_ob.azimuth.is_some() && saal_ob.elevation.is_some() {
                let lst = epoch.get_gst() + lla[1].to_radians();
                let mut xa_rae = [0.0; astro::XA_RAE_SIZE];
                let sensor_teme: [f64; 3] = observatory.get_state_at_epoch(epoch).position.into();
                xa_rae[astro::XA_RAE_AZ] = saal_ob.azimuth.unwrap();
                xa_rae[astro::XA_RAE_EL] = saal_ob.elevation.unwrap();
                xa_rae[astro::XA_RAE_RANGE] = saal_ob.range.unwrap_or(1.0);
                xa_rae[astro::XA_RAE_RANGEDOT] = saal_ob.range_rate.unwrap_or(0.0);
                xa_rae[astro::XA_RAE_AZDOT] = saal_ob.azimuth_rate.unwrap_or(0.0);
                xa_rae[astro::XA_RAE_ELDOT] = saal_ob.elevation_rate.unwrap_or(0.0);
                let teme_ob = astro::horizon_to_teme(lst, lla[0], &sensor_teme, &xa_rae).unwrap();
                let xa_topo = astro::teme_to_topo(lst, lla[0], &sensor_teme, &teme_ob).unwrap();
                right_ascension = xa_topo[astro::XA_TOPO_RA];
                declination = xa_topo[astro::XA_TOPO_DEC];
                if saal_ob.azimuth_rate.is_some() && saal_ob.elevation_rate.is_some() {
                    right_ascension_rate = Some(xa_topo[astro::XA_TOPO_RADOT]);
                    declination_rate = Some(xa_topo[astro::XA_TOPO_DECDOT]);
                    angular_rate_noise = Some(DEFAULT_ANGULAR_RATE_NOISE);
                }
                if saal_sensor.azimuth_noise.is_some() && saal_sensor.elevation_noise.is_some() {
                    angular_noise = (saal_sensor.azimuth_noise.unwrap().powi(2)
                        + saal_sensor.elevation_noise.unwrap().powi(2))
                    .sqrt();
                }
                if saal_sensor.azimuth_rate_noise.is_some() && saal_sensor.elevation_rate_noise.is_some() {
                    angular_rate_noise = Some(
                        (saal_sensor.azimuth_rate_noise.unwrap().powi(2)
                            + saal_sensor.elevation_rate_noise.unwrap().powi(2))
                        .sqrt(),
                    );
                }
            }
            let mut ob_sensor = Sensor::new(angular_noise);
            let mut topo = TopocentricElements::new(right_ascension, declination);
            log::debug!(
                "Set right ascension to {:.6} ± {:.6} deg",
                right_ascension,
                angular_noise
            );
            log::debug!("Set declination to {:.6} ± {:.6} deg", declination, angular_noise);

            ob_sensor.id = saal_sensor.number.to_string();
            ob_sensor.name = saal_sensor.description.clone();

            if let (Some(r), Some(rn)) = (range, range_noise) {
                log::debug!("Set range to {:.3} ± {:.3} km", r, rn);
                ob_sensor.range_noise = Some(rn);
                topo.range = Some(r);
            }
            if let (Some(rr), Some(rrn)) = (range_rate, range_rate_noise) {
                log::debug!("Set range rate to {:.3} ± {:.3} km/s", rr, rrn);
                ob_sensor.range_rate_noise = Some(rrn);
                topo.range_rate = Some(rr);
            }
            if let Some(arn) = angular_rate_noise
                && (right_ascension_rate.is_some() || declination_rate.is_some())
            {
                log::debug!(
                    "Set right ascension rate to {:?} ± {:.3} deg/s",
                    right_ascension_rate,
                    arn
                );
                log::debug!("Set declination rate to {:?} ± {:.3} deg/s", declination_rate, arn);
                topo.right_ascension_rate = right_ascension_rate;
                topo.declination_rate = declination_rate;
                ob_sensor.angular_rate_noise = Some(arn);
            }

            let mut observation =
                Observation::new(ob_sensor, epoch, topo, observatory.get_state_at_epoch(epoch).position);
            observation.observed_satellite_id = Some(saal_ob.norad_id.to_string());
            observations.push(observation);
        }
        obs::clear();
        sensor::clear()?;
        Ok(observations)
    }

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
                "Error propagating satellite {} to {}: {}",
                satellite.id,
                self.get_epoch().to_iso(),
                get_last_error_message()
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

    pub fn get_observer_position(&self) -> CartesianVector {
        self.observer_teme_position
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

    pub fn get_association(&self, satellite: &Satellite) -> Option<ObservationAssociation> {
        if let Some(residual) = self.get_residual(satellite) {
            let confidence = if residual.get_range() < HIGH_ASSOCIATION_CLOS_RANGE {
                AssociationConfidence::High
            } else if residual.get_range() < MEDIUM_ASSOCIATION_CLOS_RANGE {
                AssociationConfidence::Medium
            } else if residual.get_range() < LOW_ASSOCIATION_CLOS_RANGE {
                AssociationConfidence::Low
            } else {
                return None;
            };
            Some(ObservationAssociation::new(
                self.id.clone(),
                satellite.id.clone(),
                residual,
                confidence,
            ))
        } else {
            None
        }
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
            Some(satellite_state) => self.get_residual_from_state(&satellite_state),
            None => None,
        }
    }

    pub fn get_residual_from_state(&self, satellite_state: &CartesianState) -> Option<ObservationResidual> {
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

    pub fn get_association_from_state(
        &self,
        satellite_id: &str,
        satellite_state: &CartesianState,
    ) -> Option<ObservationAssociation> {
        if let Some(residual) = self.get_residual_from_state(satellite_state) {
            let confidence = if residual.get_range() < HIGH_ASSOCIATION_CLOS_RANGE {
                AssociationConfidence::High
            } else if residual.get_range() < MEDIUM_ASSOCIATION_CLOS_RANGE {
                AssociationConfidence::Medium
            } else if residual.get_range() < LOW_ASSOCIATION_CLOS_RANGE {
                AssociationConfidence::Low
            } else {
                return None;
            };
            Some(ObservationAssociation::new(
                self.id.clone(),
                satellite_id.to_string(),
                residual,
                confidence,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_saal_files() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let observations_result = Observation::from_saal_files("tests/sensors.dat", "tests/test-b3-obs.txt");
        assert!(observations_result.is_ok());
        let observations = observations_result.unwrap();
        assert_eq!(observations.len(), 5053);
    }
}
