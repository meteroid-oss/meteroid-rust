// this file is @generated
use serde::{Deserialize, Serialize};

use super::license_signing_key_status::LicenseSigningKeyStatus;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct LicenseSigningKey {
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The key id, the `kid` header of the licenses it signs.
    pub kid: String,

    /// Ed25519 public key, PEM-encoded.
    pub public_key_pem: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retired_at: Option<chrono::DateTime<chrono::Utc>>,

    pub status: LicenseSigningKeyStatus,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl LicenseSigningKey {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        kid: impl Into<String>,
        public_key_pem: impl Into<String>,
        status: LicenseSigningKeyStatus,
    ) -> Self {
        Self {
            created_at,
            kid: kid.into(),
            public_key_pem: public_key_pem.into(),
            retired_at: None,
            status,
            extra: serde_json::Map::new(),
        }
    }
}
