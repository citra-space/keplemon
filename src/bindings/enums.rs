mod association_confidence;
mod classification;
mod covariance_type;
mod geodetic_model;
mod keplerian_type;
mod mean_equinox;
mod reference_frame;
mod time_system;

pub use association_confidence::PyAssociationConfidence;
pub use classification::PyClassification;
pub use covariance_type::PyCovarianceType;
pub use geodetic_model::PyGeodeticModel;
pub use keplerian_type::PyKeplerianType;
pub use mean_equinox::PyMeanEquinox;
pub use reference_frame::PyReferenceFrame;
pub use time_system::PyTimeSystem;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_enums(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let enums = PyModule::new(parent_module.py(), "enums")?;
    enums.add_class::<PyTimeSystem>()?;
    enums.add_class::<PyReferenceFrame>()?;
    enums.add_class::<PyClassification>()?;
    enums.add_class::<PyKeplerianType>()?;
    enums.add_class::<PyMeanEquinox>()?;
    enums.add_class::<PyGeodeticModel>()?;
    enums.add_class::<PyCovarianceType>()?;
    enums.add_class::<PyAssociationConfidence>()?;
    py_run!(
        parent_module.py(),
        enums,
        "import sys; sys.modules['keplemon._keplemon.enums'] = enums"
    );
    parent_module.add_submodule(&enums)
}
