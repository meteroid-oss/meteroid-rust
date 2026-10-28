// this file is @generated
use serde::{Deserialize, Serialize};

/// Restricts the customer's usage events a subscription bills to those whose `property` equals
/// `value` (for example `api_key` = `key-A`). Among the customer's subscriptions pricing the same
/// metric, one with a usage filter bills the events matching it (an event two filters match goes
/// to the older subscription); one without bills the events matching no other subscription's
/// filter. Two subscriptions without a usage filter both bill every such event. Events matching
/// no filter while every subscription pricing their metric has one are billed by none and
/// reported as unattributed usage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UsageFilter {
    /// Event property name, e.g. `api_key`.
    pub property: String,

    /// Value the property must have.
    pub value: String,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl UsageFilter {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(property: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
            extra: serde_json::Map::new(),
        }
    }
}
