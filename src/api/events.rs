// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// The events API, from the client's
/// [`events`](crate::api::Meteroid::events).
#[derive(Clone)]
pub struct Events {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Events")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Events {
    pub(super) fn new(cfg: std::sync::Arc<Configuration>) -> Self {
        Self {
            cfg,
            options: crate::api::RequestOptions::default(),
        }
    }

    /// Applies `options` (headers, timeout, retries, idempotency key) to the calls made
    /// through the returned value.
    #[must_use]
    pub fn with_options(mut self, options: crate::api::RequestOptions) -> Self {
        self.options = options;
        self
    }

    /// Ingest events
    ///
    /// Ingest usage events for metering and billing purposes.
    ///
    /// Events are deduplicated by `(event_id, customer_id)` — re-sending the same pair will not be
    /// double-counted. The first copy received is kept and later copies are ignored, whatever they change.
    ///
    /// By default, any invalid event rejects the entire batch. Set `allow_partial_failures` to `true` to ingest valid events and receive per-event failure details in the response body.
    ///
    /// `POST /api/v1/events/ingest`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 429, 500).
    pub fn ingest(
        &self,
        ingest_events_request: crate::models::IngestEventsRequest,
    ) -> crate::api::Call<crate::models::IngestEventsResponse> {
        crate::request::Request::new(http::Method::POST, "/api/v1/events/ingest")
            .with_body_param(ingest_events_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Amend an event
    ///
    /// Replaces the timestamp and the properties of an ingested event. The code and the customer
    /// cannot change: void the event and send a new one instead. The change is applied asynchronously,
    /// in order with the event's other copies.
    ///
    /// Only events ingested in the last 35 days can be amended, and neither their current nor their
    /// new timestamp can be in an invoiced period. Tenants that keep the last copy of an event correct
    /// it by sending it again instead.
    ///
    /// An event is found once metering has stored it: a just-ingested event can still return `404`.
    /// Wait until an amend is visible before sending the next change to the same event; concurrent
    /// changes resolve by `ingested_at`, the latest wins.
    ///
    /// `POST /api/v1/events/{event_id}/amend`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 403, 404, 409, 429, 500, 503).
    pub fn amend(
        &self,
        event_id: &str,
        amend_event_request: crate::models::AmendEventRequest,
    ) -> crate::api::Call<crate::models::EventChangeResponse> {
        crate::request::Request::new(http::Method::POST, "/api/v1/events/{event_id}/amend")
            .with_path_param("event_id", event_id)
            .with_body_param(amend_event_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Void an event
    ///
    /// Cancels an ingested event: it no longer counts in usage. The change is applied asynchronously,
    /// in order with the event's other copies.
    ///
    /// Only events ingested in the last 35 days, outside an invoiced period, can be voided. Tenants
    /// that keep the last copy of an event correct it by sending it again instead.
    ///
    /// An event is found once metering has stored it: a just-ingested event can still return `404`,
    /// and so does a retried void once the first one is applied. Wait until a change is visible
    /// before sending the next one to the same event; concurrent changes resolve by `ingested_at`.
    ///
    /// `POST /api/v1/events/{event_id}/void`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 403, 404, 409, 429, 500, 503).
    pub fn void(
        &self,
        event_id: &str,
        void_event_request: crate::models::VoidEventRequest,
    ) -> crate::api::Call<crate::models::EventChangeResponse> {
        crate::request::Request::new(http::Method::POST, "/api/v1/events/{event_id}/void")
            .with_path_param("event_id", event_id)
            .with_body_param(void_event_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }
}
