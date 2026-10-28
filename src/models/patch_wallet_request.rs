// this file is @generated
use serde::{Deserialize, Serialize};

use super::{auto_top_up_write_request::AutoTopUpWriteRequest, usage_at_zero::UsageAtZero};

/// Partial update of a wallet's configuration: omit a field to leave it unchanged.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PatchWalletRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub auto_top_up: Option<Option<AutoTopUpWriteRequest>>,

    /// Ascending, distinct, at most 5, non-negative. Omit to leave unchanged; an empty list
    /// clears every threshold.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub low_balance_thresholds: Option<Option<Vec<i64>>>,

    /// Credit unit wallets: the price this customer agreed for one credit, in major units of the
    /// customer's currency, as a decimal string (positive, at most 12 decimal places). Purchases
    /// and automatic top-ups default to it instead of the list price. Omit to leave it unchanged;
    /// `null` returns the customer to the list price. A change applies to purchases quoted after
    /// it. An automatic top-up keeps running at the new price when it is at or below the price
    /// its terms were authorized at, and stops until they are confirmed again when it is above.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub unit_price: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::models::codec::nullable"
    )]
    pub usage_at_zero: Option<Option<UsageAtZero>>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl PatchWalletRequest {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            auto_top_up: None,
            low_balance_thresholds: None,
            unit_price: None,
            usage_at_zero: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `auto_top_up`.
    #[must_use]
    pub fn auto_top_up(mut self, auto_top_up: impl Into<AutoTopUpWriteRequest>) -> Self {
        self.auto_top_up = Some(Some(auto_top_up.into()));
        self
    }

    /// Sends `auto_top_up` as `null`, clearing it.
    #[must_use]
    pub fn clear_auto_top_up(mut self) -> Self {
        self.auto_top_up = Some(None);
        self
    }

    /// Sets `low_balance_thresholds`.
    #[must_use]
    pub fn low_balance_thresholds(mut self, low_balance_thresholds: impl Into<Vec<i64>>) -> Self {
        self.low_balance_thresholds = Some(Some(low_balance_thresholds.into()));
        self
    }

    /// Sends `low_balance_thresholds` as `null`, clearing it.
    #[must_use]
    pub fn clear_low_balance_thresholds(mut self) -> Self {
        self.low_balance_thresholds = Some(None);
        self
    }

    /// Sets `unit_price`.
    #[must_use]
    pub fn unit_price(mut self, unit_price: impl Into<String>) -> Self {
        self.unit_price = Some(Some(unit_price.into()));
        self
    }

    /// Sends `unit_price` as `null`, clearing it.
    #[must_use]
    pub fn clear_unit_price(mut self) -> Self {
        self.unit_price = Some(None);
        self
    }

    /// Sets `usage_at_zero`.
    #[must_use]
    pub fn usage_at_zero(mut self, usage_at_zero: impl Into<UsageAtZero>) -> Self {
        self.usage_at_zero = Some(Some(usage_at_zero.into()));
        self
    }

    /// Sends `usage_at_zero` as `null`, clearing it.
    #[must_use]
    pub fn clear_usage_at_zero(mut self) -> Self {
        self.usage_at_zero = Some(None);
        self
    }
}
