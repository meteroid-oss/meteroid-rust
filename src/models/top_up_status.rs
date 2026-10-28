// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TopUpStatus {
    Succeeded,
    Pending,
    RequiresCustomerAction,
    AwaitingPayment,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl TopUpStatus {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Succeeded => "SUCCEEDED",
            Self::Pending => "PENDING",
            Self::RequiresCustomerAction => "REQUIRES_CUSTOMER_ACTION",
            Self::AwaitingPayment => "AWAITING_PAYMENT",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for TopUpStatus {
    fn from(value: &str) -> Self {
        match value {
            "SUCCEEDED" => Self::Succeeded,
            "PENDING" => Self::Pending,
            "REQUIRES_CUSTOMER_ACTION" => Self::RequiresCustomerAction,
            "AWAITING_PAYMENT" => Self::AwaitingPayment,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for TopUpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for TopUpStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TopUpStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for TopUpStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
