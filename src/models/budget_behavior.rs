// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// What reaching the amount does. A SOFT budget only sends alerts; a HARD budget also makes the
/// entitlement check answer BUDGET_EXCEEDED for the features whose usage counts in its spend. A
/// HARD budget on one subscription refuses the checks naming that subscription, and the checks
/// naming none once every subscription charging the feature's metric is refused by its own.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum BudgetBehavior {
    Soft,
    Hard,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl BudgetBehavior {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Soft => "SOFT",
            Self::Hard => "HARD",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for BudgetBehavior {
    fn from(value: &str) -> Self {
        match value {
            "SOFT" => Self::Soft,
            "HARD" => Self::Hard,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for BudgetBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for BudgetBehavior {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for BudgetBehavior {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for BudgetBehavior {
    fn encode(&self) -> String {
        self.to_string()
    }
}
