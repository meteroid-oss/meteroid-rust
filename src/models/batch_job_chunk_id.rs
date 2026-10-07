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
pub struct BatchJobChunkId(String);

impl BatchJobChunkId {
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

impl std::ops::Deref for BatchJobChunkId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BatchJobChunkId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for BatchJobChunkId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for BatchJobChunkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for BatchJobChunkId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for BatchJobChunkId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<&String> for BatchJobChunkId {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<&BatchJobChunkId> for BatchJobChunkId {
    fn from(value: &BatchJobChunkId) -> Self {
        value.clone()
    }
}

impl From<BatchJobChunkId> for String {
    fn from(value: BatchJobChunkId) -> Self {
        value.0
    }
}

impl PartialEq<str> for BatchJobChunkId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for BatchJobChunkId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for BatchJobChunkId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl crate::request::QueryParamValue for BatchJobChunkId {
    fn encode(&self) -> String {
        self.0.clone()
    }
}
