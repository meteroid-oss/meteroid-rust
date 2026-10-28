// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum LedgerEntryType {
    Credit,
    InvoiceApplication,
    InvoiceReturn,
    CreditReversal,
    CreditReinstatement,
    CreditExpiry,
    CreditRollover,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl LedgerEntryType {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Credit => "CREDIT",
            Self::InvoiceApplication => "INVOICE_APPLICATION",
            Self::InvoiceReturn => "INVOICE_RETURN",
            Self::CreditReversal => "CREDIT_REVERSAL",
            Self::CreditReinstatement => "CREDIT_REINSTATEMENT",
            Self::CreditExpiry => "CREDIT_EXPIRY",
            Self::CreditRollover => "CREDIT_ROLLOVER",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for LedgerEntryType {
    fn from(value: &str) -> Self {
        match value {
            "CREDIT" => Self::Credit,
            "INVOICE_APPLICATION" => Self::InvoiceApplication,
            "INVOICE_RETURN" => Self::InvoiceReturn,
            "CREDIT_REVERSAL" => Self::CreditReversal,
            "CREDIT_REINSTATEMENT" => Self::CreditReinstatement,
            "CREDIT_EXPIRY" => Self::CreditExpiry,
            "CREDIT_ROLLOVER" => Self::CreditRollover,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for LedgerEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for LedgerEntryType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for LedgerEntryType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for LedgerEntryType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
