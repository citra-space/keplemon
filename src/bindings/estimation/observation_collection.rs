use super::{PyCollectionAssociationReport, PyObservation, PyObservationAssociation};
use crate::bindings::bodies::{PyConstellation, PySatellite};
use crate::bindings::elements::PyCartesianVector;
use crate::bindings::time::PyEpoch;
use crate::estimation::ObservationCollection;
use pyo3::prelude::*;

#[pyclass(name = "ObservationCollection")]
#[derive(Debug, Clone)]
pub struct PyObservationCollection {
    inner: ObservationCollection,
}

impl From<ObservationCollection> for PyObservationCollection {
    fn from(inner: ObservationCollection) -> Self {
        Self { inner }
    }
}

impl PyObservationCollection {
    pub fn inner(&self) -> &ObservationCollection {
        &self.inner
    }
}

#[pymethods]
impl PyObservationCollection {
    #[new]
    pub fn new(obs: Vec<PyObservation>) -> PyResult<Self> {
        let observations = obs.into_iter().map(|o| o.into()).collect();
        ObservationCollection::new(observations)
            .map(PyObservationCollection::from)
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    #[staticmethod]
    pub fn get_list(obs: Vec<PyObservation>) -> Vec<PyObservationCollection> {
        let observations = obs.into_iter().map(|o| o.into()).collect();
        ObservationCollection::get_list(observations)
            .into_iter()
            .map(PyObservationCollection::from)
            .collect()
    }

    #[getter]
    pub fn get_sensor_position(&self) -> PyCartesianVector {
        self.inner.get_sensor_position().into()
    }

    #[getter]
    pub fn get_sensor_direction(&self) -> PyCartesianVector {
        self.inner.get_sensor_direction().into()
    }

    #[getter]
    pub fn get_field_of_view(&self) -> f64 {
        self.inner.get_field_of_view()
    }

    #[getter]
    pub fn get_observations(&self) -> Vec<PyObservation> {
        self.inner
            .get_observations()
            .iter()
            .cloned()
            .map(PyObservation::from)
            .collect()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    pub fn get_visibility(&self, satellite: &PySatellite) -> bool {
        self.inner.get_visibility(satellite.inner())
    }

    pub fn get_association(&self, satellite: &PySatellite) -> Option<PyObservationAssociation> {
        self.inner
            .get_association(satellite.inner())
            .map(PyObservationAssociation::from)
    }

    pub fn get_association_report(&self, satellites: &PyConstellation) -> PyCollectionAssociationReport {
        PyCollectionAssociationReport::from(self.inner.get_association_report(satellites.inner()))
    }
}
