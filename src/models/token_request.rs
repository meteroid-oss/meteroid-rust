// this file is @generated
use serde::{Deserialize, Serialize};

/// Token request (from POST body, application/x-www-form-urlencoded)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TokenRequest {
    /// Client ID (if not using HTTP Basic auth)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// Client secret (if not using HTTP Basic auth)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Authorization code (for authorization_code grant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// PKCE code verifier (for authorization_code grant with PKCE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,

    /// Grant type: "authorization_code" or "refresh_token"
    pub grant_type: String,

    /// Redirect URI (for authorization_code grant, must match the one used in /authorize)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,

    /// Refresh token (for refresh_token grant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl TokenRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(grant_type: impl Into<String>) -> Self {
        Self {
            client_id: None,
            client_secret: None,
            code: None,
            code_verifier: None,
            grant_type: grant_type.into(),
            redirect_uri: None,
            refresh_token: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `client_id`.
    #[must_use]
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Sets `client_secret`.
    #[must_use]
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    /// Sets `code`.
    #[must_use]
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Sets `code_verifier`.
    #[must_use]
    pub fn code_verifier(mut self, code_verifier: impl Into<String>) -> Self {
        self.code_verifier = Some(code_verifier.into());
        self
    }

    /// Sets `redirect_uri`.
    #[must_use]
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    /// Sets `refresh_token`.
    #[must_use]
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = Some(refresh_token.into());
        self
    }
}
