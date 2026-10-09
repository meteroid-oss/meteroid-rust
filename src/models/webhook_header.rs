// this file is @generated
use serde::{Deserialize, Serialize};

/// A custom header sent with every delivery. A sensitive header never returns its
/// value; `set` says whether it has one.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WebhookHeader {
    pub name: String,

    pub sensitive: bool,

    pub set: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WebhookHeader {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(name: impl Into<String>, sensitive: bool, set: bool) -> Self {
        Self {
            name: name.into(),
            sensitive,
            set,
            value: None,
            extra: serde_json::Map::new(),
        }
    }
}
