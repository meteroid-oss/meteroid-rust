// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// What a customer's usage does once the credit it can use runs out: what the subscription's plan
/// says (`FOLLOW_PLAN`), or always or never refused for this customer.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum UsageAtZero {
    FollowPlan,
    AlwaysPause,
    NeverPause,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl UsageAtZero {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::FollowPlan => "FOLLOW_PLAN",
            Self::AlwaysPause => "ALWAYS_PAUSE",
            Self::NeverPause => "NEVER_PAUSE",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for UsageAtZero {
    fn from(value: &str) -> Self {
        match value {
            "FOLLOW_PLAN" => Self::FollowPlan,
            "ALWAYS_PAUSE" => Self::AlwaysPause,
            "NEVER_PAUSE" => Self::NeverPause,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for UsageAtZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for UsageAtZero {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for UsageAtZero {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for UsageAtZero {
    fn encode(&self) -> String {
        self.to_string()
    }
}
