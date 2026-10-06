// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// What a customer portal token may do.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CustomerPortalScope {
    Read,
    Manage,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CustomerPortalScope {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Read => "read",
            Self::Manage => "manage",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CustomerPortalScope {
    fn from(value: &str) -> Self {
        match value {
            "read" => Self::Read,
            "manage" => Self::Manage,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CustomerPortalScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CustomerPortalScope {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CustomerPortalScope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CustomerPortalScope {
    fn encode(&self) -> String {
        self.to_string()
    }
}
