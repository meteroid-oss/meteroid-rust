// this file is @generated

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(transparent)]
pub struct TenantId(String);

impl TenantId {
    /// The value, as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The value as a `String`.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::ops::Deref for TenantId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TenantId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for TenantId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for TenantId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for TenantId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<&String> for TenantId {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<&TenantId> for TenantId {
    fn from(value: &TenantId) -> Self {
        value.clone()
    }
}

impl From<TenantId> for String {
    fn from(value: TenantId) -> Self {
        value.0
    }
}

impl PartialEq<str> for TenantId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for TenantId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for TenantId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl crate::request::QueryParamValue for TenantId {
    fn encode(&self) -> String {
        self.0.clone()
    }
}
