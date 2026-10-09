// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`WebhookEndpointsEndpoints::list_deliveries`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WebhookEndpointsEndpointsListDeliveriesOptions {
    /// Only return deliveries in this state.
    pub status: Option<WebhookDeliveryStatus>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl WebhookEndpointsEndpointsListDeliveriesOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            status: None,
            page: None,
            per_page: None,
        }
    }

    /// Sets the `status` query parameter.
    #[must_use]
    pub fn status(mut self, status: impl Into<WebhookDeliveryStatus>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Sets the `status` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_status(mut self, status: Option<WebhookDeliveryStatus>) -> Self {
        self.status = status;
        self
    }

    /// Sets the `page` query parameter.
    #[must_use]
    pub fn page(mut self, page: impl Into<i32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sets the `page` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_page(mut self, page: Option<i32>) -> Self {
        self.page = page;
        self
    }

    /// Sets the `per_page` query parameter.
    #[must_use]
    pub fn per_page(mut self, per_page: impl Into<i32>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Sets the `per_page` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_per_page(mut self, per_page: Option<i32>) -> Self {
        self.per_page = per_page;
        self
    }
}

/// The webhook endpoints endpoints API, from
/// [`WebhookEndpoints::endpoints`](crate::api::WebhookEndpoints::endpoints).
#[derive(Clone)]
pub struct WebhookEndpointsEndpoints {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for WebhookEndpointsEndpoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookEndpointsEndpoints")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl WebhookEndpointsEndpoints {
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

    /// List webhook endpoints
    ///
    /// `GET /api/v1/webhooks/endpoints`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 429).
    pub fn list(&self) -> crate::api::Call<crate::models::WebhookEndpointListResponse> {
        crate::request::Request::new(http::Method::GET, "/api/v1/webhooks/endpoints")
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Create a webhook endpoint
    ///
    /// The signing secret is returned once, in this response only.
    ///
    /// `POST /api/v1/webhooks/endpoints`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 409, 429).
    pub fn create(
        &self,
        create_webhook_endpoint_request: crate::models::CreateWebhookEndpointRequest,
    ) -> crate::api::Call<crate::models::CreatedWebhookEndpoint> {
        crate::request::Request::new(http::Method::POST, "/api/v1/webhooks/endpoints")
            .with_body_param(create_webhook_endpoint_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get a webhook endpoint
    ///
    /// `GET /api/v1/webhooks/endpoints/{endpoint_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn retrieve(
        &self,
        endpoint_id: impl Into<crate::models::WebhookEndpointId>,
    ) -> crate::api::Call<crate::models::WebhookEndpoint> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/webhooks/endpoints/{endpoint_id}",
        )
        .with_path_param(
            "endpoint_id",
            Into::<crate::models::WebhookEndpointId>::into(endpoint_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Delete a webhook endpoint
    ///
    /// The endpoint is archived and its pending deliveries are cancelled.
    ///
    /// `DELETE /api/v1/webhooks/endpoints/{endpoint_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn delete(
        &self,
        endpoint_id: impl Into<crate::models::WebhookEndpointId>,
    ) -> crate::api::Call<()> {
        crate::request::Request::new(
            http::Method::DELETE,
            "/api/v1/webhooks/endpoints/{endpoint_id}",
        )
        .with_path_param(
            "endpoint_id",
            Into::<crate::models::WebhookEndpointId>::into(endpoint_id),
        )
        .with_options(&self.options)
        .empty(&self.cfg)
    }

    /// Update a webhook endpoint
    ///
    /// Omitted fields are left untouched. Re-enabling a disabled endpoint resets its
    /// consecutive failure count.
    ///
    /// `PATCH /api/v1/webhooks/endpoints/{endpoint_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn update(
        &self,
        endpoint_id: impl Into<crate::models::WebhookEndpointId>,
        update_webhook_endpoint_request: crate::models::UpdateWebhookEndpointRequest,
    ) -> crate::api::Call<crate::models::WebhookEndpoint> {
        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/webhooks/endpoints/{endpoint_id}",
        )
        .with_path_param(
            "endpoint_id",
            Into::<crate::models::WebhookEndpointId>::into(endpoint_id),
        )
        .with_body_param(update_webhook_endpoint_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    fn list_deliveries_request(
        &self,
        endpoint_id: impl Into<crate::models::WebhookEndpointId>,
        options: impl Into<Option<WebhookEndpointsEndpointsListDeliveriesOptions>>,
    ) -> crate::api::Call<crate::models::WebhookDeliveryListResponse> {
        let WebhookEndpointsEndpointsListDeliveriesOptions {
            status,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/webhooks/endpoints/{endpoint_id}/deliveries",
        )
        .with_path_param(
            "endpoint_id",
            Into::<crate::models::WebhookEndpointId>::into(endpoint_id),
        )
        .with_optional_query_param("status", status)
        .with_optional_query_param("page", page)
        .with_optional_query_param("per_page", per_page)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// List deliveries for a webhook endpoint
    ///
    /// `GET /api/v1/webhooks/endpoints/{endpoint_id}/deliveries`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`WebhookDelivery`](crate::models::WebhookDelivery) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn list_deliveries(
        &self,
        endpoint_id: impl Into<crate::models::WebhookEndpointId>,
        options: impl Into<Option<WebhookEndpointsEndpointsListDeliveriesOptions>>,
    ) -> crate::api::PageCall<
        crate::models::WebhookDeliveryListResponse,
        crate::models::WebhookDelivery,
    > {
        static WALK: crate::api::pagination::Walk<WebhookEndpointsEndpointsListDeliveriesOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let endpoint_id: crate::models::WebhookEndpointId = endpoint_id.into();
        let this = self.clone();
        let call = move |options: WebhookEndpointsEndpointsListDeliveriesOptions| {
            this.list_deliveries_request(&endpoint_id, options)
        };
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::WEBHOOK_ENDPOINTS_ENDPOINTS_LIST_DELIVERIES,
            call,
        )
    }

    /// Rotate a webhook endpoint secret
    ///
    /// The previous secret keeps signing alongside the new one for 24 hours, so consumers
    /// can roll over without dropping events.
    ///
    /// `POST /api/v1/webhooks/endpoints/{endpoint_id}/rotate-secret`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn rotate_secret(
        &self,
        endpoint_id: impl Into<crate::models::WebhookEndpointId>,
    ) -> crate::api::Call<crate::models::WebhookEndpointSecret> {
        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/webhooks/endpoints/{endpoint_id}/rotate-secret",
        )
        .with_path_param(
            "endpoint_id",
            Into::<crate::models::WebhookEndpointId>::into(endpoint_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Reveal a webhook endpoint secret
    ///
    /// `GET /api/v1/webhooks/endpoints/{endpoint_id}/secret`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn retrieve_secret(
        &self,
        endpoint_id: impl Into<crate::models::WebhookEndpointId>,
    ) -> crate::api::Call<crate::models::WebhookEndpointSecret> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/webhooks/endpoints/{endpoint_id}/secret",
        )
        .with_path_param(
            "endpoint_id",
            Into::<crate::models::WebhookEndpointId>::into(endpoint_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
