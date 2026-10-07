// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`ProductFamilies::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ProductFamiliesListOptions {
    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `created_at`. Direction: `asc` or `desc`. Default: `created_at.desc`.
    pub order_by: Option<String>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
    /// The `search` query parameter.
    pub search: Option<String>,
}

impl ProductFamiliesListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            order_by: None,
            page: None,
            per_page: None,
            search: None,
        }
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

    /// Sets the `search` query parameter.
    #[must_use]
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }

    /// Sets the `search` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_search(mut self, search: Option<String>) -> Self {
        self.search = search;
        self
    }
}

/// The product families API, from the client's
/// [`product_families`](crate::api::Meteroid::product_families).
#[derive(Clone)]
pub struct ProductFamilies {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for ProductFamilies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProductFamilies")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl ProductFamilies {
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
        options: impl Into<Option<ProductFamiliesListOptions>>,
    ) -> crate::api::Call<crate::models::ProductFamilyListResponse> {
        let ProductFamiliesListOptions {
            order_by,
            page,
            per_page,
            search,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/product_families")
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_optional_query_param("search", search)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// List product families
    ///
    /// `GET /api/v1/product_families`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`ProductFamily`](crate::models::ProductFamily) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (429).
    pub fn list(
        &self,
        options: impl Into<Option<ProductFamiliesListOptions>>,
    ) -> crate::api::PageCall<crate::models::ProductFamilyListResponse, crate::models::ProductFamily>
    {
        static WALK: crate::api::pagination::Walk<ProductFamiliesListOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let this = self.clone();
        let call = move |options: ProductFamiliesListOptions| this.list_request(options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::PRODUCT_FAMILIES_LIST,
            call,
        )
    }

    /// Create product family
    ///
    /// `POST /api/v1/product_families`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (429).
    pub fn create(
        &self,
        product_family_create_request: crate::models::ProductFamilyCreateRequest,
    ) -> crate::api::Call<crate::models::ProductFamily> {
        crate::request::Request::new(http::Method::POST, "/api/v1/product_families")
            .with_body_param(product_family_create_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get product family
    ///
    /// Retrieve a single product family by ID or alias.
    ///
    /// `GET /api/v1/product_families/{id_or_alias}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (429).
    pub fn retrieve(&self, id_or_alias: &str) -> crate::api::Call<crate::models::ProductFamily> {
        crate::request::Request::new(http::Method::GET, "/api/v1/product_families/{id_or_alias}")
            .with_path_param("id_or_alias", id_or_alias)
            .with_options(&self.options)
            .json(&self.cfg)
    }
}
