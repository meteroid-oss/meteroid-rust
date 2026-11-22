// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    plan_version_id::PlanVersionId, scheduled_event_id::ScheduledEventId,
    scheduled_event_type::ScheduledEventType,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PendingScheduledEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,

    pub customer_initiated: bool,

    pub event_type: ScheduledEventType,

    pub id: ScheduledEventId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_plan_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_plan_version_id: Option<PlanVersionId>,

    /// Date the event takes effect at the start of
    pub scheduled_date: chrono::NaiveDate,

    /// Properties this version of the SDK does not know, sent back as received.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl PendingScheduledEvent {
    /// Creates a value from its required fields.
    #[must_use]
    pub fn new(
        customer_initiated: bool,
        event_type: ScheduledEventType,
        id: impl Into<ScheduledEventId>,
        scheduled_date: chrono::NaiveDate,
    ) -> Self {
        Self {
            cancel_reason: None,
            customer_initiated,
            event_type,
            id: id.into(),
            new_plan_name: None,
            new_plan_version_id: None,
            scheduled_date,
            extra: serde_json::Map::new(),
        }
    }
}
