use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeSystem {
    UTC,
    TAI,
    UT1,
    TT,
}

impl Display for TimeSystem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TimeSystem::UTC => write!(f, "UTC"),
            TimeSystem::TAI => write!(f, "TAI"),
            TimeSystem::UT1 => write!(f, "UT1"),
            TimeSystem::TT => write!(f, "TT"),
        }
    }
}
