use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Classification {
    Unclassified = 1,
    Confidential = 2,
    Secret = 3,
}

impl FromStr for Classification {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "U" => Ok(Classification::Unclassified),
            "C" => Ok(Classification::Confidential),
            "S" => Ok(Classification::Secret),
            _ => Err(format!("Invalid TLE classification: {}", s)),
        }
    }
}

impl Classification {
    pub fn as_char(&self) -> &str {
        match self {
            Classification::Unclassified => "U",
            Classification::Confidential => "C",
            Classification::Secret => "S",
        }
    }
}
