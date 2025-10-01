use super::Satellite;
use crate::bodies::Observatory;
use crate::catalogs::TLECatalog;
use crate::configs;
use crate::elements::{CartesianState, Ephemeris, OrbitPlotData};
use crate::events::{CloseApproachReport, HorizonAccessReport};
use crate::time::{Epoch, TimeSpan};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

#[pyclass]
#[derive(Default)]
pub struct Constellation {
    name: Option<String>,
    satellites: HashMap<String, Satellite>,
}

#[pymethods]
impl Constellation {
    #[new]
    pub fn new() -> Self {
        Constellation {
            name: None,
            satellites: HashMap::new(),
        }
    }

    #[staticmethod]
    pub fn from_tle_catalog(catalog: TLECatalog) -> Self {
        let mut constellation = Constellation::new();
        for satellite_id in catalog.keys() {
            if let Some(tle) = catalog.get(satellite_id.clone()) {
                let sat = Satellite::from_tle(tle);
                constellation.add(satellite_id, sat);
            }
        }
        constellation.name = catalog.get_name();
        constellation
    }

    pub fn get_states_at_epoch(&self, epoch: Epoch) -> HashMap<String, Option<CartesianState>> {
        self.satellites
            .par_iter()
            .map(|(satellite_id, sat)| {
                let state = sat.get_state_at_epoch(epoch);
                (satellite_id.clone(), state)
            })
            .collect()
    }

    pub fn get_plot_data(&self, start: Epoch, end: Epoch, step: TimeSpan) -> HashMap<String, OrbitPlotData> {
        self.satellites
            .par_iter()
            .filter_map(|(satellite_id, sat)| {
                sat.get_plot_data(start, end, step)
                    .map(|plot_data| (satellite_id.clone(), plot_data))
            })
            .collect()
    }

    pub fn step_to_epoch(&mut self, epoch: Epoch) -> Constellation {
        let sat_map = self
            .satellites
            .par_iter_mut()
            .filter_map(|(sat_id, sat)| match sat.step_to_epoch(epoch) {
                Ok(_) => Some((sat_id.clone(), sat.clone())),
                Err(_) => None,
            })
            .collect();
        let mut new_constellation = Constellation::new();
        new_constellation.satellites = sat_map;
        new_constellation.name = self.name.clone();
        new_constellation
    }

    pub fn get_horizon_access_report(
        &self,
        site: &Observatory,
        start: Epoch,
        end: Epoch,
        min_el: f64,
        min_duration: TimeSpan,
    ) -> HorizonAccessReport {
        // get TEME states for site
        let site_ephem = site.get_ephemeris(start, end, min_duration);

        // get TEME states for all satellites
        let sat_ephem_list: Vec<Ephemeris> = self
            .satellites
            .par_iter()
            .filter_map(|(_, sat)| sat.get_ephemeris(start, end, min_duration))
            .collect();

        // create empty report
        let mut report = HorizonAccessReport::new(start, end, min_el, min_duration);

        // parallelize the access report generation
        let num = sat_ephem_list.len();
        let accesses = (0..num)
            .into_par_iter()
            .filter_map(|i| {
                let sat_ephem = &sat_ephem_list[i];
                sat_ephem.get_horizon_accesses(&site_ephem, min_el, min_duration)
            })
            .collect::<Vec<_>>();

        report.set_accesses(accesses.into_iter().flatten().collect());
        report
    }

    pub fn get_ca_report_vs_one(
        &self,
        sat: &Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: f64,
    ) -> CloseApproachReport {
        match sat.get_ephemeris(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES)) {
            Some(ephemeris) => {
                let close_approaches = self
                    .satellites
                    .par_iter()
                    .filter_map(|(_, other_sat)| {
                        if sat.get_apoapsis()? < other_sat.get_periapsis()? - distance_threshold
                            || other_sat.get_apoapsis()? < sat.get_periapsis()? - distance_threshold
                            || sat.get_periapsis()? > other_sat.get_apoapsis()? + distance_threshold
                            || other_sat.get_periapsis()? > sat.get_apoapsis()? + distance_threshold
                        {
                            return None;
                        }
                        match other_sat.get_ephemeris(
                            start,
                            end,
                            TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES),
                        ) {
                            Some(other_ephemeris) => ephemeris.get_close_approach(&other_ephemeris, distance_threshold),

                            None => None,
                        }
                    })
                    .collect();
                let mut report = CloseApproachReport::new(start, end, distance_threshold);
                report.set_close_approaches(close_approaches);
                report
            }
            None => CloseApproachReport::new(start, end, distance_threshold),
        }
    }

    pub fn get_ca_report_vs_many(&self, start: Epoch, end: Epoch, distance_threshold: f64) -> CloseApproachReport {
        let mut report = CloseApproachReport::new(start, end, distance_threshold);
        let ephem_list: Vec<Ephemeris> = self
            .satellites
            .par_iter()
            .filter_map(|(_, sat)| {
                sat.get_ephemeris(start, end, TimeSpan::from_minutes(configs::CONJUNCTION_STEP_MINUTES))
            })
            .collect();
        let num = ephem_list.len();
        let close_approaches = (0..num)
            .into_par_iter()
            .flat_map(|i| {
                let pri_ephem = &ephem_list[i];
                let pri_sat = &self.satellites.get(&pri_ephem.get_satellite_id()).unwrap();
                (i + 1..num)
                    .into_par_iter()
                    .filter_map(|j| {
                        let sec_ephem = &ephem_list[j];
                        let sec_sat = &self.satellites.get(&sec_ephem.get_satellite_id()).unwrap();
                        if pri_sat.get_apoapsis()? < sec_sat.get_periapsis()? - distance_threshold
                            || sec_sat.get_apoapsis()? < pri_sat.get_periapsis()? - distance_threshold
                            || pri_sat.get_periapsis()? > sec_sat.get_apoapsis()? + distance_threshold
                            || sec_sat.get_periapsis()? > pri_sat.get_apoapsis()? + distance_threshold
                        {
                            return None;
                        }
                        pri_ephem.get_close_approach(sec_ephem, distance_threshold)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        report.set_close_approaches(close_approaches);
        report
    }

    pub fn get_ephemeris(
        &self,
        start_epoch: Epoch,
        end_epoch: Epoch,
        step_size: TimeSpan,
    ) -> HashMap<String, Option<Ephemeris>> {
        self.satellites
            .par_iter()
            .map(|(satellite_id, sat)| {
                let ephemeris = sat.get_ephemeris(start_epoch, end_epoch, step_size);
                (satellite_id.clone(), ephemeris)
            })
            .collect()
    }

    fn __getitem__(&self, satellite_id: String) -> PyResult<Satellite> {
        match self.get(satellite_id.clone()) {
            Some(sat) => Ok(sat),
            None => Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "Invalid key: {}",
                satellite_id
            ))),
        }
    }

    fn keys(&self) -> Vec<String> {
        self.satellites.keys().cloned().collect()
    }

    fn __setitem__(&mut self, satellite_id: String, state: Satellite) {
        self.satellites.insert(satellite_id, state);
    }

    pub fn add(&mut self, satellite_id: String, sat: Satellite) {
        self.satellites.insert(satellite_id, sat);
    }

    pub fn get(&self, satellite_id: String) -> Option<Satellite> {
        self.satellites.get(&satellite_id).cloned()
    }

    pub fn remove(&mut self, satellite_id: String) {
        self.satellites.remove(&satellite_id);
    }

    pub fn clear(&mut self) {
        self.satellites.clear();
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    #[setter]
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    #[getter]
    pub fn get_count(&self) -> usize {
        self.satellites.len()
    }
}
