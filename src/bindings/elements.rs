mod bore_to_body_angles;
mod cartesian_state;
mod cartesian_vector;
mod ephemeris;
mod equinoctial_elements;
mod geodetic_position;
mod horizon_elements;
mod horizon_state;
mod keplerian_elements;
mod keplerian_state;
mod orbit_plot_data;
mod relative_state;
mod spherical_vector;
mod tle;
mod topocentric_elements;
mod topocentric_state;

pub use bore_to_body_angles::PyBoreToBodyAngles;
pub use cartesian_state::PyCartesianState;
pub use cartesian_vector::PyCartesianVector;
pub use ephemeris::PyEphemeris;
pub use equinoctial_elements::PyEquinoctialElements;
pub use geodetic_position::PyGeodeticPosition;
pub use horizon_elements::PyHorizonElements;
pub use horizon_state::PyHorizonState;
pub use keplerian_elements::PyKeplerianElements;
pub use keplerian_state::PyKeplerianState;
pub use orbit_plot_data::{PyOrbitPlotData, PyOrbitPlotState};
pub use relative_state::PyRelativeState;
pub use spherical_vector::PySphericalVector;
pub use tle::PyTLE;
pub use topocentric_elements::PyTopocentricElements;
pub use topocentric_state::PyTopocentricState;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_elements(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let elements = PyModule::new(parent_module.py(), "elements")?;
    elements.add_class::<PyTLE>()?;
    elements.add_class::<PyCartesianState>()?;
    elements.add_class::<PyCartesianVector>()?;
    elements.add_class::<PyKeplerianState>()?;
    elements.add_class::<PyKeplerianElements>()?;
    elements.add_class::<PyEphemeris>()?;
    elements.add_class::<PySphericalVector>()?;
    elements.add_class::<PyTopocentricElements>()?;
    elements.add_class::<PyEquinoctialElements>()?;
    elements.add_class::<PyGeodeticPosition>()?;
    elements.add_class::<PyHorizonElements>()?;
    elements.add_class::<PyHorizonState>()?;
    elements.add_class::<PyOrbitPlotState>()?;
    elements.add_class::<PyOrbitPlotData>()?;
    elements.add_class::<PyRelativeState>()?;
    elements.add_class::<PyBoreToBodyAngles>()?;
    elements.add_class::<PyTopocentricState>()?;
    py_run!(
        parent_module.py(),
        elements,
        "import sys; sys.modules['keplemon._keplemon.elements'] = elements"
    );
    parent_module.add_submodule(&elements)
}
