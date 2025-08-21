mod force_properties;
mod inertial_propagator;
mod sgp4_output;

pub use force_properties::ForceProperties;
pub use inertial_propagator::InertialPropagator;
use pyo3::prelude::*;
use pyo3::py_run;
pub use sgp4_output::SGP4Output;

pub const FINITE_DIFFERENCE_EPSILON: f64 = 1e-10;
pub const FINITE_DIFFERENCE_STEP_SECONDS: f64 = 10.0;

pub fn register_propagation(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let propagation = PyModule::new(parent_module.py(), "propagation")?;
    propagation.add_class::<ForceProperties>()?;
    propagation.add_class::<InertialPropagator>()?;
    propagation.add_class::<SGP4Output>()?;
    propagation.add_function(wrap_pyfunction!(
        force_properties::b_star_to_drag_coefficient,
        &propagation
    )?)?;
    propagation.add_function(wrap_pyfunction!(
        force_properties::drag_coefficient_to_b_star,
        &propagation
    )?)?;
    py_run!(
        parent_module.py(),
        propagation,
        "import sys; sys.modules['keplemon._keplemon.propagation'] = propagation"
    );
    parent_module.add_submodule(&propagation)
}
