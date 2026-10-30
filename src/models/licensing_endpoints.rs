// this file is @generated
use serde::{Deserialize, Serialize};

/// The public endpoints your installations talk to.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct LicensingEndpoints {
    pub activate_url: String,

    pub events_url: String,

    pub jwks_url: String,

    pub refresh_url: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl LicensingEndpoints {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        activate_url: impl Into<String>,
        events_url: impl Into<String>,
        jwks_url: impl Into<String>,
        refresh_url: impl Into<String>,
    ) -> Self {
        Self {
            activate_url: activate_url.into(),
            events_url: events_url.into(),
            jwks_url: jwks_url.into(),
            refresh_url: refresh_url.into(),
            extra: serde_json::Map::new(),
        }
    }
}
