use saal::tle;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeplerianType {
    MeanKozaiGP = tle::TLETYPE_SGP as isize,
    MeanBrouwerGP = tle::TLETYPE_SGP4 as isize,
    MeanBrouwerXP = tle::TLETYPE_XP as isize,
    Osculating = tle::TLETYPE_SP as isize,
}

impl TryFrom<i32> for KeplerianType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KeplerianType::MeanKozaiGP),
            2 => Ok(KeplerianType::MeanBrouwerGP),
            4 => Ok(KeplerianType::MeanBrouwerXP),
            _ => Err("Invalid KeplerianType value"),
        }
    }
}

impl TryFrom<f64> for KeplerianType {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value as i32 {
            0 => Ok(KeplerianType::MeanKozaiGP),
            2 => Ok(KeplerianType::MeanBrouwerGP),
            4 => Ok(KeplerianType::MeanBrouwerXP),
            _ => Err("Invalid KeplerianType value"),
        }
    }
}
