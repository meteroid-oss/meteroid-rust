// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Which charges credit pays: ALL pays any part of an invoice, USAGE only its usage charges.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CreditAppliesTo {
    All,
    Usage,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CreditAppliesTo {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "ALL",
            Self::Usage => "USAGE",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CreditAppliesTo {
    fn from(value: &str) -> Self {
        match value {
            "ALL" => Self::All,
            "USAGE" => Self::Usage,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CreditAppliesTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CreditAppliesTo {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CreditAppliesTo {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CreditAppliesTo {
    fn encode(&self) -> String {
        self.to_string()
    }
}
