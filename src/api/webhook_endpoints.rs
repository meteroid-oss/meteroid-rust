// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// The webhook endpoints API, from the client's
/// [`webhook_endpoints`](crate::api::Meteroid::webhook_endpoints).
#[derive(Clone)]
pub struct WebhookEndpoints {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for WebhookEndpoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookEndpoints")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl WebhookEndpoints {
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

    /// The endpoints API, with the options of this one.
    #[must_use]
    pub fn endpoints(&self) -> super::WebhookEndpointsEndpoints {
        super::WebhookEndpointsEndpoints::new(self.cfg.clone()).with_options(self.options.clone())
    }

    /// Resend a webhook delivery
    ///
    /// Re-queues the same event for the same endpoint. Fails if the endpoint is disabled.
    ///
    /// `POST /api/v1/webhooks/deliveries/{delivery_id}/resend`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn resend_webhook_delivery(
        &self,
        delivery_id: impl Into<crate::models::WebhookDeliveryId>,
    ) -> crate::api::Call<crate::models::WebhookDelivery> {
        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/webhooks/deliveries/{delivery_id}/resend",
        )
        .with_path_param(
            "delivery_id",
            Into::<crate::models::WebhookDeliveryId>::into(delivery_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
