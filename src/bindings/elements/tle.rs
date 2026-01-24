use super::{PyCartesianState, PyKeplerianState};
use crate::bindings::enums::{PyClassification, PyKeplerianType};
use crate::bindings::estimation::PyObservation;
use crate::bindings::propagation::PyForceProperties;
use crate::bindings::time::PyEpoch;
use crate::elements::TLE;
use crate::enums::Classification;
use crate::propagation::ForceProperties;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "TLE")]
#[derive(Debug, PartialEq, Clone)]
pub struct PyTLE {
    inner: TLE,
}

impl From<TLE> for PyTLE {
    fn from(inner: TLE) -> Self {
        Self { inner }
    }
}

impl From<PyTLE> for TLE {
    fn from(value: PyTLE) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyTLE {
    #[staticmethod]
    #[pyo3(signature = (line_1, line_2, line_3 = None))]
    pub fn from_lines(line_1: &str, line_2: &str, line_3: Option<&str>) -> PyResult<PyTLE> {
        match TLE::from_lines(line_1, line_2, line_3) {
            Ok(tle) => Ok(PyTLE::from(tle)),
            Err(e) => Err(PyValueError::new_err(e)),
        }
    }

    /// Create a TLE from Keplerian elements
    ///
    /// This allows creating TLE objects from orbital elements (COE) instead of
    /// TLE line strings. The resulting TLE can be used with batch propagation
    /// including CUDA-accelerated SGP4.
    ///
    /// # Arguments
    /// * `keplerian_state` - Orbital state with epoch and Keplerian elements
    /// * `norad_id` - NORAD catalog ID (use 99999 for analyst objects)
    /// * `name` - Optional satellite name
    /// * `classification` - Security classification (default: Unclassified)
    /// * `designator` - International designator (default: empty)
    /// * `force_properties` - Atmospheric drag/SRP properties (optional)
    ///
    /// # Returns
    /// TLE object that can be propagated using SGP4 (CPU or CUDA)
    #[staticmethod]
    #[pyo3(signature = (keplerian_state, norad_id = 99999, name = None, classification = None, designator = None, force_properties = None))]
    pub fn from_elements(
        keplerian_state: PyKeplerianState,
        norad_id: i32,
        name: Option<String>,
        classification: Option<PyClassification>,
        designator: Option<String>,
        force_properties: Option<PyForceProperties>,
    ) -> PyResult<PyTLE> {
        let classification: Classification = classification
            .map(Classification::from)
            .unwrap_or(Classification::Unclassified);
        let designator = designator.unwrap_or_default();
        let force_properties: ForceProperties = force_properties.map(ForceProperties::from).unwrap_or_default();

        // Generate a unique satellite ID if not provided via name
        let satellite_id = name.clone().unwrap_or_else(|| format!("SAT-{}", norad_id));

        match TLE::new(
            satellite_id,
            norad_id,
            name,
            classification,
            designator,
            keplerian_state.into(),
            force_properties,
        ) {
            Ok(tle) => Ok(PyTLE::from(tle)),
            Err(e) => Err(PyValueError::new_err(e)),
        }
    }

    #[getter]
    pub fn get_lines(&self) -> (String, String) {
        self.inner.get_lines().unwrap()
    }

    #[getter]
    pub fn get_inclination(&self) -> f64 {
        self.inner.get_inclination()
    }

    #[getter]
    pub fn get_raan(&self) -> f64 {
        self.inner.get_raan()
    }

    #[getter]
    pub fn get_semi_major_axis(&self) -> f64 {
        self.inner.get_semi_major_axis()
    }

    #[getter]
    pub fn get_apoapsis(&self) -> f64 {
        self.inner.get_apoapsis()
    }

    #[getter]
    pub fn get_periapsis(&self) -> f64 {
        self.inner.get_periapsis()
    }

    #[getter]
    pub fn get_eccentricity(&self) -> f64 {
        self.inner.get_eccentricity()
    }

    #[getter]
    pub fn get_argument_of_perigee(&self) -> f64 {
        self.inner.get_argument_of_perigee()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.inner.get_name()
    }

    #[getter]
    pub fn get_mean_anomaly(&self) -> f64 {
        self.inner.get_mean_anomaly()
    }

    #[getter]
    pub fn get_mean_motion(&self) -> f64 {
        self.inner.get_mean_motion()
    }

    #[getter]
    pub fn get_type(&self) -> PyKeplerianType {
        PyKeplerianType::from(self.inner.get_type())
    }

    #[getter]
    pub fn get_b_star(&self) -> f64 {
        self.inner.get_b_star()
    }

    #[getter]
    pub fn get_mean_motion_dot(&self) -> f64 {
        self.inner.get_mean_motion_dot()
    }

    #[getter]
    pub fn get_mean_motion_dot_dot(&self) -> f64 {
        self.inner.get_mean_motion_dot_dot()
    }

    #[getter]
    pub fn get_agom(&self) -> f64 {
        self.inner.get_agom()
    }

    #[getter]
    pub fn get_b_term(&self) -> f64 {
        self.inner.get_b_term()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    #[getter]
    pub fn get_classification(&self) -> PyClassification {
        PyClassification::from(self.inner.classification)
    }

    #[getter]
    pub fn get_designator(&self) -> String {
        self.inner.designator.clone()
    }

    #[getter]
    pub fn get_norad_id(&self) -> i32 {
        self.inner.norad_id
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.inner.satellite_id.clone()
    }

    #[getter]
    fn get_cartesian_state(&self) -> PyCartesianState {
        PyCartesianState::from(self.inner.get_cartesian_state())
    }

    /// Propagate multiple TLEs to multiple epochs using batch propagation
    ///
    /// GPU acceleration is used automatically when beneficial based on problem size.
    ///
    /// # Arguments
    /// * `tles` - List of TLEs to propagate
    /// * `epochs` - List of epochs to propagate to
    ///
    /// # Returns
    /// 2D list of states: result[sat_idx][epoch_idx]
    #[staticmethod]
    #[pyo3(signature = (tles, epochs))]
    pub fn propagate_batch(
        py: Python<'_>,
        tles: Vec<PyTLE>,
        epochs: Vec<PyEpoch>,
    ) -> PyResult<Vec<Vec<PyCartesianState>>> {
        let tles: Vec<TLE> = tles.into_iter().map(|tle| tle.into()).collect();
        let epochs: Vec<crate::time::Epoch> = epochs.into_iter().map(|e| e.into()).collect();

        py.detach(|| {
            TLE::propagate_batch(&tles, &epochs)
                .map(|results| {
                    results
                        .into_iter()
                        .map(|sat_states| sat_states.into_iter().map(PyCartesianState::from).collect())
                        .collect()
                })
                .map_err(|e| PyValueError::new_err(e))
        })
    }

    /// Propagate a single TLE to multiple epochs
    ///
    /// Automatically uses GPU if the number of epochs is large enough.
    ///
    /// # Arguments
    /// * `epochs` - List of epochs to propagate to
    ///
    /// # Returns
    /// List of states, one for each epoch
    #[pyo3(signature = (epochs))]
    pub fn propagate_to_epochs(&self, py: Python<'_>, epochs: Vec<PyEpoch>) -> PyResult<Vec<PyCartesianState>> {
        let epochs: Vec<crate::time::Epoch> = epochs.into_iter().map(|e| e.into()).collect();

        py.detach(|| {
            self.inner
                .propagate_to_epochs(&epochs)
                .map(|states| states.into_iter().map(PyCartesianState::from).collect())
                .map_err(|e| PyValueError::new_err(e))
        })
    }

    /// Get an observation at a specific epoch
    ///
    /// Creates an observation from the TLE's propagated state at the given epoch,
    /// useful for batch least squares fitting of multiple TLEs.
    ///
    /// # Arguments
    /// * `epoch` - The epoch at which to generate the observation
    pub fn get_observation_at_epoch(&self, epoch: PyEpoch) -> PyResult<PyObservation> {
        self.inner
            .get_observation_at_epoch(epoch.into())
            .map(PyObservation::from)
            .map_err(PyValueError::new_err)
    }

    /// Get a Cartesian state at a specific epoch by propagating the TLE
    ///
    /// # Arguments
    /// * `epoch` - The epoch at which to get the state
    ///
    /// # Returns
    /// CartesianState in TEME frame at the specified epoch
    pub fn get_state_at_epoch(&self, epoch: PyEpoch) -> PyResult<PyCartesianState> {
        self.inner
            .get_cartesian_state_at_epoch(epoch.into())
            .map(PyCartesianState::from)
            .map_err(PyValueError::new_err)
    }

    /// Get a Keplerian state at a specific epoch by propagating the TLE
    ///
    /// Returns mean elements appropriate to the TLE type (Kozai for SGP4, Brouwer for XP).
    ///
    /// # Arguments
    /// * `epoch` - The epoch at which to get the state
    ///
    /// # Returns
    /// KeplerianState in TEME frame at the specified epoch
    pub fn get_keplerian_state_at_epoch(&self, epoch: PyEpoch) -> PyResult<PyKeplerianState> {
        self.inner
            .get_keplerian_state_at_epoch(epoch.into())
            .map(PyKeplerianState::from)
            .map_err(PyValueError::new_err)
    }
}
