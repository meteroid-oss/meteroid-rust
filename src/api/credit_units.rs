// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`CreditUnits::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreditUnitsListOptions {
    /// Include archived units. Defaults to false.
    pub include_archived: Option<bool>,
}

impl CreditUnitsListOptions {
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

/// The credit units API, from the client's
/// [`credit_units`](crate::api::Meteroid::credit_units).
#[derive(Clone)]
pub struct CreditUnits {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for CreditUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreditUnits")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl CreditUnits {
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

    /// The prices API, with the options of this one.
    #[must_use]
    pub fn prices(&self) -> super::CreditUnitsPrices {
        super::CreditUnitsPrices::new(self.cfg.clone()).with_options(self.options.clone())
    }

    /// List credit units
    ///
    /// The tenant's credit units (e.g. "Halyard credits"): denominations usage can be priced and
    /// credit granted in, instead of money, each with its selling prices.
    ///
    /// `GET /api/v1/credit-units`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 429, 500).
    pub fn list(
        &self,
        options: impl Into<Option<CreditUnitsListOptions>>,
    ) -> crate::api::Call<crate::models::CreditUnitListResponse> {
        let CreditUnitsListOptions { include_archived } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/credit-units")
            .with_optional_query_param("include_archived", include_archived)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Create a credit unit
    ///
    /// `code` and `precision` are immutable once created. Price usage in the unit with a plan fee's
    /// `credit_unit`, grant it with a credit pack or a credit grant, and sell it with a top-up once
    /// it has a `product_id` and a selling price.
    ///
    /// `POST /api/v1/credit-units`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 409, 429, 500).
    pub fn create(
        &self,
        credit_unit_create_request: crate::models::CreditUnitCreateRequest,
    ) -> crate::api::Call<crate::models::CreditUnit> {
        crate::request::Request::new(http::Method::POST, "/api/v1/credit-units")
            .with_body_param(credit_unit_create_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get a credit unit
    ///
    /// The unit with its active selling prices.
    ///
    /// `GET /api/v1/credit-units/{id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn retrieve(&self, id: &str) -> crate::api::Call<crate::models::CreditUnit> {
        crate::request::Request::new(http::Method::GET, "/api/v1/credit-units/{id}")
            .with_path_param("id", id)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Update a credit unit
    ///
    /// Partial update: an absent field is left unchanged. `code` and `precision` are immutable and
    /// cannot be changed here. Setting `archived: true` blocks the unit from new prices, packs and
    /// grants; existing ones keep working.
    ///
    /// `PATCH /api/v1/credit-units/{id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 409, 429, 500).
    pub fn update(
        &self,
        id: &str,
        credit_unit_update_request: crate::models::CreditUnitUpdateRequest,
    ) -> crate::api::Call<crate::models::CreditUnit> {
        crate::request::Request::new(http::Method::PATCH, "/api/v1/credit-units/{id}")
            .with_path_param("id", id)
            .with_body_param(credit_unit_update_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }
}
