// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// How a voluntary refund was issued: through the provider, or recorded after a wire or cash
/// movement made outside Meteroid.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum RefundMode {
    Online,
    Offline,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl RefundMode {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Online => "ONLINE",
            Self::Offline => "OFFLINE",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for RefundMode {
    fn from(value: &str) -> Self {
        match value {
            "ONLINE" => Self::Online,
            "OFFLINE" => Self::Offline,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for RefundMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for RefundMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for RefundMode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for RefundMode {
    fn encode(&self) -> String {
        self.to_string()
    }
}
