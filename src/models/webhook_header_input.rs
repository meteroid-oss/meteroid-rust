// this file is @generated
use serde::{Deserialize, Serialize};

/// A custom header to send with every delivery. A sensitive header is write-only:
/// it is never returned, and on update sending it without a value keeps its value.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WebhookHeaderInput {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WebhookHeaderInput {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sensitive: None,
            value: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `sensitive`.
    #[must_use]
    pub fn sensitive(mut self, sensitive: impl Into<bool>) -> Self {
        self.sensitive = Some(sensitive.into());
        self
    }

    /// Sets `value`.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}
