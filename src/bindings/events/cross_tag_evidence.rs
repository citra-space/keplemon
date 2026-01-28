use crate::bindings::estimation::{PyObservation, PyObservationAssociation};
use crate::bindings::time::PyEpoch;
use crate::events::CrossTagEvidence;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "CrossTagEvidence")]
#[derive(Debug, Clone)]
pub struct PyCrossTagEvidence {
    inner: CrossTagEvidence,
}

impl From<CrossTagEvidence> for PyCrossTagEvidence {
    fn from(inner: CrossTagEvidence) -> Self {
        Self { inner }
    }
}

impl From<PyCrossTagEvidence> for CrossTagEvidence {
    fn from(value: PyCrossTagEvidence) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyCrossTagEvidence {
    #[new]
    pub fn new(
        epoch: PyEpoch,
        sensor_id: String,
        orphan_count: usize,
        approved_satellite_matched: bool,
        uct_was_visible: bool,
        conclusion: String,
        approved_associations: Vec<PyObservationAssociation>,
        orphan_observations: Vec<PyObservation>,
    ) -> Self {
        let epoch: Epoch = epoch.into();
        let approved_associations = approved_associations
            .into_iter()
            .map(crate::estimation::ObservationAssociation::from)
            .collect();
        let orphan_observations = orphan_observations
            .into_iter()
            .map(crate::estimation::Observation::from)
            .collect();
        CrossTagEvidence::new(
            epoch,
            sensor_id,
            orphan_count,
            approved_satellite_matched,
            uct_was_visible,
            conclusion,
            approved_associations,
            orphan_observations,
        )
        .into()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    #[getter]
    pub fn get_sensor_id(&self) -> String {
        self.inner.get_sensor_id()
    }

    #[getter]
    pub fn get_orphan_count(&self) -> usize {
        self.inner.get_orphan_count()
    }

    #[getter]
    pub fn get_approved_satellite_matched(&self) -> bool {
        self.inner.get_approved_satellite_matched()
    }

    #[getter]
    pub fn get_uct_was_visible(&self) -> bool {
        self.inner.get_uct_was_visible()
    }

    #[getter]
    pub fn get_conclusion(&self) -> String {
        self.inner.get_conclusion()
    }

    #[getter]
    pub fn get_approved_associations(&self) -> Vec<PyObservationAssociation> {
        self.inner
            .get_approved_associations()
            .into_iter()
            .map(PyObservationAssociation::from)
            .collect()
    }

    #[getter]
    pub fn get_orphan_observations(&self) -> Vec<PyObservation> {
        self.inner
            .get_orphan_observations()
            .into_iter()
            .map(PyObservation::from)
            .collect()
    }
}
