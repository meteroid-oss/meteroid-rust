// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CreditGrantSource {
    Manual,
    Purchase,
    Plan,
    CreditNote,
    Adjustment,
    Migrated,
    Rollover,
    Coupon,
    Overpayment,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CreditGrantSource {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Manual => "MANUAL",
            Self::Purchase => "PURCHASE",
            Self::Plan => "PLAN",
            Self::CreditNote => "CREDIT_NOTE",
            Self::Adjustment => "ADJUSTMENT",
            Self::Migrated => "MIGRATED",
            Self::Rollover => "ROLLOVER",
            Self::Coupon => "COUPON",
            Self::Overpayment => "OVERPAYMENT",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CreditGrantSource {
    fn from(value: &str) -> Self {
        match value {
            "MANUAL" => Self::Manual,
            "PURCHASE" => Self::Purchase,
            "PLAN" => Self::Plan,
            "CREDIT_NOTE" => Self::CreditNote,
            "ADJUSTMENT" => Self::Adjustment,
            "MIGRATED" => Self::Migrated,
            "ROLLOVER" => Self::Rollover,
            "COUPON" => Self::Coupon,
            "OVERPAYMENT" => Self::Overpayment,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CreditGrantSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CreditGrantSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CreditGrantSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CreditGrantSource {
    fn encode(&self) -> String {
        self.to_string()
    }
}
