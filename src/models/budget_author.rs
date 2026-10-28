// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Who set the budget: you (TENANT) or the customer, from the portal (CUSTOMER).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum BudgetAuthor {
    Tenant,
    Customer,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl BudgetAuthor {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tenant => "TENANT",
            Self::Customer => "CUSTOMER",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for BudgetAuthor {
    fn from(value: &str) -> Self {
        match value {
            "TENANT" => Self::Tenant,
            "CUSTOMER" => Self::Customer,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for BudgetAuthor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for BudgetAuthor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for BudgetAuthor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for BudgetAuthor {
    fn encode(&self) -> String {
        self.to_string()
    }
}
