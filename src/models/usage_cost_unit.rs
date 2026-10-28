// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// What a cost is counted in.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum UsageCostUnit {
    Currency,
    CreditUnit,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl UsageCostUnit {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Currency => "CURRENCY",
            Self::CreditUnit => "CREDIT_UNIT",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for UsageCostUnit {
    fn from(value: &str) -> Self {
        match value {
            "CURRENCY" => Self::Currency,
            "CREDIT_UNIT" => Self::CreditUnit,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for UsageCostUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for UsageCostUnit {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for UsageCostUnit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for UsageCostUnit {
    fn encode(&self) -> String {
        self.to_string()
    }
}
