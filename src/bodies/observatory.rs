use super::{Constellation, Satellite, Sensor};
use crate::configs::MIN_EPHEMERIS_POINTS;
use crate::elements::{CartesianState, CartesianVector, Ephemeris, TopocentricElements};
use crate::enums::ReferenceFrame;
use crate::events::{FieldOfViewCandidate, FieldOfViewReport};
use crate::time::{Epoch, TimeSpan};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use saal::astro;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Observatory {
    pub id: String,
    pub name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub sensors: Vec<Sensor>,
}

impl From<CartesianState> for Observatory {
    fn from(state: CartesianState) -> Self {
        let theta_g = state.epoch.to_fk5_greenwich_angle();
        let teme = state.to_frame(ReferenceFrame::TEME);
        let lla = astro::gst_teme_to_lla(theta_g, &teme.position.into());
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            latitude: lla[0],
            longitude: lla[1],
            altitude: lla[2],
            sensors: Vec::new(),
        }
    }
}

impl Observatory {
    pub fn get_ephemeris(&self, start_epoch: Epoch, end_epoch: Epoch, step: TimeSpan) -> Ephemeris {
        let ephemeris = Ephemeris::new(self.id.clone(), None, self.get_state_at_epoch(start_epoch)).unwrap();
        let diff = end_epoch - start_epoch;
        let max_step = TimeSpan::from_minutes(diff.in_minutes() / MIN_EPHEMERIS_POINTS as f64);
        let dt = if step < max_step { step } else { max_step };
        let mut next_epoch: Epoch = start_epoch + dt;

        while next_epoch <= end_epoch {
            ephemeris.add_state(self.get_state_at_epoch(next_epoch)).unwrap();
            next_epoch += dt;
        }
        ephemeris
    }

    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            latitude,
            longitude,
            altitude,
            sensors: Vec::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    pub fn get_topocentric_to_satellite(
        &self,
        epoch: Epoch,
        sat: &Satellite,
        reference_frame: ReferenceFrame,
    ) -> Result<TopocentricElements, String> {
        let observer_position = self.get_state_at_epoch(epoch).position;
        if let Some(sat_state) = sat.get_state_at_epoch(epoch) {
            let theta_g = epoch.to_fk5_greenwich_angle();
            let lla = [self.latitude, self.longitude, self.altitude];
            let topo = astro::teme_to_topo(
                theta_g + lla[1].to_radians(),
                lla[0],
                &observer_position.into(),
                &sat_state.into(),
            )
            .unwrap();
            let mut elements = TopocentricElements::new(topo[astro::XA_TOPO_RA], topo[astro::XA_TOPO_DEC]);
            if reference_frame == ReferenceFrame::J2000 {
                let (ra, dec) = astro::topo_teme_to_meme(
                    astro::YROFEQNX_2000 as i32,
                    epoch.days_since_1950,
                    elements.right_ascension,
                    elements.declination,
                );
                elements.right_ascension = ra;
                elements.declination = dec;
            }
            elements.range = Some(topo[astro::XA_TOPO_RANGE]);
            elements.range_rate = Some(topo[astro::XA_TOPO_RANGEDOT]);
            elements.right_ascension_rate = Some(topo[astro::XA_TOPO_RADOT]);
            elements.declination_rate = Some(topo[astro::XA_TOPO_DECDOT]);
            Ok(elements)
        } else {
            Err(format!(
                "Unable to calculate topocentric elements from observatory {} to satellite {} at epoch {:?}",
                self.id, sat.id, epoch
            ))
        }
    }

    pub fn get_field_of_view_report(
        &self,
        epoch: Epoch,
        sensor_direction: TopocentricElements,
        angular_threshold: f64,
        sats: Constellation,
        reference_frame: ReferenceFrame,
    ) -> FieldOfViewReport {
        let observer_position = self.get_state_at_epoch(epoch).position;
        let mut report = FieldOfViewReport::new(
            epoch,
            observer_position,
            &sensor_direction,
            angular_threshold,
            reference_frame,
        );
        let teme_direction = sensor_direction.get_observed_direction();
        let theta_g = epoch.to_fk5_greenwich_angle();

        let candidates: Vec<FieldOfViewCandidate> = sats
            .get_satellites()
            .par_iter()
            .filter_map(|(sat_id, sat)| {
                if let Some(sat_state) = sat.get_state_at_epoch(epoch) {
                    let relative_position = sat_state.position - observer_position;
                    let angle = teme_direction.angle(&relative_position).to_degrees();
                    if angle <= angular_threshold {
                        let topo = astro::teme_to_topo(
                            theta_g + self.longitude.to_radians(),
                            self.latitude,
                            &observer_position.into(),
                            &sat_state.into(),
                        )
                        .unwrap();
                        let mut elements = TopocentricElements::new(topo[astro::XA_TOPO_RA], topo[astro::XA_TOPO_DEC]);
                        if reference_frame == ReferenceFrame::J2000 {
                            let (ra, dec) = astro::topo_teme_to_meme(
                                astro::YROFEQNX_2000 as i32,
                                epoch.days_since_1950,
                                elements.right_ascension,
                                elements.declination,
                            );
                            elements.right_ascension = ra;
                            elements.declination = dec;
                        }
                        elements.range = Some(topo[astro::XA_TOPO_RANGE]);
                        elements.range_rate = Some(topo[astro::XA_TOPO_RANGEDOT]);
                        elements.right_ascension_rate = Some(topo[astro::XA_TOPO_RADOT]);
                        elements.declination_rate = Some(topo[astro::XA_TOPO_DECDOT]);

                        Some(FieldOfViewCandidate::new(sat_id.clone(), &elements))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        report.set_candidates(candidates);
        report
    }

    pub fn get_state_at_epoch(&self, epoch: Epoch) -> CartesianState {
        let teme_pos = astro::lla_to_teme(epoch.days_since_1950, &[self.latitude, self.longitude, self.altitude]);
        CartesianState::new(
            epoch,
            CartesianVector::from(teme_pos),
            CartesianVector::from([0.0, 0.0, 0.0]),
            ReferenceFrame::TEME,
        )
    }

    pub fn get_theta(&self, epoch: Epoch) -> f64 {
        let theta_g = epoch.to_fk5_greenwich_angle();
        theta_g + self.longitude.to_radians()
    }
}
