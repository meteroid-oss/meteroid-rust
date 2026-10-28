// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`Budgets::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BudgetsListOptions {
    /// Defaults to false.
    pub include_archived: Option<bool>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl BudgetsListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            include_archived: None,
            page: None,
            per_page: None,
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

/// Query and header parameters of [`Budgets::create`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct BudgetsCreateOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl BudgetsCreateOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// Query and header parameters of [`Budgets::update`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct BudgetsUpdateOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl BudgetsUpdateOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// Query and header parameters of [`Budgets::archive`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct BudgetsArchiveOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl BudgetsArchiveOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// The budgets API, from the client's
/// [`budgets`](crate::api::Meteroid::budgets).
#[derive(Clone)]
pub struct Budgets {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Budgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Budgets")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Budgets {
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
        id_or_alias: &str,
        options: impl Into<Option<BudgetsListOptions>>,
    ) -> crate::api::Call<crate::models::BudgetListResponse> {
        let BudgetsListOptions {
            include_archived,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/customers/{id_or_alias}/budgets")
            .with_path_param("id_or_alias", id_or_alias)
            .with_optional_query_param("include_archived", include_archived)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// List budgets
    ///
    /// A customer's budgets, oldest first, with their estimated spend in the current billing period.
    /// Archived ones are left out unless requested.
    ///
    /// Beta: the request and response shapes of this endpoint may change before general availability.
    ///
    /// `GET /api/v1/customers/{id_or_alias}/budgets`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`Budget`](crate::models::Budget) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn list(
        &self,
        id_or_alias: &str,
        options: impl Into<Option<BudgetsListOptions>>,
    ) -> crate::api::PageCall<crate::models::BudgetListResponse, crate::models::Budget> {
        static WALK: crate::api::pagination::Walk<BudgetsListOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let id_or_alias = id_or_alias.to_string();
        let this = self.clone();
        let call = move |options: BudgetsListOptions| this.list_request(&id_or_alias, options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::BUDGETS_LIST,
            call,
        )
    }

    /// Create a budget
    ///
    /// Sets a spending limit for one currency the customer is billed in, with alerts at percentages
    /// of it, on the customer's whole spend or on one subscription's. A customer has one active
    /// budget per currency, and per subscription.
    ///
    /// Beta: the request and response shapes of this endpoint may change before general availability.
    ///
    /// `POST /api/v1/customers/{id_or_alias}/budgets`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 409, 429, 500).
    pub fn create(
        &self,
        id_or_alias: &str,
        create_budget_request: crate::models::CreateBudgetRequest,
        options: BudgetsCreateOptions,
    ) -> crate::api::Call<crate::models::Budget> {
        let BudgetsCreateOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/customers/{id_or_alias}/budgets",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_body_param(create_budget_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Update a budget
    ///
    /// Changes the amount, thresholds or behavior. Send the `version` you read as `expected_version`; a newer
    /// stored version, for instance after the customer edited it, is refused with 409.
    ///
    /// Beta: the request and response shapes of this endpoint may change before general availability.
    ///
    /// `PATCH /api/v1/customers/{id_or_alias}/budgets/{budget_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 409, 429, 500).
    pub fn update(
        &self,
        id_or_alias: &str,
        budget_id: &str,
        patch_budget_request: crate::models::PatchBudgetRequest,
        options: BudgetsUpdateOptions,
    ) -> crate::api::Call<crate::models::Budget> {
        let BudgetsUpdateOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/customers/{id_or_alias}/budgets/{budget_id}",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_path_param("budget_id", budget_id)
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_body_param(patch_budget_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Archive a budget
    ///
    /// Stops the budget from sending alerts and, for a HARD budget, from refusing access. Archiving an
    /// archived budget returns it unchanged.
    ///
    /// Beta: the request and response shapes of this endpoint may change before general availability.
    ///
    /// `POST /api/v1/customers/{id_or_alias}/budgets/{budget_id}/archive`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 409, 429, 500).
    pub fn archive(
        &self,
        id_or_alias: &str,
        budget_id: &str,
        options: BudgetsArchiveOptions,
    ) -> crate::api::Call<crate::models::Budget> {
        let BudgetsArchiveOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/customers/{id_or_alias}/budgets/{budget_id}/archive",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_path_param("budget_id", budget_id)
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
