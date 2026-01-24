use super::PyObservationResidual;
use crate::bindings::enums::PyAssociationConfidence;
use crate::enums::AssociationConfidence;
use crate::estimation::ObservationAssociation;
use pyo3::prelude::*;

#[pyclass(name = "ObservationAssociation")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyObservationAssociation {
    inner: ObservationAssociation,
}

impl From<ObservationAssociation> for PyObservationAssociation {
    fn from(inner: ObservationAssociation) -> Self {
        Self { inner }
    }
}

impl From<PyObservationAssociation> for ObservationAssociation {
    fn from(value: PyObservationAssociation) -> Self {
        value.inner
    }
}

impl PyObservationAssociation {
    pub fn new(
        observation_id: String,
        satellite_id: String,
        residual: PyObservationResidual,
        confidence: AssociationConfidence,
    ) -> Self {
        Self {
            inner: ObservationAssociation::new(observation_id, satellite_id, residual.into(), confidence),
        }
    }
}

#[pymethods]
impl PyObservationAssociation {
    #[getter]
    pub fn get_observation_id(&self) -> &str {
        self.inner.get_observation_id()
    }

    #[getter]
    pub fn get_satellite_id(&self) -> &str {
        self.inner.get_satellite_id()
    }

    #[getter]
    pub fn get_confidence(&self) -> PyAssociationConfidence {
        PyAssociationConfidence::from(self.inner.get_confidence())
    }

    #[getter]
    pub fn get_residual(&self) -> PyObservationResidual {
        PyObservationResidual::from(self.inner.get_residual())
    }
}
