// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BillingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycles: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_terms: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start_day: Option<i32>,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl BillingConfig {
    /// Creates a value with every field unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            billing_cycles: None,
            net_terms: None,
            period_start_day: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Sets `billing_cycles`.
    #[must_use]
    pub fn billing_cycles(mut self, billing_cycles: impl Into<i32>) -> Self {
        self.billing_cycles = Some(billing_cycles.into());
        self
    }

    /// Sets `net_terms`.
    #[must_use]
    pub fn net_terms(mut self, net_terms: impl Into<i32>) -> Self {
        self.net_terms = Some(net_terms.into());
        self
    }

    /// Sets `period_start_day`.
    #[must_use]
    pub fn period_start_day(mut self, period_start_day: impl Into<i32>) -> Self {
        self.period_start_day = Some(period_start_day.into());
        self
    }
}
