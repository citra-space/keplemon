use super::PyFieldOfViewCandidate;
use crate::bindings::elements::{PyCartesianVector, PyTopocentricElements};
use crate::bindings::enums::PyReferenceFrame;
use crate::bindings::time::PyEpoch;
use crate::enums::ReferenceFrame;
use crate::events::{FieldOfViewCandidate, FieldOfViewReport};
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass(name = "FieldOfViewReport")]
pub struct PyFieldOfViewReport {
    inner: FieldOfViewReport,
}

impl From<FieldOfViewReport> for PyFieldOfViewReport {
    fn from(inner: FieldOfViewReport) -> Self {
        Self { inner }
    }
}

impl From<PyFieldOfViewReport> for FieldOfViewReport {
    fn from(value: PyFieldOfViewReport) -> Self {
        value.inner
    }
}

impl PyFieldOfViewReport {
    pub fn set_candidates(&mut self, candidates: Vec<PyFieldOfViewCandidate>) {
        let candidates: Vec<FieldOfViewCandidate> = candidates.into_iter().map(FieldOfViewCandidate::from).collect();
        self.inner.set_candidates(candidates);
    }
}

#[pymethods]
impl PyFieldOfViewReport {
    #[new]
    pub fn new(
        epoch: PyEpoch,
        sensor_position: PyCartesianVector,
        sensor_direction: &PyTopocentricElements,
        fov_angle: f64,
        reference_frame: PyReferenceFrame,
    ) -> Self {
        let epoch: Epoch = epoch.into();
        let reference_frame: ReferenceFrame = reference_frame.into();
        FieldOfViewReport::new(
            epoch,
            sensor_position.into(),
            &(*sensor_direction).into(),
            fov_angle,
            reference_frame,
        )
        .into()
    }

    #[getter]
    pub fn get_epoch(&self) -> PyEpoch {
        self.inner.get_epoch().into()
    }

    #[getter]
    pub fn get_reference_frame(&self) -> PyReferenceFrame {
        PyReferenceFrame::from(self.inner.get_reference_frame())
    }

    #[getter]
    pub fn get_sensor_position(&self) -> PyCartesianVector {
        PyCartesianVector::from(self.inner.get_sensor_position())
    }

    #[getter]
    pub fn get_sensor_direction(&self) -> PyTopocentricElements {
        PyTopocentricElements::from(self.inner.get_sensor_direction())
    }

    #[getter]
    pub fn get_fov_angle(&self) -> f64 {
        self.inner.get_fov_angle()
    }

    #[getter]
    pub fn get_candidates(&self) -> Vec<PyFieldOfViewCandidate> {
        self.inner
            .get_candidates()
            .into_iter()
            .map(PyFieldOfViewCandidate::from)
            .collect()
    }
}
