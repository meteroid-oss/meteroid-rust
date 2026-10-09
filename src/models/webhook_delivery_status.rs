// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum WebhookDeliveryStatus {
    Pending,
    InFlight,
    Succeeded,
    Failed,
    Cancelled,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl WebhookDeliveryStatus {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "PENDING",
            Self::InFlight => "IN_FLIGHT",
            Self::Succeeded => "SUCCEEDED",
            Self::Failed => "FAILED",
            Self::Cancelled => "CANCELLED",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for WebhookDeliveryStatus {
    fn from(value: &str) -> Self {
        match value {
            "PENDING" => Self::Pending,
            "IN_FLIGHT" => Self::InFlight,
            "SUCCEEDED" => Self::Succeeded,
            "FAILED" => Self::Failed,
            "CANCELLED" => Self::Cancelled,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for WebhookDeliveryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for WebhookDeliveryStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WebhookDeliveryStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for WebhookDeliveryStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
