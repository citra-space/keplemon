mod candidate_analysis;
mod close_approach;
mod close_approach_report;
mod cross_tag_evidence;
mod cross_tag_report;
mod field_of_view_candidate;
mod field_of_view_report;
mod horizon_access;
mod horizon_access_report;
mod maneuver_event;
mod maneuver_report;
mod proximity_event;
mod proximity_report;

pub use candidate_analysis::PyCandidateAnalysis;
pub use close_approach::PyCloseApproach;
pub use close_approach_report::PyCloseApproachReport;
pub use cross_tag_evidence::PyCrossTagEvidence;
pub use cross_tag_report::{PyCrossTagReport, PyCrossTagResult};
pub use field_of_view_candidate::PyFieldOfViewCandidate;
pub use field_of_view_report::PyFieldOfViewReport;
pub use horizon_access::PyHorizonAccess;
pub use horizon_access_report::PyHorizonAccessReport;
pub use maneuver_event::PyManeuverEvent;
pub use maneuver_report::PyManeuverReport;
pub use proximity_event::PyProximityEvent;
pub use proximity_report::PyProximityReport;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_events(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let events = PyModule::new(parent_module.py(), "events")?;
    events.add_class::<PyCandidateAnalysis>()?;
    events.add_class::<PyCloseApproach>()?;
    events.add_class::<PyCloseApproachReport>()?;
    events.add_class::<PyCrossTagEvidence>()?;
    events.add_class::<PyCrossTagReport>()?;
    events.add_class::<PyCrossTagResult>()?;
    events.add_class::<PyHorizonAccess>()?;
    events.add_class::<PyHorizonAccessReport>()?;
    events.add_class::<PyFieldOfViewCandidate>()?;
    events.add_class::<PyFieldOfViewReport>()?;
    events.add_class::<PyManeuverEvent>()?;
    events.add_class::<PyManeuverReport>()?;
    events.add_class::<PyProximityEvent>()?;
    events.add_class::<PyProximityReport>()?;
    py_run!(
        parent_module.py(),
        events,
        "import sys; sys.modules['keplemon._keplemon.events'] = events"
    );
    parent_module.add_submodule(&events)
}
