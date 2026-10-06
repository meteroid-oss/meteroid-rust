// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CustomerPortalTokenResponse {
    /// Base URL of the public REST API
    pub api_url: String,

    /// When the token expires (RFC 3339)
    pub expires_at: chrono::DateTime<chrono::Utc>,

    /// Hosted customer portal URL, token included
    pub portal_link: String,

    /// Base URL of the customer portal
    pub portal_url: String,

    /// JWT token for portal access
    pub token: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CustomerPortalTokenResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        api_url: impl Into<String>,
        expires_at: chrono::DateTime<chrono::Utc>,
        portal_link: impl Into<String>,
        portal_url: impl Into<String>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            api_url: api_url.into(),
            expires_at,
            portal_link: portal_link.into(),
            portal_url: portal_url.into(),
            token: token.into(),
            extra: serde_json::Map::new(),
        }
    }
}
