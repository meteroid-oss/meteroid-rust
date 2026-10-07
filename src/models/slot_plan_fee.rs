// this file is @generated
use serde::{Deserialize, Serialize};

use super::term_rate::TermRate;

/// Slot-based fee (e.g., per-seat pricing)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SlotPlanFee {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_count: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i32>,

    pub rates: Vec<TermRate>,

    pub slot_unit_name: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl SlotPlanFee {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(rates: Vec<TermRate>, slot_unit_name: impl Into<String>) -> Self {
        Self {
            minimum_count: None,
            quota: None,
            rates,
            slot_unit_name: slot_unit_name.into(),
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `minimum_count`.
    #[must_use]
    pub fn minimum_count(mut self, minimum_count: impl Into<i32>) -> Self {
        self.minimum_count = Some(minimum_count.into());
        self
    }

    /// Sets `quota`.
    #[must_use]
    pub fn quota(mut self, quota: impl Into<i32>) -> Self {
        self.quota = Some(quota.into());
        self
    }
}
