mod close_approach;
mod close_approach_report;
mod field_of_view_candidate;
mod field_of_view_report;
mod horizon_access;
mod horizon_access_report;

pub use close_approach::CloseApproach;
pub use close_approach_report::CloseApproachReport;
pub use field_of_view_candidate::FieldOfViewCandidate;
pub use field_of_view_report::FieldOfViewReport;
pub use horizon_access::HorizonAccess;
pub use horizon_access_report::HorizonAccessReport;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_events(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let events = PyModule::new(parent_module.py(), "events")?;
    events.add_class::<CloseApproach>()?;
    events.add_class::<CloseApproachReport>()?;
    events.add_class::<HorizonAccess>()?;
    events.add_class::<HorizonAccessReport>()?;
    events.add_class::<FieldOfViewReport>()?;
    py_run!(
        parent_module.py(),
        events,
        "import sys; sys.modules['keplemon._keplemon.events'] = events"
    );
    parent_module.add_submodule(&events)
}
