// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`UsageCustomer::retrieve`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UsageCustomerRetrieveOptions {
    /// The `start_date` query parameter.
    pub start_date: chrono::NaiveDate,
    /// The `end_date` query parameter.
    pub end_date: chrono::NaiveDate,
    /// The `metric_id` query parameter.
    pub metric_id: Option<BillableMetricId>,
}

impl UsageCustomerRetrieveOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(
        start_date: impl Into<chrono::NaiveDate>,
        end_date: impl Into<chrono::NaiveDate>,
    ) -> Self {
        Self {
            start_date: start_date.into(),
            end_date: end_date.into(),
            metric_id: None,
        }
    }

    /// Sets the `metric_id` query parameter.
    #[must_use]
    pub fn metric_id(mut self, metric_id: impl Into<BillableMetricId>) -> Self {
        self.metric_id = Some(metric_id.into());
        self
    }

    /// Sets the `metric_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_metric_id(mut self, metric_id: Option<BillableMetricId>) -> Self {
        self.metric_id = metric_id;
        self
    }
}

/// Query and header parameters of [`UsageCustomer::retrieve_unattributed`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct UsageCustomerRetrieveUnattributedOptions {
    /// First day of the range. Defaults to the first day of the current month (UTC).
    pub start_date: Option<chrono::NaiveDate>,
    /// Day after the range (exclusive). Defaults to the first day of the next month.
    pub end_date: Option<chrono::NaiveDate>,
}

impl UsageCustomerRetrieveUnattributedOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            start_date: None,
            end_date: None,
        }
    }

    /// Sets the `start_date` query parameter.
    #[must_use]
    pub fn start_date(mut self, start_date: impl Into<chrono::NaiveDate>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    /// Sets the `start_date` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_start_date(mut self, start_date: Option<chrono::NaiveDate>) -> Self {
        self.start_date = start_date;
        self
    }

    /// Sets the `end_date` query parameter.
    #[must_use]
    pub fn end_date(mut self, end_date: impl Into<chrono::NaiveDate>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    /// Sets the `end_date` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_end_date(mut self, end_date: Option<chrono::NaiveDate>) -> Self {
        self.end_date = end_date;
        self
    }
}

/// The usage customer API, from
/// [`Usage::customer`](crate::api::Usage::customer).
#[derive(Clone)]
pub struct UsageCustomer {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for UsageCustomer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UsageCustomer")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl UsageCustomer {
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

    /// Get customer usage
    ///
    /// Retrieve aggregated usage data for a customer over a specified period.
    ///
    /// `GET /api/v1/usage/customer/{customer_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn retrieve(
        &self,
        customer_id: &str,
        options: UsageCustomerRetrieveOptions,
    ) -> crate::api::Call<crate::models::UsageResponse> {
        let UsageCustomerRetrieveOptions {
            start_date,
            end_date,
            metric_id,
        } = options;

        crate::request::Request::new(http::Method::GET, "/api/v1/usage/customer/{customer_id}")
            .with_path_param("customer_id", customer_id)
            .with_query_param("start_date", &start_date)
            .with_query_param("end_date", &end_date)
            .with_optional_query_param("metric_id", metric_id)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get unattributed usage
    ///
    /// Retrieve a customer's usage that no subscription bills. When subscriptions of the customer
    /// pricing a metric set a usage filter, events matching none of the filters are billed by a
    /// subscription without one; when every such subscription has a filter, those events are kept
    /// and reported here instead of being billed.
    ///
    /// `GET /api/v1/usage/customer/{customer_id}/unattributed`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn retrieve_unattributed(
        &self,
        customer_id: &str,
        options: impl Into<Option<UsageCustomerRetrieveUnattributedOptions>>,
    ) -> crate::api::Call<crate::models::UnattributedUsageResponse> {
        let UsageCustomerRetrieveUnattributedOptions {
            start_date,
            end_date,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/usage/customer/{customer_id}/unattributed",
        )
        .with_path_param("customer_id", customer_id)
        .with_optional_query_param("start_date", start_date)
        .with_optional_query_param("end_date", end_date)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
