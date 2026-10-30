// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IssueLicenseRequest {
    /// Customer ID or alias
    pub customer_id: String,

    /// Offline only. The installation's id as it reports it, up to 255 characters without
    /// spaces: the license names it (`aud`) and stays bound to it. Without one, the license is
    /// not tied to an installation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// Delivers the license at once as a signed offline license, for an installation that
    /// can't reach the license server: no activation code, no refresh. Default: false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,

    /// Ties the license to this subscription of the customer: it is renewed while the
    /// subscription is paid, and carries its plan's entitlements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    /// Online: lifetime of each license token the installation refreshes, 1 to 31 days.
    /// Default: 7. Offline: lifetime of the signed license, 1 to 400 days. Default: 365.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_days: Option<i32>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl IssueLicenseRequest {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(customer_id: impl Into<String>) -> Self {
        Self {
            customer_id: customer_id.into(),
            instance_id: None,
            offline: None,
            subscription_id: None,
            validity_days: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `instance_id`.
    #[must_use]
    pub fn instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.instance_id = Some(instance_id.into());
        self
    }

    /// Sets `offline`.
    #[must_use]
    pub fn offline(mut self, offline: impl Into<bool>) -> Self {
        self.offline = Some(offline.into());
        self
    }

    /// Sets `subscription_id`.
    #[must_use]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    /// Sets `validity_days`.
    #[must_use]
    pub fn validity_days(mut self, validity_days: impl Into<i32>) -> Self {
        self.validity_days = Some(validity_days.into());
        self
    }
}
