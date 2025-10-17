use super::{Constellation, Satellite, Sensor};
use crate::configs::MIN_EPHEMERIS_POINTS;
use crate::elements::{CartesianState, CartesianVector, Ephemeris, TopocentricElements};
use crate::enums::ReferenceFrame;
use crate::events::{FieldOfViewCandidate, FieldOfViewReport};
use crate::exceptions::SAALError;
use crate::saal::astro_func_interface;
use crate::time::{Epoch, TimeSpan};
use pyo3::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use uuid::Uuid;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct Observatory {
    id: String,
    name: Option<String>,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    sensors: Vec<Sensor>,
}

impl Observatory {
    pub fn get_ephemeris(&self, start_epoch: Epoch, end_epoch: Epoch, step: TimeSpan) -> Ephemeris {
        let ephemeris = Ephemeris::new(self.id.clone(), None, self.get_state_at_epoch(start_epoch));
        let diff = end_epoch - start_epoch;
        let max_step = TimeSpan::from_minutes(diff.in_minutes() / MIN_EPHEMERIS_POINTS as f64);
        let dt = if step < max_step { step } else { max_step };
        let mut next_epoch: Epoch = start_epoch + dt;

        while next_epoch <= end_epoch {
            ephemeris.add_state(self.get_state_at_epoch(next_epoch));
            next_epoch += dt;
        }
        ephemeris
    }
}

#[pymethods]
impl Observatory {
    #[new]
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

    #[staticmethod]
    pub fn from_cartesian_state(state: CartesianState) -> Self {
        let theta_g = state.get_epoch().to_fk5_greenwich_angle();
        let teme = state.to_frame(ReferenceFrame::TEME);
        let lla = astro_func_interface::theta_teme_to_lla(theta_g, &teme.position.into());
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            latitude: lla[0],
            longitude: lla[1],
            altitude: lla[2],
            sensors: Vec::new(),
        }
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    #[getter]
    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    #[getter]
    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    #[getter]
    pub fn get_altitude(&self) -> f64 {
        self.altitude
    }

    #[getter]
    pub fn get_sensors(&self) -> Vec<Sensor> {
        self.sensors.clone()
    }

    #[setter]
    pub fn set_id(&mut self, site_id: String) {
        self.id = site_id;
    }

    #[setter]
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    #[setter]
    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    #[setter]
    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    #[setter]
    pub fn set_altitude(&mut self, altitude: f64) {
        self.altitude = altitude;
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    pub fn get_topocentric_to_satellite(
        &self,
        epoch: Epoch,
        sat: &Satellite,
        reference_frame: ReferenceFrame,
    ) -> PyResult<TopocentricElements> {
        let observer_position = self.get_state_at_epoch(epoch).position;
        if let Some(sat_state) = sat.get_state_at_epoch(epoch) {
            let theta_g = epoch.to_fk5_greenwich_angle();
            let lla = astro_func_interface::theta_teme_to_lla(theta_g, &observer_position.into());
            let topo = astro_func_interface::teme_to_topo(
                theta_g + lla[1].to_radians(),
                lla[0],
                &observer_position.into(),
                &sat_state.position.into(),
                &sat_state.velocity.into(),
            );
            let mut elements = TopocentricElements::new(
                topo[astro_func_interface::XA_TOPO_RA],
                topo[astro_func_interface::XA_TOPO_DEC],
            );
            if reference_frame == ReferenceFrame::J2000 {
                let (ra, dec) = astro_func_interface::topo_date_to_equinox(
                    astro_func_interface::YROFEQNX_2000 as i32,
                    epoch.days_since_1950,
                    elements.get_right_ascension(),
                    elements.get_declination(),
                );
                elements.set_right_ascension(ra);
                elements.set_declination(dec);
            }
            elements.set_range(Some(topo[astro_func_interface::XA_TOPO_RANGE]));
            elements.set_range_rate(Some(topo[astro_func_interface::XA_TOPO_RANGEDOT]));
            elements.set_right_ascension_rate(Some(topo[astro_func_interface::XA_TOPO_RADOT]));
            elements.set_declination_rate(Some(topo[astro_func_interface::XA_TOPO_DECDOT]));
            Ok(elements)
        } else {
            Err(PyErr::new::<SAALError, _>(format!(
                "Satellite state not available at epoch {}",
                epoch.to_iso()
            )))
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
        let lla = astro_func_interface::theta_teme_to_lla(theta_g, &observer_position.into());
        let candidates: Vec<FieldOfViewCandidate> = sats
            .get_satellites()
            .par_iter()
            .filter_map(|(sat_id, sat)| {
                if let Some(sat_state) = sat.get_state_at_epoch(epoch) {
                    let relative_position = sat_state.position - observer_position;
                    let angle = teme_direction.angle(&relative_position).to_degrees();
                    if angle <= angular_threshold {
                        let topo = astro_func_interface::teme_to_topo(
                            theta_g + lla[1].to_radians(),
                            lla[0],
                            &observer_position.into(),
                            &sat_state.position.into(),
                            &sat_state.velocity.into(),
                        );
                        let mut elements = TopocentricElements::new(
                            topo[astro_func_interface::XA_TOPO_RA],
                            topo[astro_func_interface::XA_TOPO_DEC],
                        );
                        if reference_frame == ReferenceFrame::J2000 {
                            let (ra, dec) = astro_func_interface::topo_date_to_equinox(
                                astro_func_interface::YROFEQNX_2000 as i32,
                                epoch.days_since_1950,
                                elements.get_right_ascension(),
                                elements.get_declination(),
                            );
                            elements.set_right_ascension(ra);
                            elements.set_declination(dec);
                        }
                        elements.set_range(Some(topo[astro_func_interface::XA_TOPO_RANGE]));
                        elements.set_range_rate(Some(topo[astro_func_interface::XA_TOPO_RANGEDOT]));
                        elements.set_right_ascension_rate(Some(topo[astro_func_interface::XA_TOPO_RADOT]));
                        elements.set_declination_rate(Some(topo[astro_func_interface::XA_TOPO_DECDOT]));

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
        let teme_pos = astro_func_interface::lla_to_teme_position(
            epoch.days_since_1950,
            &[self.latitude, self.longitude, self.altitude],
        );
        CartesianState::new(
            epoch,
            CartesianVector::from(teme_pos),
            CartesianVector::from([0.0, 0.0, 0.0]),
            ReferenceFrame::TEME,
        )
    }
}
