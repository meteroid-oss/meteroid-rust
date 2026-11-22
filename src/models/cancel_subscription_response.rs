// this file is @generated
use serde::{Deserialize, Serialize};

use super::{pending_scheduled_event::PendingScheduledEvent, subscription::Subscription};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct CancelSubscriptionResponse {
    /// Changes still scheduled after this request, including the cancellation.
    pub pending_events: Vec<PendingScheduledEvent>,

    /// Date the subscription ends at the start of: its first day without service. Empty when the
    /// cancellation applied immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_cancellation_date: Option<chrono::NaiveDate>,

    pub subscription: Subscription,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl CancelSubscriptionResponse {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(pending_events: Vec<PendingScheduledEvent>, subscription: Subscription) -> Self {
        Self {
            pending_events,
            scheduled_cancellation_date: None,
            subscription,
            extra: serde_json::Map::new(),
        }
    }
}
