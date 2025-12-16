use crate::saal::TLEInterface;
use pyo3::prelude::*;
use std::convert::TryFrom;

#[pyclass]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeplerianType {
    MeanKozaiGP = TLEInterface::TLETYPE_SGP,
    MeanBrouwerGP = TLEInterface::TLETYPE_SGP4,
    MeanBrouwerXP = TLEInterface::TLETYPE_XP,
    Osculating = TLEInterface::TLETYPE_SP,
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

#[pymethods]
impl KeplerianType {
    pub fn __repr__(&self) -> &'static str {
        match self {
            KeplerianType::MeanKozaiGP => "KeplerianType.MeanKozaiGP",
            KeplerianType::MeanBrouwerGP => "KeplerianType.MeanBrouwerGP",
            KeplerianType::MeanBrouwerXP => "KeplerianType.MeanBrouwerXP",
            KeplerianType::Osculating => "KeplerianType.Osculating",
        }
    }

    #[getter]
    pub fn value(&self) -> i32 {
        match self {
            KeplerianType::MeanKozaiGP => TLEInterface::TLETYPE_SGP as i32,
            KeplerianType::MeanBrouwerGP => TLEInterface::TLETYPE_SGP4 as i32,
            KeplerianType::MeanBrouwerXP => TLEInterface::TLETYPE_XP as i32,
            KeplerianType::Osculating => TLEInterface::TLETYPE_XP as i32,
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}
