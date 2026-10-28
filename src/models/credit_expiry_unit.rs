// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CreditExpiryUnit {
    Day,
    Month,
    BillingPeriod,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CreditExpiryUnit {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Day => "DAY",
            Self::Month => "MONTH",
            Self::BillingPeriod => "BILLING_PERIOD",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CreditExpiryUnit {
    fn from(value: &str) -> Self {
        match value {
            "DAY" => Self::Day,
            "MONTH" => Self::Month,
            "BILLING_PERIOD" => Self::BillingPeriod,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CreditExpiryUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CreditExpiryUnit {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CreditExpiryUnit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CreditExpiryUnit {
    fn encode(&self) -> String {
        self.to_string()
    }
}
