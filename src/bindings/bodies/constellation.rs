use super::{PyObservatory, PySatellite};
use crate::bindings::catalogs::PyTLECatalog;
use crate::bindings::elements::{PyCartesianState, PyEphemeris, PyOrbitPlotData};
use crate::bindings::estimation::{PyCollectionAssociationReport, PyObservation, PyObservationCollection};
use crate::bindings::events::{
    PyCloseApproachReport, PyCrossTagReport, PyHorizonAccessReport, PyManeuverReport, PyProximityReport,
};
use crate::bindings::propagation::PyPropagationBackend;
use crate::bindings::time::{PyEpoch, PyTimeSpan};
use crate::bodies::{Constellation, Satellite};
use crate::catalogs::TLECatalog;
use crate::estimation::ObservationCollection;
use crate::time::{Epoch, TimeSpan};
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::HashMap;

#[pyclass(name = "Constellation")]
#[derive(Default, Debug, Clone)]
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
        py: Python<'_>,
        site: &PyObservatory,
        start: PyEpoch,
        end: PyEpoch,
        min_el: f64,
        min_duration: PyTimeSpan,
    ) -> PyHorizonAccessReport {
        let min_duration: TimeSpan = min_duration.into();
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        py.detach(|| {
            PyHorizonAccessReport::from(self.inner.get_horizon_access_report(
                site.inner(),
                start,
                end,
                min_el,
                min_duration,
            ))
        })
    }

    pub fn get_ca_report_vs_one(
        &mut self,
        py: Python<'_>,
        sat: &mut PySatellite,
        start: PyEpoch,
        end: PyEpoch,
        distance_threshold: f64,
    ) -> PyCloseApproachReport {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        py.detach(|| {
            PyCloseApproachReport::from(self.inner.get_ca_report_vs_one(
                sat.inner_mut(),
                start,
                end,
                distance_threshold,
            ))
        })
    }

    pub fn get_ca_report_vs_many(
        &mut self,
        py: Python<'_>,
        start: PyEpoch,
        end: PyEpoch,
        distance_threshold: f64,
    ) -> PyCloseApproachReport {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        py.detach(|| PyCloseApproachReport::from(self.inner.get_ca_report_vs_many(start, end, distance_threshold)))
    }

    pub fn get_proximity_report_vs_one(
        &mut self,
        py: Python<'_>,
        sat: &mut PySatellite,
        start: PyEpoch,
        end: PyEpoch,
        distance_threshold: f64,
    ) -> PyProximityReport {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        py.detach(|| {
            PyProximityReport::from(self.inner.get_proximity_report_vs_one(
                sat.inner_mut(),
                start,
                end,
                distance_threshold,
            ))
        })
    }

    pub fn get_proximity_report_vs_many(
        &mut self,
        py: Python<'_>,
        start: PyEpoch,
        end: PyEpoch,
        distance_threshold: f64,
    ) -> PyProximityReport {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        py.detach(|| PyProximityReport::from(self.inner.get_proximity_report_vs_many(start, end, distance_threshold)))
    }

    pub fn detect_cross_tags(
        &mut self,
        py: Python<'_>,
        uct: &mut PySatellite,
        observations: Vec<PyObservation>,
        start: PyEpoch,
        end: PyEpoch,
        proximity_threshold: Option<f64>,
        confidence_threshold: Option<f64>,
    ) -> PyCrossTagReport {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        let observations: Vec<crate::estimation::Observation> = observations.into_iter().map(|o| o.into()).collect();
        py.detach(|| {
            PyCrossTagReport::from(self.inner.detect_cross_tags(
                uct.inner_mut(),
                &observations,
                start,
                end,
                proximity_threshold,
                confidence_threshold,
            ))
        })
    }

    pub fn get_maneuver_events(
        &mut self,
        py: Python<'_>,
        future_sats: &mut PyConstellation,
        start: PyEpoch,
        end: PyEpoch,
        distance_threshold: f64,
        velocity_threshold: f64,
    ) -> PyManeuverReport {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        py.detach(|| {
            PyManeuverReport::from(self.inner.get_maneuver_events(
                &mut future_sats.inner,
                start,
                end,
                distance_threshold,
                velocity_threshold,
            ))
        })
    }

    pub fn get_ephemeris(
        &mut self,
        py: Python<'_>,
        start_epoch: PyEpoch,
        end_epoch: PyEpoch,
        step_size: PyTimeSpan,
    ) -> HashMap<String, Option<PyEphemeris>> {
        let step_size: TimeSpan = step_size.into();
        let start_epoch: Epoch = start_epoch.into();
        let end_epoch: Epoch = end_epoch.into();
        py.detach(|| {
            self.inner
                .get_ephemeris(start_epoch, end_epoch, step_size)
                .into_iter()
                .map(|(id, ephem)| (id, ephem.map(PyEphemeris::from)))
                .collect()
        })
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

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let keys = self.inner.get_keys();
        let list = PyList::new(py, keys)?;
        list.call_method0("__iter__")
    }

    fn __contains__(&self, key: &str) -> bool {
        self.inner.get(key.to_string()).is_some()
    }

    fn __len__(&self) -> usize {
        self.inner.get_count()
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

    /// Get states at multiple epochs using batch propagation (GPU-accelerated when available)
    ///
    /// # Arguments
    /// * `epochs` - List of epochs to propagate to
    /// * `backend` - Optional backend selection (Auto, Cpu, or Gpu)
    ///
    /// # Returns
    /// Dictionary mapping satellite ID to list of states (one per epoch)
    #[pyo3(signature = (epochs, backend = None))]
    pub fn get_states_at_epochs(
        &self,
        py: Python<'_>,
        epochs: Vec<PyEpoch>,
        backend: Option<PyPropagationBackend>,
    ) -> HashMap<String, Vec<Option<PyCartesianState>>> {
        let epochs: Vec<Epoch> = epochs.into_iter().map(|e| e.into()).collect();
        let backend = backend.map(|b| b.into());

        py.detach(|| {
            self.inner
                .get_states_at_epochs(&epochs, backend)
                .into_iter()
                .map(|(id, states)| {
                    let py_states = states
                        .into_iter()
                        .map(|opt_state| opt_state.map(PyCartesianState::from))
                        .collect();
                    (id, py_states)
                })
                .collect()
        })
    }

    /// Get batch ephemeris for all satellites
    ///
    /// # Arguments
    /// * `start` - Start epoch
    /// * `end` - End epoch
    /// * `step` - Time step between epochs
    /// * `backend` - Optional backend selection (Auto, Cpu, or Gpu)
    ///
    /// # Returns
    /// Dictionary mapping satellite ID to list of states
    #[pyo3(signature = (start, end, step, backend = None))]
    pub fn get_batch_ephemeris(
        &self,
        py: Python<'_>,
        start: PyEpoch,
        end: PyEpoch,
        step: PyTimeSpan,
        backend: Option<PyPropagationBackend>,
    ) -> HashMap<String, Vec<Option<PyCartesianState>>> {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        let step: TimeSpan = step.into();
        let backend = backend.map(|b| b.into());

        py.detach(|| {
            self.inner
                .get_batch_ephemeris(start, end, step, backend)
                .into_iter()
                .map(|(id, states)| {
                    let py_states = states
                        .into_iter()
                        .map(|opt_state| opt_state.map(PyCartesianState::from))
                        .collect();
                    (id, py_states)
                })
                .collect()
        })
    }

    /// Check if GPU acceleration is available
    #[staticmethod]
    pub fn is_gpu_available() -> bool {
        Constellation::is_gpu_available()
    }

    /// Cache ephemeris for all satellites in the constellation
    ///
    /// # Arguments
    /// * `start` - Start epoch for ephemeris caching
    /// * `end` - End epoch for ephemeris caching
    /// * `step` - Time step between cached states
    pub fn cache_ephemeris(&mut self, py: Python<'_>, start: PyEpoch, end: PyEpoch, step: PyTimeSpan) {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        let step: TimeSpan = step.into();
        py.detach(|| {
            self.inner.cache_ephemeris(start, end, step);
        });
    }

    /// Get association reports for multiple observation collections
    ///
    /// # Arguments
    /// * `collections` - List of observation collections to find associations for
    ///
    /// # Returns
    /// List of CollectionAssociationReport, one per input collection
    pub fn get_association_reports(
        &self,
        py: Python<'_>,
        collections: Vec<PyObservationCollection>,
    ) -> Vec<PyCollectionAssociationReport> {
        let collections: Vec<ObservationCollection> = collections.into_iter().map(|c| c.inner().clone()).collect();
        py.detach(|| {
            self.inner
                .get_association_reports(&collections)
                .into_iter()
                .map(PyCollectionAssociationReport::from)
                .collect()
        })
    }
}
