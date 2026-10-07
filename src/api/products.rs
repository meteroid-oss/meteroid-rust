// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`Products::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ProductsListOptions {
    /// The `product_family_id` query parameter.
    pub product_family_id: Option<ProductFamilyId>,
    /// The `search` query parameter.
    pub search: Option<String>,
    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `created_at`. Direction: `asc` or `desc`. Default: `name.asc`.
    pub order_by: Option<String>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl ProductsListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            product_family_id: None,
            search: None,
            order_by: None,
            page: None,
            per_page: None,
        }
    }

    /// Sets the `product_family_id` query parameter.
    #[must_use]
    pub fn product_family_id(mut self, product_family_id: impl Into<ProductFamilyId>) -> Self {
        self.product_family_id = Some(product_family_id.into());
        self
    }

    /// Sets the `product_family_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_product_family_id(mut self, product_family_id: Option<ProductFamilyId>) -> Self {
        self.product_family_id = product_family_id;
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

/// The products API, from the client's
/// [`products`](crate::api::Meteroid::products).
#[derive(Clone)]
pub struct Products {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Products {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Products")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Products {
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
        options: impl Into<Option<ProductsListOptions>>,
    ) -> crate::api::Call<crate::models::ProductListResponse> {
        let ProductsListOptions {
            product_family_id,
            search,
            order_by,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/products")
            .with_optional_query_param("product_family_id", product_family_id)
            .with_optional_query_param("search", search)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// List products
    ///
    /// `GET /api/v1/products`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`Product`](crate::models::Product) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 429).
    pub fn list(
        &self,
        options: impl Into<Option<ProductsListOptions>>,
    ) -> crate::api::PageCall<crate::models::ProductListResponse, crate::models::Product> {
        static WALK: crate::api::pagination::Walk<ProductsListOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let this = self.clone();
        let call = move |options: ProductsListOptions| this.list_request(options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::PRODUCTS_LIST,
            call,
        )
    }

    /// Create a product
    ///
    /// `POST /api/v1/products`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 429).
    pub fn create(
        &self,
        create_product_request: crate::models::CreateProductRequest,
    ) -> crate::api::Call<crate::models::Product> {
        crate::request::Request::new(http::Method::POST, "/api/v1/products")
            .with_body_param(create_product_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get product details
    ///
    /// `GET /api/v1/products/{product_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn retrieve(
        &self,
        product_id: impl Into<crate::models::ProductId>,
    ) -> crate::api::Call<crate::models::Product> {
        crate::request::Request::new(http::Method::GET, "/api/v1/products/{product_id}")
            .with_path_param(
                "product_id",
                Into::<crate::models::ProductId>::into(product_id),
            )
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Update a product
    ///
    /// Partially update product fields. The fee_type is immutable and cannot be changed.
    ///
    /// `PATCH /api/v1/products/{product_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn update(
        &self,
        product_id: impl Into<crate::models::ProductId>,
        update_product_request: crate::models::UpdateProductRequest,
    ) -> crate::api::Call<crate::models::Product> {
        crate::request::Request::new(http::Method::PATCH, "/api/v1/products/{product_id}")
            .with_path_param(
                "product_id",
                Into::<crate::models::ProductId>::into(product_id),
            )
            .with_body_param(update_product_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Archive a product
    ///
    /// `POST /api/v1/products/{product_id}/archive`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn archive(&self, product_id: impl Into<crate::models::ProductId>) -> crate::api::Call<()> {
        crate::request::Request::new(http::Method::POST, "/api/v1/products/{product_id}/archive")
            .with_path_param(
                "product_id",
                Into::<crate::models::ProductId>::into(product_id),
            )
            .with_options(&self.options)
            .empty(&self.cfg)
    }

    /// List product entitlements
    ///
    /// `GET /api/v1/products/{product_id}/entitlements`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn list_entitlements(
        &self,
        product_id: impl Into<crate::models::ProductId>,
    ) -> crate::api::Call<crate::models::ResolvedEntitlementListResponse> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/products/{product_id}/entitlements",
        )
        .with_path_param(
            "product_id",
            Into::<crate::models::ProductId>::into(product_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Create product entitlements
    ///
    /// A product has no entitlement rows of its own: its entitlements are the feature-level
    /// defaults of the features scoped to it, which is what `GET` on this path resolves. Every
    /// spec must therefore target a feature belonging to `product_id`. Features that already
    /// carry a default entitlement are skipped.
    ///
    /// Specs are validated up front, but the writes are not atomic: each feature is written on
    /// its own, so a failure part-way can leave earlier specs committed. Retrying is safe.
    ///
    /// `POST /api/v1/products/{product_id}/entitlements`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn create_entitlement(
        &self,
        product_id: impl Into<crate::models::ProductId>,
        create_entitlements_request: crate::models::CreateEntitlementsRequest,
    ) -> crate::api::Call<crate::models::EntitlementListResponse> {
        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/products/{product_id}/entitlements",
        )
        .with_path_param(
            "product_id",
            Into::<crate::models::ProductId>::into(product_id),
        )
        .with_body_param(create_entitlements_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Unarchive a product
    ///
    /// `POST /api/v1/products/{product_id}/unarchive`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn unarchive(
        &self,
        product_id: impl Into<crate::models::ProductId>,
    ) -> crate::api::Call<()> {
        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/products/{product_id}/unarchive",
        )
        .with_path_param(
            "product_id",
            Into::<crate::models::ProductId>::into(product_id),
        )
        .with_options(&self.options)
        .empty(&self.cfg)
    }
}
