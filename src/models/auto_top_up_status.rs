// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AutoTopUpStatus {
    Active,
    Disabled,
    Suspended,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl AutoTopUpStatus {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "ACTIVE",
            Self::Disabled => "DISABLED",
            Self::Suspended => "SUSPENDED",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for AutoTopUpStatus {
    fn from(value: &str) -> Self {
        match value {
            "ACTIVE" => Self::Active,
            "DISABLED" => Self::Disabled,
            "SUSPENDED" => Self::Suspended,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for AutoTopUpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for AutoTopUpStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AutoTopUpStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for AutoTopUpStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
