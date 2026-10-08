// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// The products entitlements API, from
/// [`Products::entitlements`](crate::api::Products::entitlements).
#[derive(Clone)]
pub struct ProductsEntitlements {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for ProductsEntitlements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProductsEntitlements")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl ProductsEntitlements {
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

    /// List product entitlements
    ///
    /// `GET /api/v1/products/{product_id}/entitlements`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn list(
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
    pub fn create(
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
}
