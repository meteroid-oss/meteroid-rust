// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// PAID credit was bought, through Meteroid or outside it, and applies after tax like a payment.
/// PROMOTIONAL credit was given (included with a plan, a coupon, bonus credit, a goodwill grant):
/// it applies before tax, like a discount, so it is spent before paid credit. Credit-unit grants
/// are drawn together, whatever their type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CreditGrantType {
    Paid,
    Promotional,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CreditGrantType {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Paid => "PAID",
            Self::Promotional => "PROMOTIONAL",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CreditGrantType {
    fn from(value: &str) -> Self {
        match value {
            "PAID" => Self::Paid,
            "PROMOTIONAL" => Self::Promotional,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CreditGrantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CreditGrantType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CreditGrantType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CreditGrantType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
