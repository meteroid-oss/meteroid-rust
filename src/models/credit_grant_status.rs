// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CreditGrantStatus {
    Pending,
    Active,
    Depleted,
    Expired,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CreditGrantStatus {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "PENDING",
            Self::Active => "ACTIVE",
            Self::Depleted => "DEPLETED",
            Self::Expired => "EXPIRED",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CreditGrantStatus {
    fn from(value: &str) -> Self {
        match value {
            "PENDING" => Self::Pending,
            "ACTIVE" => Self::Active,
            "DEPLETED" => Self::Depleted,
            "EXPIRED" => Self::Expired,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CreditGrantStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CreditGrantStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CreditGrantStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CreditGrantStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
