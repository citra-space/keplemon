mod force_properties;
mod inertial_propagator;
mod sgp4_output;

pub use force_properties::{ForceProperties, b_star_to_drag_coefficient, drag_coefficient_to_b_star};
pub use inertial_propagator::InertialPropagator;
pub use sgp4_output::SGP4Output;

pub const FINITE_DIFFERENCE_EPSILON: f64 = 1e-10;
pub const FINITE_DIFFERENCE_STEP_SECONDS: f64 = 10.0;
