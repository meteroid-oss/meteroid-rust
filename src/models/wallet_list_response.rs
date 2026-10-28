// this file is @generated
use serde::{Deserialize, Serialize};

use super::wallet::Wallet;

/// A customer's wallets, the customer's own currency first.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WalletListResponse {
    pub data: Vec<Wallet>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl WalletListResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(data: Vec<Wallet>) -> Self {
        Self {
            data,
            extra: serde_json::Map::new(),
        }
    }
}
