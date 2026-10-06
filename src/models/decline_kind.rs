// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Why the payment provider declined a charge.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DeclineKind {
    InsufficientFunds,
    DoNotHonor,
    CardExpired,
    AuthenticationRequired,
    MandateInactive,
    Fraud,
    ProcessingError,
    Other,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl DeclineKind {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::InsufficientFunds => "INSUFFICIENT_FUNDS",
            Self::DoNotHonor => "DO_NOT_HONOR",
            Self::CardExpired => "CARD_EXPIRED",
            Self::AuthenticationRequired => "AUTHENTICATION_REQUIRED",
            Self::MandateInactive => "MANDATE_INACTIVE",
            Self::Fraud => "FRAUD",
            Self::ProcessingError => "PROCESSING_ERROR",
            Self::Other => "OTHER",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for DeclineKind {
    fn from(value: &str) -> Self {
        match value {
            "INSUFFICIENT_FUNDS" => Self::InsufficientFunds,
            "DO_NOT_HONOR" => Self::DoNotHonor,
            "CARD_EXPIRED" => Self::CardExpired,
            "AUTHENTICATION_REQUIRED" => Self::AuthenticationRequired,
            "MANDATE_INACTIVE" => Self::MandateInactive,
            "FRAUD" => Self::Fraud,
            "PROCESSING_ERROR" => Self::ProcessingError,
            "OTHER" => Self::Other,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for DeclineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for DeclineKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DeclineKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for DeclineKind {
    fn encode(&self) -> String {
        self.to_string()
    }
}
