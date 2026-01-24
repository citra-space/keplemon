mod batch_least_squares;
mod collection_association_report;
mod covariance;
mod observation;
mod observation_association;
mod observation_collection;
mod observation_residual;

pub use batch_least_squares::PyBatchLeastSquares;
pub use collection_association_report::PyCollectionAssociationReport;
pub use covariance::PyCovariance;
pub use observation::PyObservation;
pub use observation_association::PyObservationAssociation;
pub use observation_collection::PyObservationCollection;
pub use observation_residual::PyObservationResidual;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_estimation(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let estimation = PyModule::new(parent_module.py(), "estimation")?;
    estimation.add_class::<PyObservation>()?;
    estimation.add_class::<PyObservationResidual>()?;
    estimation.add_class::<PyBatchLeastSquares>()?;
    estimation.add_class::<PyCovariance>()?;
    estimation.add_class::<PyObservationAssociation>()?;
    estimation.add_class::<PyObservationCollection>()?;
    estimation.add_class::<PyCollectionAssociationReport>()?;
    py_run!(
        parent_module.py(),
        estimation,
        "import sys; sys.modules['keplemon._keplemon.estimation'] = estimation"
    );
    parent_module.add_submodule(&estimation)
}
