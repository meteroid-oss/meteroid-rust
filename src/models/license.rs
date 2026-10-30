// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    customer_id::CustomerId, license_id::LicenseId, license_status::LicenseStatus,
    subscription_id::SubscriptionId,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct License {
    /// Issued automatically for a subscription to a plan that issues licenses.
    pub auto_issued: bool,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub customer_id: CustomerId,

    pub customer_name: String,

    /// Whether the activation code can still be retrieved: only until activation or revocation.
    pub has_activation_code: bool,

    pub id: LicenseId,

    /// The installation the license is bound to, once activated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refreshed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Issued as a signed offline license: never activated nor refreshed.
    pub offline: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Revoked to stop at once: the installation drops its license on its next refresh instead
    /// of keeping it until it expires.
    pub revoked_immediately: bool,

    pub status: LicenseStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<SubscriptionId>,

    /// Lifetime of each license token the installation receives on refresh; for an offline
    /// license, the lifetime it was issued with.
    pub validity_days: i32,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl License {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        auto_issued: bool,
        created_at: chrono::DateTime<chrono::Utc>,
        customer_id: impl Into<CustomerId>,
        customer_name: impl Into<String>,
        has_activation_code: bool,
        id: impl Into<LicenseId>,
        offline: bool,
        revoked_immediately: bool,
        status: LicenseStatus,
        validity_days: i32,
    ) -> Self {
        Self {
            auto_issued,
            created_at,
            customer_id: customer_id.into(),
            customer_name: customer_name.into(),
            has_activation_code,
            id: id.into(),
            instance_id: None,
            last_refreshed_at: None,
            offline,
            plan_name: None,
            revoked_at: None,
            revoked_immediately,
            status,
            subscription_id: None,
            validity_days,
            extra: serde_json::Map::new(),
        }
    }
}
