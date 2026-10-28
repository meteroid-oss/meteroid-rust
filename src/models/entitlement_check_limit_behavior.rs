// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EntitlementCheckLimitBehavior {
    Track,
    Soft,
    Hard,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl EntitlementCheckLimitBehavior {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Track => "TRACK",
            Self::Soft => "SOFT",
            Self::Hard => "HARD",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for EntitlementCheckLimitBehavior {
    fn from(value: &str) -> Self {
        match value {
            "TRACK" => Self::Track,
            "SOFT" => Self::Soft,
            "HARD" => Self::Hard,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for EntitlementCheckLimitBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for EntitlementCheckLimitBehavior {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for EntitlementCheckLimitBehavior {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for EntitlementCheckLimitBehavior {
    fn encode(&self) -> String {
        self.to_string()
    }
}
