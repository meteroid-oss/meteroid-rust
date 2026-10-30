// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum LicenseStatus {
    Pending,
    Active,
    Revoked,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl LicenseStatus {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "PENDING",
            Self::Active => "ACTIVE",
            Self::Revoked => "REVOKED",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for LicenseStatus {
    fn from(value: &str) -> Self {
        match value {
            "PENDING" => Self::Pending,
            "ACTIVE" => Self::Active,
            "REVOKED" => Self::Revoked,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for LicenseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for LicenseStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for LicenseStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for LicenseStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
