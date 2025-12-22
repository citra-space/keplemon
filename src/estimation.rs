mod batch_least_squares;
mod covariance;
mod observation;
mod observation_association;
mod observation_residual;

pub use batch_least_squares::BatchLeastSquares;
pub use covariance::Covariance;
pub use observation::Observation;
pub use observation_association::ObservationAssociation;
pub use observation_residual::ObservationResidual;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_estimation(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let estimation = PyModule::new(parent_module.py(), "estimation")?;
    estimation.add_class::<Observation>()?;
    estimation.add_class::<ObservationResidual>()?;
    estimation.add_class::<BatchLeastSquares>()?;
    estimation.add_class::<Covariance>()?;
    estimation.add_class::<ObservationAssociation>()?;
    py_run!(
        parent_module.py(),
        estimation,
        "import sys; sys.modules['keplemon._keplemon.estimation'] = estimation"
    );
    parent_module.add_submodule(&estimation)
}
