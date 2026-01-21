mod constellation;
mod observatory;
mod satellite;
mod sensor;

pub use constellation::PyConstellation;
pub use observatory::PyObservatory;
pub use satellite::PySatellite;
pub use sensor::PySensor;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_bodies(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let bodies = PyModule::new(parent_module.py(), "bodies")?;
    bodies.add_class::<PySatellite>()?;
    bodies.add_class::<PyConstellation>()?;
    bodies.add_class::<PySensor>()?;
    bodies.add_class::<PyObservatory>()?;
    py_run!(
        parent_module.py(),
        bodies,
        "import sys; sys.modules['keplemon._keplemon.bodies'] = bodies"
    );
    parent_module.add_submodule(&bodies)
}
