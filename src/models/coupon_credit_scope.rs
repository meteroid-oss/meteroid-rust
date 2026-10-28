// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Which subscriptions a promotional credit coupon's credit pays.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CouponCreditScope {
    Customer,
    Subscription,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl CouponCreditScope {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Customer => "CUSTOMER",
            Self::Subscription => "SUBSCRIPTION",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for CouponCreditScope {
    fn from(value: &str) -> Self {
        match value {
            "CUSTOMER" => Self::Customer,
            "SUBSCRIPTION" => Self::Subscription,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for CouponCreditScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CouponCreditScope {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CouponCreditScope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for CouponCreditScope {
    fn encode(&self) -> String {
        self.to_string()
    }
}
