use super::PyObservatory;
use crate::bindings::elements::{
    PyBoreToBodyAngles, PyCartesianState, PyEphemeris, PyGeodeticPosition, PyKeplerianState, PyOrbitPlotData,
    PyRelativeState, PyTLE,
};
use crate::bindings::estimation::{
    PyObservation, PyObservationAssociation, PyObservationCollection, PyObservationResidual,
};
use crate::bindings::events::{PyCloseApproach, PyHorizonAccessReport, PyManeuverEvent, PyProximityReport};
use crate::bindings::propagation::PyForceProperties;
use crate::bindings::time::{PyEpoch, PyTimeSpan};
use crate::bodies::{Observatory, Satellite};
use crate::elements::{Ephemeris, TLE};
use crate::estimation::Observation;
use crate::propagation::ForceProperties;
use crate::time::{Epoch, TimeSpan};
use nalgebra::{DMatrix, DVector};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "Satellite", subclass)]
#[derive(Debug, Clone)]
pub struct PySatellite {
    inner: Satellite,
}

impl From<Satellite> for PySatellite {
    fn from(inner: Satellite) -> Self {
        Self { inner }
    }
}

impl From<PySatellite> for Satellite {
    fn from(value: PySatellite) -> Self {
        value.inner
    }
}

impl PySatellite {
    pub fn inner(&self) -> &Satellite {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Satellite {
        &mut self.inner
    }

    pub fn get_jacobian(&self, ob: &Observation, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        self.inner.get_jacobian(ob, use_drag, use_srp)
    }

    pub fn build_perturbed_satellites(&self, use_drag: bool, use_srp: bool) -> Result<Vec<(Satellite, f64)>, String> {
        self.inner.build_perturbed_satellites(use_drag, use_srp)
    }

    pub fn clone_at_epoch(&self, epoch: PyEpoch) -> Result<Self, String> {
        let epoch: Epoch = epoch.into();
        self.inner.clone_at_epoch(epoch).map(PySatellite::from)
    }

    pub fn get_prior_node(&self, epoch: PyEpoch) -> Result<PyEpoch, String> {
        let epoch: Epoch = epoch.into();
        self.inner.get_prior_node(epoch).map(PyEpoch::from)
    }

    pub fn new_with_delta_x(&self, delta_x: &DVector<f64>, use_drag: bool, use_srp: bool) -> Result<Self, String> {
        self.inner
            .new_with_delta_x(delta_x, use_drag, use_srp)
            .map(PySatellite::from)
    }

    pub fn step_to_epoch(&mut self, epoch: PyEpoch) -> Result<(), String> {
        let epoch: Epoch = epoch.into();
        self.inner.step_to_epoch(epoch)
    }

    pub fn get_ephemeris(&mut self, start_epoch: PyEpoch, end_epoch: PyEpoch, step: PyTimeSpan) -> Option<Ephemeris> {
        let start_epoch: Epoch = start_epoch.into();
        let end_epoch: Epoch = end_epoch.into();
        let step: TimeSpan = step.into();
        self.inner.get_ephemeris(start_epoch, end_epoch, step)
    }
}

impl Default for PySatellite {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl PySatellite {
    #[new]
    pub fn new() -> Self {
        Satellite::new().into()
    }

    pub fn get_relative_state_at_epoch(&self, origin: &PySatellite, epoch: PyEpoch) -> Option<PyRelativeState> {
        let epoch: Epoch = epoch.into();
        self.inner
            .get_relative_state_at_epoch(origin.inner(), epoch)
            .map(PyRelativeState::from)
    }

    pub fn get_body_angles_at_epoch(&self, other: &PySatellite, epoch: PyEpoch) -> Option<PyBoreToBodyAngles> {
        let epoch: Epoch = epoch.into();
        self.inner
            .get_body_angles_at_epoch(other.inner(), epoch)
            .map(PyBoreToBodyAngles::from)
    }

    #[getter]
    pub fn get_geodetic_position(&self) -> Option<PyGeodeticPosition> {
        self.inner.get_geodetic_position().map(PyGeodeticPosition::from)
    }

    pub fn to_tle(&self) -> PyResult<Option<PyTLE>> {
        match self.inner.get_keplerian_state() {
            Some(_) => Ok(Some(PyTLE::from(TLE::from(self.inner.clone())))),
            None => Ok(None),
        }
    }

