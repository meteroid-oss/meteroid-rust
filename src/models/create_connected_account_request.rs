// this file is @generated
use serde::{Deserialize, Serialize};

use super::{country_code::CountryCode, onboarding_mode::OnboardingMode};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateConnectedAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<CountryCode>,

    pub email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<OnboardingMode>,

    pub organization_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CreateConnectedAccountRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(email: impl Into<String>, organization_name: impl Into<String>) -> Self {
        Self {
            country: None,
            email: email.into(),
            metadata: None,
            mode: None,
            organization_name: organization_name.into(),
            redirect_url: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `country`.
    #[must_use]
    pub fn country(mut self, country: impl Into<CountryCode>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Sets `metadata`.
    #[must_use]
    pub fn metadata(mut self, metadata: impl Into<serde_json::Value>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    /// Sets `mode`.
    #[must_use]
    pub fn mode(mut self, mode: impl Into<OnboardingMode>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    /// Sets `redirect_url`.
    #[must_use]
    pub fn redirect_url(mut self, redirect_url: impl Into<String>) -> Self {
        self.redirect_url = Some(redirect_url.into());
        self
    }
}
