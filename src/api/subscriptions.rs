// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`Subscriptions::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SubscriptionsListOptions {
    /// Filter by customer ID or alias
    pub customer_id: Option<String>,
    /// The `plan_id` query parameter.
    pub plan_id: Option<PlanId>,
    /// The `statuses` query parameter.
    pub statuses: Option<Vec<SubscriptionStatusEnum>>,
    /// Sort order. Format: `column.direction`. Allowed columns: `customer_name`, `plan_name`, `mrr_cents`, `billing_start_date`, `end_date`, `status`, `created_at`. Direction: `asc` or `desc`. Default: `created_at.desc`.
    pub order_by: Option<String>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl SubscriptionsListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            customer_id: None,
            plan_id: None,
            statuses: None,
            order_by: None,
            page: None,
            per_page: None,
        }
    }

    /// Sets the `customer_id` query parameter.
    #[must_use]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    /// Sets the `customer_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_customer_id(mut self, customer_id: Option<String>) -> Self {
        self.customer_id = customer_id;
        self
    }

    /// Sets the `plan_id` query parameter.
    #[must_use]
    pub fn plan_id(mut self, plan_id: impl Into<PlanId>) -> Self {
        self.plan_id = Some(plan_id.into());
        self
    }

    /// Sets the `plan_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_plan_id(mut self, plan_id: Option<PlanId>) -> Self {
        self.plan_id = plan_id;
        self
    }

    /// Sets the `statuses` query parameter.
    #[must_use]
    pub fn statuses(mut self, statuses: impl Into<Vec<SubscriptionStatusEnum>>) -> Self {
        self.statuses = Some(statuses.into());
        self
    }

    /// Sets the `statuses` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_statuses(mut self, statuses: Option<Vec<SubscriptionStatusEnum>>) -> Self {
        self.statuses = statuses;
        self
    }

    /// Sets the `order_by` query parameter.
    #[must_use]
    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    /// Sets the `order_by` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_order_by(mut self, order_by: Option<String>) -> Self {
        self.order_by = order_by;
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

/// The subscriptions API, from the client's
/// [`subscriptions`](crate::api::Meteroid::subscriptions).
#[derive(Clone)]
pub struct Subscriptions {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Subscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscriptions")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Subscriptions {
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

    fn list_request(
        &self,
        options: impl Into<Option<SubscriptionsListOptions>>,
    ) -> crate::api::Call<crate::models::SubscriptionListResponse> {
        let SubscriptionsListOptions {
            customer_id,
            plan_id,
            statuses,
            order_by,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/subscriptions")
            .with_optional_query_param("customer_id", customer_id)
            .with_optional_query_param("plan_id", plan_id)
            .with_optional_exploded_query_param("statuses", statuses)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// List subscriptions with optional filtering by customer or plan.
    ///
    /// `GET /api/v1/subscriptions`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`Subscription`](crate::models::Subscription) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 429, 500).
    pub fn list(
        &self,
        options: impl Into<Option<SubscriptionsListOptions>>,
    ) -> crate::api::PageCall<crate::models::SubscriptionListResponse, crate::models::Subscription>
    {
        static WALK: crate::api::pagination::Walk<SubscriptionsListOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let this = self.clone();
        let call = move |options: SubscriptionsListOptions| this.list_request(options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::SUBSCRIPTIONS_LIST,
            call,
        )
    }

    /// Create subscription
    ///
    /// Create a new subscription for a customer with a specific plan.
    ///
    /// `POST /api/v1/subscriptions`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 429, 500).
    pub fn create(
        &self,
        subscription_create_request: crate::models::SubscriptionCreateRequest,
    ) -> crate::api::Call<crate::models::SubscriptionDetails> {
        crate::request::Request::new(http::Method::POST, "/api/v1/subscriptions")
            .with_body_param(subscription_create_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get subscription details
    ///
    /// Retrieve detailed information about a subscription including price components and schedules.
    ///
    /// `GET /api/v1/subscriptions/{subscription_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn retrieve(
        &self,
        subscription_id: impl Into<crate::models::SubscriptionId>,
    ) -> crate::api::Call<crate::models::SubscriptionDetails> {
        crate::request::Request::new(http::Method::GET, "/api/v1/subscriptions/{subscription_id}")
            .with_path_param(
                "subscription_id",
                Into::<crate::models::SubscriptionId>::into(subscription_id),
            )
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Update subscription settings like payment configuration, billing options, etc.
    ///
    /// `PATCH /api/v1/subscriptions/{subscription_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn update(
        &self,
        subscription_id: impl Into<crate::models::SubscriptionId>,
        subscription_update_request: crate::models::SubscriptionUpdateRequest,
    ) -> crate::api::Call<crate::models::SubscriptionUpdateResponse> {
        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/subscriptions/{subscription_id}",
        )
        .with_path_param(
            "subscription_id",
            Into::<crate::models::SubscriptionId>::into(subscription_id),
        )
        .with_body_param(subscription_update_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Cancel subscription
    ///
    /// Cancel a subscription either immediately or at the end of the billing period.
    ///
    /// `POST /api/v1/subscriptions/{subscription_id}/cancel`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn cancel(
        &self,
        subscription_id: impl Into<crate::models::SubscriptionId>,
        cancel_subscription_request: crate::models::CancelSubscriptionRequest,
    ) -> crate::api::Call<crate::models::CancelSubscriptionResponse> {
        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/subscriptions/{subscription_id}/cancel",
        )
        .with_path_param(
            "subscription_id",
            Into::<crate::models::SubscriptionId>::into(subscription_id),
        )
        .with_body_param(cancel_subscription_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// List subscription entitlements
    ///
    /// `GET /api/v1/subscriptions/{subscription_id}/entitlements`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn list_entitlements(
        &self,
        subscription_id: impl Into<crate::models::SubscriptionId>,
    ) -> crate::api::Call<crate::models::EffectiveEntitlementListResponse> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/subscriptions/{subscription_id}/entitlements",
        )
        .with_path_param(
            "subscription_id",
            Into::<crate::models::SubscriptionId>::into(subscription_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Get subscription summary
    ///
    /// Retrieve a subscription without its components, add-ons, coupons and entitlements: the same
    /// shape as list items, for callers that only need status and billing dates.
    ///
    /// `GET /api/v1/subscriptions/{subscription_id}/summary`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn retrieve_summary(
        &self,
        subscription_id: impl Into<crate::models::SubscriptionId>,
    ) -> crate::api::Call<crate::models::Subscription> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/subscriptions/{subscription_id}/summary",
        )
        .with_path_param(
            "subscription_id",
            Into::<crate::models::SubscriptionId>::into(subscription_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
