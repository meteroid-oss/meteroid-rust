// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// What happens once usage reaches the limit.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EntitlementOverageBehavior {
    Block,
    Allow,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl EntitlementOverageBehavior {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Block => "BLOCK",
            Self::Allow => "ALLOW",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for EntitlementOverageBehavior {
    fn from(value: &str) -> Self {
        match value {
            "BLOCK" => Self::Block,
            "ALLOW" => Self::Allow,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for EntitlementOverageBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for EntitlementOverageBehavior {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for EntitlementOverageBehavior {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for EntitlementOverageBehavior {
    fn encode(&self) -> String {
        self.to_string()
    }
}
