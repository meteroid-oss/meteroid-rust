// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// How a top-up is paid: CHARGE a saved payment method now, or SEND_INVOICE and grant the credit
/// once the invoice is paid.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TopUpCollection {
    Charge,
    SendInvoice,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl TopUpCollection {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Charge => "CHARGE",
            Self::SendInvoice => "SEND_INVOICE",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for TopUpCollection {
    fn from(value: &str) -> Self {
        match value {
            "CHARGE" => Self::Charge,
            "SEND_INVOICE" => Self::SendInvoice,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for TopUpCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for TopUpCollection {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TopUpCollection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for TopUpCollection {
    fn encode(&self) -> String {
        self.to_string()
    }
}
