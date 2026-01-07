mod force_properties;
mod inertial_propagator;
mod sgp4_output;

pub use force_properties::PyForceProperties;
pub use inertial_propagator::PyInertialPropagator;
pub use sgp4_output::PySGP4Output;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_propagation(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let propagation = PyModule::new(parent_module.py(), "propagation")?;
    propagation.add_class::<PyForceProperties>()?;
    propagation.add_class::<PyInertialPropagator>()?;
    propagation.add_class::<PySGP4Output>()?;
    py_run!(
        parent_module.py(),
        propagation,
        "import sys; sys.modules['keplemon._keplemon.propagation'] = propagation"
    );
    parent_module.add_submodule(&propagation)
}
