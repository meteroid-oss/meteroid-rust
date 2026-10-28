// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`CreditUnitsPrices::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreditUnitsPricesListOptions {
    /// Include archived prices. Defaults to false.
    pub include_archived: Option<bool>,
}

impl CreditUnitsPricesListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            include_archived: None,
        }
    }

    /// Sets the `include_archived` query parameter.
    #[must_use]
    pub fn include_archived(mut self, include_archived: impl Into<bool>) -> Self {
        self.include_archived = Some(include_archived.into());
        self
    }

    /// Sets the `include_archived` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_include_archived(mut self, include_archived: Option<bool>) -> Self {
        self.include_archived = include_archived;
        self
    }
}

/// The credit units prices API, from
/// [`CreditUnits::prices`](crate::api::CreditUnits::prices).
#[derive(Clone)]
pub struct CreditUnitsPrices {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for CreditUnitsPrices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreditUnitsPrices")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl CreditUnitsPrices {
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

    /// List a credit unit's selling prices
    ///
    /// The active price per currency, and archived ones when asked, newest first.
    ///
    /// `GET /api/v1/credit-units/{id}/prices`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn list(
        &self,
        id: &str,
        options: impl Into<Option<CreditUnitsPricesListOptions>>,
    ) -> crate::api::Call<crate::models::CreditUnitPriceListResponse> {
        let CreditUnitsPricesListOptions { include_archived } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/credit-units/{id}/prices")
            .with_path_param("id", id)
            .with_optional_query_param("include_archived", include_archived)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Set a credit unit's selling price
    ///
    /// The price of one credit in a currency, used to sell the unit through top-ups and automatic
    /// top-ups (taxed at purchase under the unit's `product_id`). Replaces the active price in that
    /// currency, which is archived: a top-up already priced keeps its terms.
    ///
    /// `PUT /api/v1/credit-units/{id}/prices/{currency}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 409, 429, 500).
    pub fn update(
        &self,
        id: &str,
        currency: &str,
        credit_unit_price_set_request: crate::models::CreditUnitPriceSetRequest,
    ) -> crate::api::Call<crate::models::CreditUnitPrice> {
        crate::request::Request::new(
            http::Method::PUT,
            "/api/v1/credit-units/{id}/prices/{currency}",
        )
        .with_path_param("id", id)
        .with_path_param("currency", currency)
        .with_body_param(credit_unit_price_set_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Remove a credit unit's selling price
    ///
    /// Archives the active price in a currency: the unit can no longer be sold in it (top-ups and
    /// automatic top-ups included) until a new price is set.
    ///
    /// `DELETE /api/v1/credit-units/{id}/prices/{currency}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn archive(
        &self,
        id: &str,
        currency: &str,
    ) -> crate::api::Call<crate::models::CreditUnitPrice> {
        crate::request::Request::new(
            http::Method::DELETE,
            "/api/v1/credit-units/{id}/prices/{currency}",
        )
        .with_path_param("id", id)
        .with_path_param("currency", currency)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