    #[staticmethod]
    pub fn from_tle(tle: PyTLE) -> Self {
        let tle: TLE = tle.into();
        Satellite::from(tle).into()
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.inner.id.clone()
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
    pub fn get_periapsis(&self) -> Option<f64> {
        self.inner.get_periapsis()
    }

    #[getter]
    pub fn get_apoapsis(&self) -> Option<f64> {
        self.inner.get_apoapsis()
    }

    #[setter]
    pub fn set_id(&mut self, satellite_id: String) {
        self.inner.id = satellite_id;
    }

    #[getter]
    pub fn get_norad_id(&self) -> i32 {
        self.inner.norad_id
    }

    #[setter]
    pub fn set_norad_id(&mut self, norad_id: i32) {
        self.inner.norad_id = norad_id;
    }

    #[pyo3(name = "get_state_at_epoch")]
    pub fn py_get_state_at_epoch(&self, epoch: PyEpoch) -> Option<PyCartesianState> {
        let epoch: Epoch = epoch.into();
        self.inner.get_state_at_epoch(epoch).map(PyCartesianState::from)
    }

    #[pyo3(name = "step_to_epoch")]
    pub fn py_step_to_epoch(&mut self, epoch: PyEpoch) -> PyResult<()> {
        self.step_to_epoch(epoch)
            .map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
    }

    #[setter]
    pub fn set_keplerian_state(&mut self, keplerian_state: PyKeplerianState) -> PyResult<()> {
        self.inner
            .set_keplerian_state(keplerian_state.into())
            .map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
    }

    #[setter]
    pub fn set_force_properties(&mut self, force_properties: PyForceProperties) {
        let force_properties: ForceProperties = force_properties.into();
        self.inner.set_force_properties(force_properties);
    }

    #[getter]
    pub fn get_force_properties(&self) -> PyForceProperties {
        PyForceProperties::from(self.inner.get_force_properties())
    }

    #[getter]
    pub fn get_keplerian_state(&self) -> Option<PyKeplerianState> {
        self.inner.get_keplerian_state().map(PyKeplerianState::from)
    }

    #[pyo3(name = "get_ephemeris")]
    pub fn py_get_ephemeris(
        &mut self,
        start_epoch: PyEpoch,
        end_epoch: PyEpoch,
        step: PyTimeSpan,
    ) -> Option<PyEphemeris> {
        self.get_ephemeris(start_epoch, end_epoch, step).map(PyEphemeris::from)
    }

    pub fn get_plot_data(&self, start: PyEpoch, end: PyEpoch, step: PyTimeSpan) -> Option<PyOrbitPlotData> {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        let step: TimeSpan = step.into();
        self.inner.get_plot_data(start, end, step).map(PyOrbitPlotData::from)
    }

    pub fn get_close_approach(
        &mut self,
        other: &mut PySatellite,
        start_epoch: PyEpoch,
        end_epoch: PyEpoch,
        distance_threshold: f64,
    ) -> Option<PyCloseApproach> {
        let start_epoch: Epoch = start_epoch.into();
        let end_epoch: Epoch = end_epoch.into();
        self.inner
            .get_close_approach(other.inner_mut(), start_epoch, end_epoch, distance_threshold)
            .map(PyCloseApproach::from)
    }

    pub fn get_proximity_report(
        &mut self,
        py: Python<'_>,
        other: &mut PySatellite,
        start_epoch: PyEpoch,
        end_epoch: PyEpoch,
        distance_threshold: f64,
    ) -> Option<PyProximityReport> {
        let start_epoch: Epoch = start_epoch.into();
        let end_epoch: Epoch = end_epoch.into();
        py.detach(|| {
            self.inner
                .get_proximity_report(other.inner_mut(), start_epoch, end_epoch, distance_threshold)
                .map(PyProximityReport::from)
        })
    }

    pub fn get_maneuver_event(
        &mut self,
        py: Python<'_>,
        future_sat: &mut PySatellite,
        start: PyEpoch,
        end: PyEpoch,
        distance_threshold: f64,
        velocity_threshold: f64,
    ) -> Option<PyManeuverEvent> {
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        py.detach(|| {
            self.inner
                .get_maneuver_event(
                    future_sat.inner_mut(),
                    start,
                    end,
                    distance_threshold,
                    velocity_threshold,
                )
                .map(PyManeuverEvent::from)
        })
    }

    pub fn get_observatory_access_report(
        &mut self,
        py: Python<'_>,
        observatories: Vec<PyObservatory>,
        start: PyEpoch,
        end: PyEpoch,
        min_el: f64,
        min_duration: PyTimeSpan,
    ) -> Option<PyHorizonAccessReport> {
        let min_duration: TimeSpan = min_duration.into();
        let observatories: Vec<Observatory> = observatories.into_iter().map(Observatory::from).collect();
        let start: Epoch = start.into();
        let end: Epoch = end.into();
        py.detach(|| {
            self.inner
                .get_observatory_access_report(observatories, start, end, min_el, min_duration)
                .map(PyHorizonAccessReport::from)
        })
    }

    pub fn get_associations(&self, collections: Vec<PyObservationCollection>) -> Vec<PyObservationAssociation> {
        let collections: Vec<_> = collections.iter().map(|c| c.inner().clone()).collect();
        self.inner
            .get_associations(&collections)
            .into_iter()
            .map(PyObservationAssociation::from)
            .collect()
    }

    pub fn get_rms(&self, obs: Vec<PyObservation>) -> PyResult<f64> {
        let obs: Vec<_> = obs.into_iter().map(|o| o.into()).collect();
        self.inner.get_rms(&obs).map_err(|e| PyErr::new::<PyValueError, _>(e))
    }

    pub fn get_residuals(&self, obs: Vec<PyObservation>) -> PyResult<Vec<PyObservationResidual>> {
        let obs: Vec<_> = obs.into_iter().map(|o| o.into()).collect();
        self.inner
            .get_residuals(&obs)
            .map(|residuals| residuals.into_iter().map(PyObservationResidual::from).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(e))
    }
}
