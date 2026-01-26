mod batch_propagator;
mod force_properties;
mod inertial_propagator;
mod sgp4_output;

use crate::propagation::{b_star_to_drag_coefficient, drag_coefficient_to_b_star};
pub use batch_propagator::{PyBatchPropagator, PyPropagationBackend};
pub use force_properties::PyForceProperties;
pub use inertial_propagator::PyInertialPropagator;
pub use sgp4_output::PySGP4Output;

use pyo3::prelude::*;
use pyo3::py_run;

#[pyfunction(name = "b_star_to_drag_coefficient")]
pub fn b_star_to_drag_coefficient_py(b_star: f64) -> PyResult<f64> {
    Ok(b_star_to_drag_coefficient(b_star))
}

#[pyfunction(name = "drag_coefficient_to_b_star")]
pub fn drag_coefficient_to_b_star_py(drag_coefficient: f64) -> PyResult<f64> {
    Ok(drag_coefficient_to_b_star(drag_coefficient))
}

pub fn register_propagation(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let propagation = PyModule::new(parent_module.py(), "propagation")?;
    propagation.add_class::<PyForceProperties>()?;
    propagation.add_class::<PyInertialPropagator>()?;
    propagation.add_class::<PySGP4Output>()?;
    propagation.add_class::<PyBatchPropagator>()?;
    propagation.add_class::<PyPropagationBackend>()?;
    propagation.add_function(wrap_pyfunction!(b_star_to_drag_coefficient_py, &propagation)?)?;
    propagation.add_function(wrap_pyfunction!(drag_coefficient_to_b_star_py, &propagation)?)?;
    py_run!(
        parent_module.py(),
        propagation,
        "import sys; sys.modules['keplemon._keplemon.propagation'] = propagation"
    );
    parent_module.add_submodule(&propagation)
}
