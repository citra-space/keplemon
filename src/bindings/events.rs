mod close_approach;
mod close_approach_report;
mod field_of_view_candidate;
mod field_of_view_report;
mod horizon_access;
mod horizon_access_report;

pub use close_approach::PyCloseApproach;
pub use close_approach_report::PyCloseApproachReport;
pub use field_of_view_candidate::PyFieldOfViewCandidate;
pub use field_of_view_report::PyFieldOfViewReport;
pub use horizon_access::PyHorizonAccess;
pub use horizon_access_report::PyHorizonAccessReport;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_events(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let events = PyModule::new(parent_module.py(), "events")?;
    events.add_class::<PyCloseApproach>()?;
    events.add_class::<PyCloseApproachReport>()?;
    events.add_class::<PyHorizonAccess>()?;
    events.add_class::<PyHorizonAccessReport>()?;
    events.add_class::<PyFieldOfViewReport>()?;
    py_run!(
        parent_module.py(),
        events,
        "import sys; sys.modules['keplemon._keplemon.events'] = events"
    );
    parent_module.add_submodule(&events)
}
