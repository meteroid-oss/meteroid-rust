// this file is @generated
use serde::{Deserialize, Serialize};

use super::customer_portal_scope::CustomerPortalScope;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerPortalTokenRequest {
    /// Token lifetime in seconds. Defaults to 86400 (24 hours).
    /// Must be between 60 and 2592000 (30 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_seconds: Option<i32>,

    /// Scopes granted to the token. Defaults to `["read", "manage"]`.
    /// Use `["read"]` for tokens that only read billing state, e.g. to gate features in a browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<CustomerPortalScope>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CustomerPortalTokenRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            expires_in_seconds: None,
            scopes: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `expires_in_seconds`.
    #[must_use]
    pub fn expires_in_seconds(mut self, expires_in_seconds: impl Into<i32>) -> Self {
        self.expires_in_seconds = Some(expires_in_seconds.into());
        self
    }

    /// Sets `scopes`.
    #[must_use]
    pub fn scopes(mut self, scopes: impl Into<Vec<CustomerPortalScope>>) -> Self {
        self.scopes = Some(scopes.into());
        self
    }
}
