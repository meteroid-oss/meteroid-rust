// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`UsageCostsCustomer::list_costs`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UsageCostsCustomerListCostsOptions {
    /// Billable metric, by ID or code.
    pub metric: String,
    /// Event property to split the usage by.
    pub group_by: Option<String>,
    /// The `granularity` query parameter.
    pub granularity: ListCustomerUsageCostsGranularity,
    /// Start of the range, inclusive.
    pub from: chrono::DateTime<chrono::Utc>,
    /// End of the range, exclusive. At most 93 days after `from`.
    pub to: chrono::DateTime<chrono::Utc>,
}

impl UsageCostsCustomerListCostsOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(
        metric: impl Into<String>,
        granularity: impl Into<ListCustomerUsageCostsGranularity>,
        from: impl Into<chrono::DateTime<chrono::Utc>>,
        to: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            metric: metric.into(),
            group_by: None,
            granularity: granularity.into(),
            from: from.into(),
            to: to.into(),
        }
    }

    /// Sets the `group_by` query parameter.
    #[must_use]
    pub fn group_by(mut self, group_by: impl Into<String>) -> Self {
        self.group_by = Some(group_by.into());
        self
    }

    /// Sets the `group_by` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_group_by(mut self, group_by: Option<String>) -> Self {
        self.group_by = group_by;
        self
    }
}

/// Query and header parameters of [`UsageCostsCustomer::list_events`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UsageCostsCustomerListEventsOptions {
    /// Billable metric, by ID or code.
    pub metric: String,
    /// Start of the range, inclusive.
    pub from: chrono::DateTime<chrono::Utc>,
    /// End of the range, exclusive. At most 93 days after `from`.
    pub to: chrono::DateTime<chrono::Utc>,
    /// The `next_cursor` of the previous page.
    pub cursor: Option<String>,
    /// Page size, 100 by default and 500 at most.
    pub limit: Option<i32>,
}

impl UsageCostsCustomerListEventsOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(
        metric: impl Into<String>,
        from: impl Into<chrono::DateTime<chrono::Utc>>,
        to: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            metric: metric.into(),
            from: from.into(),
            to: to.into(),
            cursor: None,
            limit: None,
        }
    }

    /// Sets the `cursor` query parameter.
    #[must_use]
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Sets the `cursor` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_cursor(mut self, cursor: Option<String>) -> Self {
        self.cursor = cursor;
        self
    }

    /// Sets the `limit` query parameter.
    #[must_use]
    pub fn limit(mut self, limit: impl Into<i32>) -> Self {
        self.limit = Some(limit.into());
        self
    }

    /// Sets the `limit` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_limit(mut self, limit: Option<i32>) -> Self {
        self.limit = limit;
        self
    }
}

/// The usage costs customer API, from
/// [`UsageCosts::customer`](crate::api::UsageCosts::customer).
#[derive(Clone)]
pub struct UsageCostsCustomer {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for UsageCostsCustomer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UsageCostsCustomer")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl UsageCostsCustomer {
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

    /// List a customer's usage costs over time
    ///
    /// A customer's usage of one billable metric and what it costs, per hour, day or month, optionally
    /// split by an event property, summed across the subscriptions that bill it. Buckets are cut at UTC
    /// boundaries and only cover the customer's billing periods. Per-unit and matrix pricing give exact
    /// costs. For pricing that depends on the whole period's usage (tiered, volume, package, capacity),
    /// a bucket's cost is what it adds to the period's price so far and `exact` is false. The range is
    /// at most 93 days.
    ///
    /// Beta: the request and response shapes of this endpoint may change before general availability.
    ///
    /// `GET /api/v1/usage/customer/{customer_id}/costs`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn list_costs(
        &self,
        customer_id: &str,
        options: UsageCostsCustomerListCostsOptions,
    ) -> crate::api::Call<crate::models::UsageCostListResponse> {
        let UsageCostsCustomerListCostsOptions {
            metric,
            group_by,
            granularity,
            from,
            to,
        } = options;

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/usage/customer/{customer_id}/costs",
        )
        .with_path_param("customer_id", customer_id)
        .with_query_param("metric", &metric)
        .with_optional_query_param("group_by", group_by)
        .with_query_param("granularity", &granularity)
        .with_query_param("from", &from)
        .with_query_param("to", &to)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// List a customer's usage events with their cost
    ///
    /// A customer's events for one billable metric, oldest first, each with the units it adds and what
    /// they cost at the price in force. The cost is exact for per-unit and matrix pricing and absent
    /// for pricing that depends on the whole period's usage (tiered, volume, package, capacity) and for
    /// aggregations that cannot be split per event. The range is at most 93 days.
    ///
    /// Beta: the request and response shapes of this endpoint may change before general availability.
    ///
    /// `GET /api/v1/usage/customer/{customer_id}/events`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn list_events(
        &self,
        customer_id: &str,
        options: UsageCostsCustomerListEventsOptions,
    ) -> crate::api::Call<crate::models::UsageEventListResponse> {
        let UsageCostsCustomerListEventsOptions {
            metric,
            from,
            to,
            cursor,
            limit,
        } = options;

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/usage/customer/{customer_id}/events",
        )
        .with_path_param("customer_id", customer_id)
        .with_query_param("metric", &metric)
        .with_query_param("from", &from)
        .with_query_param("to", &to)
        .with_optional_query_param("cursor", cursor)
        .with_optional_query_param("limit", limit)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
