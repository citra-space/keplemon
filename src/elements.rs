mod cartesian_state;
mod cartesian_vector;
mod ephemeris;
mod equinoctial_elements;
mod geodetic_position;
mod keplerian_elements;
mod keplerian_state;
mod spherical_vector;
mod tle;
mod topocentric_elements;

pub use cartesian_state::CartesianState;
pub use cartesian_vector::CartesianVector;
pub use ephemeris::Ephemeris;
pub use equinoctial_elements::EquinoctialElements;
pub use geodetic_position::GeodeticPosition;
pub use keplerian_elements::KeplerianElements;
pub use keplerian_state::KeplerianState;
use pyo3::prelude::*;
use pyo3::py_run;
pub use spherical_vector::SphericalVector;
pub use tle::TLE;
pub use topocentric_elements::TopocentricElements;

pub const B_STAR_TO_B_TERM: f64 = 12.741621;

pub fn register_elements(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let elements = PyModule::new(parent_module.py(), "elements")?;
    elements.add_class::<TLE>()?;
    elements.add_class::<CartesianState>()?;
    elements.add_class::<CartesianVector>()?;
    elements.add_class::<KeplerianState>()?;
    elements.add_class::<KeplerianElements>()?;
    elements.add_class::<Ephemeris>()?;
    elements.add_class::<SphericalVector>()?;
    elements.add_class::<TopocentricElements>()?;
    elements.add_class::<EquinoctialElements>()?;
    elements.add_class::<GeodeticPosition>()?;
    py_run!(
        parent_module.py(),
        elements,
        "import sys; sys.modules['keplemon._keplemon.elements'] = elements"
    );
    parent_module.add_submodule(&elements)
}
