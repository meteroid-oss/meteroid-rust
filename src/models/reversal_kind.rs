// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Why a payment was involuntarily clawed back.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ReversalKind {
    Refund,
    Chargeback,
    DebtorRecall,
    InsufficientFunds,
    MandateInvalid,
    Returned,
    Dispute,
    Other,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl ReversalKind {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Refund => "REFUND",
            Self::Chargeback => "CHARGEBACK",
            Self::DebtorRecall => "DEBTOR_RECALL",
            Self::InsufficientFunds => "INSUFFICIENT_FUNDS",
            Self::MandateInvalid => "MANDATE_INVALID",
            Self::Returned => "RETURNED",
            Self::Dispute => "DISPUTE",
            Self::Other => "OTHER",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for ReversalKind {
    fn from(value: &str) -> Self {
        match value {
            "REFUND" => Self::Refund,
            "CHARGEBACK" => Self::Chargeback,
            "DEBTOR_RECALL" => Self::DebtorRecall,
            "INSUFFICIENT_FUNDS" => Self::InsufficientFunds,
            "MANDATE_INVALID" => Self::MandateInvalid,
            "RETURNED" => Self::Returned,
            "DISPUTE" => Self::Dispute,
            "OTHER" => Self::Other,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for ReversalKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for ReversalKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ReversalKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for ReversalKind {
    fn encode(&self) -> String {
        self.to_string()
    }
}
