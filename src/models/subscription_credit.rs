// this file is @generated
use serde::{Deserialize, Serialize};

/// Credit on account that only one subscription can use.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SubscriptionCredit {
    /// In minor units of the wallet.
    pub credit_on_account: i64,

    pub subscription_id: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl SubscriptionCredit {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(credit_on_account: i64, subscription_id: impl Into<String>) -> Self {
        Self {
            credit_on_account,
            subscription_id: subscription_id.into(),
            extra: serde_json::Map::new(),
        }
    }
}
