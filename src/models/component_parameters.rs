// this file is @generated
use serde::{Deserialize, Serialize};

use super::billing_period_enum::BillingPeriodEnum;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ComponentParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<BillingPeriodEnum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed_capacity: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_slot_count: Option<i32>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl ComponentParameters {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            billing_period: None,
            committed_capacity: None,
            initial_slot_count: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `billing_period`.
    #[must_use]
    pub fn billing_period(mut self, billing_period: impl Into<BillingPeriodEnum>) -> Self {
        self.billing_period = Some(billing_period.into());
        self
    }

    /// Sets `committed_capacity`.
    #[must_use]
    pub fn committed_capacity(mut self, committed_capacity: impl Into<i64>) -> Self {
        self.committed_capacity = Some(committed_capacity.into());
        self
    }

    /// Sets `initial_slot_count`.
    #[must_use]
    pub fn initial_slot_count(mut self, initial_slot_count: impl Into<i32>) -> Self {
        self.initial_slot_count = Some(initial_slot_count.into());
        self
    }
}
