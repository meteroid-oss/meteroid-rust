// this file is @generated
use serde::{Deserialize, Serialize};

use super::calendar_unit::CalendarUnit;

/// Always ends at now — e.g. 30 days means the last 30 days, old usage drops off automatically.
/// A sliding window has no reset moment (`reset_at` is absent) and sends no threshold or
/// limit-reached webhooks; it is enforced by the entitlement check.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SlidingWindowResetPeriod {
    pub interval: i32,

    pub r#type: String,

    pub unit: CalendarUnit,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl SlidingWindowResetPeriod {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(interval: i32, unit: CalendarUnit) -> Self {
        Self {
            interval,
            r#type: "SLIDING_WINDOW".into(),
            unit,
            extra: serde_json::Map::new(),
        }
    }
}
