use super::{PyObservatory, PySatellite};
use crate::bindings::catalogs::PyTLECatalog;
use crate::bindings::elements::{PyCartesianState, PyEphemeris, PyOrbitPlotData};
use crate::bindings::events::{PyCloseApproachReport, PyHorizonAccessReport};
use crate::bindings::time::{PyEpoch, PyTimeSpan};
use crate::bodies::{Constellation, Satellite};
use crate::catalogs::TLECatalog;
use crate::time::{Epoch, TimeSpan};
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass(name = "Constellation")]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PyConstellation {
    inner: Constellation,
}

impl From<Constellation> for PyConstellation {
    fn from(inner: Constellation) -> Self {
        Self { inner }
    }
}

impl From<PyConstellation> for Constellation {
    fn from(value: PyConstellation) -> Self {
        value.inner
    }
}

impl PyConstellation {
    pub fn inner(&self) -> &Constellation {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Constellation {
        &mut self.inner
    }

    pub fn get_satellites(&self) -> &HashMap<String, Satellite> {
        self.inner.get_satellites()
    }
}

#[pymethods]
impl PyConstellation {
    #[new]
    pub fn new() -> Self {
        Constellation::new().into()
    }

    #[staticmethod]
    pub fn from_tle_catalog(catalog: PyTLECatalog) -> Self {
        Constellation::from(TLECatalog::from(catalog)).into()
    }

    pub fn get_states_at_epoch(&self, epoch: PyEpoch) -> HashMap<String, Option<PyCartesianState>> {
        let epoch: Epoch = epoch.into();
        self.inner
            .get_states_at_epoch(epoch)
            .into_iter()
            .map(|(id, state)| (id, state.map(PyCartesianState::from)))
            .collect()
    }

    pub fn get_plot_data(&self, start: PyEpoch, end: PyEpoch, step: PyTimeSpan) -> HashMap<String, PyOrbitPlotData> {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        let step: TimeSpan = step.into();
        self.inner
            .get_plot_data(start, end, step)
            .into_iter()
            .map(|(id, data)| (id, PyOrbitPlotData::from(data)))
            .collect()
    }

    pub fn step_to_epoch(&mut self, epoch: PyEpoch) -> PyConstellation {
        let epoch: Epoch = epoch.into();
        self.inner.step_to_epoch(epoch).into()
    }

    pub fn get_horizon_access_report(
        &mut self,
        site: &PyObservatory,
        start: PyEpoch,
        end: PyEpoch,
        min_el: f64,
        min_duration: PyTimeSpan,
    ) -> PyHorizonAccessReport {
        let min_duration: TimeSpan = min_duration.into();
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        PyHorizonAccessReport::from(self.inner.get_horizon_access_report(
            site.inner(),
            start,
            end,
            min_el,
            min_duration,
        ))
    }

    pub fn get_ca_report_vs_one(
        &mut self,
        sat: &mut PySatellite,
        start: PyEpoch,
        end: PyEpoch,
        distance_threshold: f64,
    ) -> PyCloseApproachReport {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        PyCloseApproachReport::from(
            self.inner
                .get_ca_report_vs_one(sat.inner_mut(), start, end, distance_threshold),
        )
    }

    pub fn get_ca_report_vs_many(
        &mut self,
        start: PyEpoch,
        end: PyEpoch,
        distance_threshold: f64,
    ) -> PyCloseApproachReport {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        PyCloseApproachReport::from(self.inner.get_ca_report_vs_many(start, end, distance_threshold))
    }

    pub fn get_ephemeris(
        &mut self,
        start_epoch: PyEpoch,
        end_epoch: PyEpoch,
        step_size: PyTimeSpan,
    ) -> HashMap<String, Option<PyEphemeris>> {
        let step_size: TimeSpan = step_size.into();
        let start_epoch: Epoch = start_epoch.into();
        let end_epoch: Epoch = end_epoch.into();
        self.inner
            .get_ephemeris(start_epoch, end_epoch, step_size)
            .into_iter()
            .map(|(id, ephem)| (id, ephem.map(PyEphemeris::from)))
            .collect()
    }

    fn __getitem__(&self, satellite_id: String) -> PyResult<PySatellite> {
        match self.get(satellite_id.clone()) {
            Some(sat) => Ok(sat),
            None => Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "Invalid key: {}",
                satellite_id
            ))),
        }
    }

    fn keys(&self) -> Vec<String> {
        self.inner.get_keys()
    }

    fn __setitem__(&mut self, satellite_id: String, state: PySatellite) {
        self.inner.add(satellite_id, state.into());
    }

    pub fn add(&mut self, satellite_id: String, sat: PySatellite) {
        self.inner.add(satellite_id, sat.into());
    }

    pub fn get(&self, satellite_id: String) -> Option<PySatellite> {
        self.inner.get(satellite_id).map(PySatellite::from)
    }

    pub fn remove(&mut self, satellite_id: String) {
        self.inner.remove(satellite_id);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[setter]
    pub fn set_name(&mut self, name: Option<String>) {
        self.inner.name = name;
    }

    #[getter]
    pub fn get_count(&self) -> usize {
        self.inner.get_count()
    }
}
