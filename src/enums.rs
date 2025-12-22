mod association_confidence;
mod classification;
mod covariance_type;
mod equinox_type;
mod geodetic_model;
mod keplerian_type;
mod reference_frame;
mod saal_key_mode;
mod time_system;

pub use association_confidence::AssociationConfidence;
pub use classification::Classification;
pub use covariance_type::CovarianceType;
pub use equinox_type::EquinoxType;
pub use geodetic_model::GeodeticModel;
pub use keplerian_type::KeplerianType;
pub use reference_frame::ReferenceFrame;
pub use saal_key_mode::SAALKeyMode;
pub use time_system::TimeSystem;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_enums(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let enums = PyModule::new(parent_module.py(), "enums")?;
    enums.add_class::<SAALKeyMode>()?;
    enums.add_class::<TimeSystem>()?;
    enums.add_class::<ReferenceFrame>()?;
    enums.add_class::<Classification>()?;
    enums.add_class::<KeplerianType>()?;
    enums.add_class::<EquinoxType>()?;
    enums.add_class::<GeodeticModel>()?;
    enums.add_class::<CovarianceType>()?;
    enums.add_class::<AssociationConfidence>()?;
    py_run!(
        parent_module.py(),
        enums,
        "import sys; sys.modules['keplemon._keplemon.enums'] = enums"
    );
    parent_module.add_submodule(&enums)
}
