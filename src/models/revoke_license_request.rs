// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RevokeLicenseRequest {
    /// Stop at once: the installation drops its license on its next refresh. Otherwise the
    /// license keeps working until it expires, and is not renewed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediately: Option<bool>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl RevokeLicenseRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            immediately: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `immediately`.
    #[must_use]
    pub fn immediately(mut self, immediately: impl Into<bool>) -> Self {
        self.immediately = Some(immediately.into());
        self
    }
}
