// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EntitlementCheckReason {
    WithinLimit,
    LimitReached,
    NoEntitlement,
    Disabled,
    BudgetExceeded,
    InsufficientBalance,
    TooManyGroups,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl EntitlementCheckReason {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::WithinLimit => "WITHIN_LIMIT",
            Self::LimitReached => "LIMIT_REACHED",
            Self::NoEntitlement => "NO_ENTITLEMENT",
            Self::Disabled => "DISABLED",
            Self::BudgetExceeded => "BUDGET_EXCEEDED",
            Self::InsufficientBalance => "INSUFFICIENT_BALANCE",
            Self::TooManyGroups => "TOO_MANY_GROUPS",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for EntitlementCheckReason {
    fn from(value: &str) -> Self {
        match value {
            "WITHIN_LIMIT" => Self::WithinLimit,
            "LIMIT_REACHED" => Self::LimitReached,
            "NO_ENTITLEMENT" => Self::NoEntitlement,
            "DISABLED" => Self::Disabled,
            "BUDGET_EXCEEDED" => Self::BudgetExceeded,
            "INSUFFICIENT_BALANCE" => Self::InsufficientBalance,
            "TOO_MANY_GROUPS" => Self::TooManyGroups,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for EntitlementCheckReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for EntitlementCheckReason {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for EntitlementCheckReason {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for EntitlementCheckReason {
    fn encode(&self) -> String {
        self.to_string()
    }
}
