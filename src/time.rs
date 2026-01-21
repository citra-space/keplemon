mod epoch;
mod time_components;
mod time_span;

pub use epoch::Epoch;
pub use time_components::TimeComponents;
pub use time_span::TimeSpan;

pub const DAYS_TO_SECONDS: f64 = 86400.0;
pub const DAYS_TO_MINUTES: f64 = 1440.0;
pub const DAYS_TO_HOURS: f64 = 24.0;
pub const SECONDS_TO_DAYS: f64 = 1.0 / DAYS_TO_SECONDS;
pub const MINUTES_TO_DAYS: f64 = 1.0 / DAYS_TO_MINUTES;
pub const HOURS_TO_DAYS: f64 = 1.0 / DAYS_TO_HOURS;
