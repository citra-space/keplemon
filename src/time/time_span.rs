use super::{DAYS_TO_HOURS, DAYS_TO_MINUTES, DAYS_TO_SECONDS, HOURS_TO_DAYS, MINUTES_TO_DAYS, SECONDS_TO_DAYS};
use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpan {
    days: f64,
}

impl PartialOrd for TimeSpan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.days.partial_cmp(&other.days)
    }
}

impl TimeSpan {
    pub fn new(days: f64) -> Self {
        Self { days }
    }
}

impl Div<f64> for TimeSpan {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.days / rhs
    }
}

impl Mul<f64> for TimeSpan {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { days: self.days * rhs }
    }
}

impl TimeSpan {
    pub fn from_days(days: f64) -> Self {
        Self { days }
    }

    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            days: seconds * SECONDS_TO_DAYS,
        }
    }

    pub fn from_minutes(minutes: f64) -> Self {
        Self {
            days: minutes * MINUTES_TO_DAYS,
        }
    }

    pub fn from_hours(hours: f64) -> Self {
        Self {
            days: hours * HOURS_TO_DAYS,
        }
    }

    pub fn in_days(&self) -> f64 {
        self.days
    }

    pub fn in_seconds(&self) -> f64 {
        self.days * DAYS_TO_SECONDS
    }

    pub fn in_minutes(&self) -> f64 {
        self.days * DAYS_TO_MINUTES
    }

    pub fn in_hours(&self) -> f64 {
        self.days * DAYS_TO_HOURS
    }
}
