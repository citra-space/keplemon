use crate::time::SECONDS_TO_DAYS;

pub const CONJUNCTION_STEP_MINUTES: f64 = 10.0;
pub const MAX_NEWTON_ITERATIONS: usize = 10;
pub const NEWTON_TOLERANCE: f64 = 1e-6;
pub const DEFAULT_SRP_TERM: f64 = 0.03;
pub const DEFAULT_DRAG_TERM: f64 = 0.01;
pub const MAX_BISECTION_ITERATIONS: usize = 10;
pub const DEFAULT_NORAD_ANALYST_ID: i32 = 99999;
pub const HORIZON_ACCESS_TOLERANCE: f64 = 1.0 * SECONDS_TO_DAYS; // in days
pub const ZERO_TOLERANCE: f64 = 1e-10;
