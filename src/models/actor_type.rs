// this file is @generated
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ActorType {
    User,
    Customer,
    System,
    ApiKey,
    OauthApp,
    /// A value this version of the SDK does not know yet.
    Unknown(String),
}

impl ActorType {
    /// The value as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "USER",
            Self::Customer => "CUSTOMER",
            Self::System => "SYSTEM",
            Self::ApiKey => "API_KEY",
            Self::OauthApp => "OAUTH_APP",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for ActorType {
    fn from(value: &str) -> Self {
        match value {
            "USER" => Self::User,
            "CUSTOMER" => Self::Customer,
            "SYSTEM" => Self::System,
            "API_KEY" => Self::ApiKey,
            "OAUTH_APP" => Self::OauthApp,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for ActorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for ActorType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ActorType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|value| Self::from(value.as_str()))
    }
}

impl crate::request::QueryParamValue for ActorType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
