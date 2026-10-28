// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Why a credit grant's unspent credit lapsed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CreditGrantLapseReason {
    Expired,
    SubscriptionEnded,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CreditGrantLapseReason {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Expired => "EXPIRED",
            Self::SubscriptionEnded => "SUBSCRIPTION_ENDED",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CreditGrantLapseReason {
    fn from(value: &str) -> Self {
        match value {
            "EXPIRED" => Self::Expired,
            "SUBSCRIPTION_ENDED" => Self::SubscriptionEnded,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CreditGrantLapseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CreditGrantLapseReason {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CreditGrantLapseReason {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CreditGrantLapseReason {
    fn encode(&self) -> String {
        self.to_string()
    }
}
