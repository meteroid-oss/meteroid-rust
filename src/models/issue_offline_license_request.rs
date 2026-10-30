// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IssueOfflineLicenseRequest {
    /// The installation's id as it reports it, up to 255 characters without spaces: the license
    /// names it (`aud`) and stays bound to it. Without one, the license is not tied to one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// Lifetime of the offline license, 1 to 400 days.
    pub valid_days: i32,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl IssueOfflineLicenseRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(valid_days: i32) -> Self {
        Self {
            instance_id: None,
            valid_days,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `instance_id`.
    #[must_use]
    pub fn instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.instance_id = Some(instance_id.into());
        self
    }
}
