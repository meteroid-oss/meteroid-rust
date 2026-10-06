// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]
#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// The entitlements API, from the client's
/// [`entitlements`](crate::api::Meteroid::entitlements).
#[derive(Clone)]
pub struct Entitlements {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Entitlements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entitlements")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Entitlements {
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

    /// Get entitlement details
    ///
    /// `GET /api/v1/entitlements/{entitlement_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn retrieve(&self, entitlement_id: &str) -> crate::api::Call<crate::models::Entitlement> {
        crate::request::Request::new(http::Method::GET, "/api/v1/entitlements/{entitlement_id}")
            .with_path_param("entitlement_id", entitlement_id)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Delete an entitlement
    ///
    /// `DELETE /api/v1/entitlements/{entitlement_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn delete(&self, entitlement_id: &str) -> crate::api::Call<()> {
        crate::request::Request::new(
            http::Method::DELETE,
            "/api/v1/entitlements/{entitlement_id}",
        )
        .with_path_param("entitlement_id", entitlement_id)
        .with_options(&self.options)
        .empty(&self.cfg)
    }

    /// Update an entitlement
    ///
    /// The new value must match the feature's declared type.
    ///
    /// `PATCH /api/v1/entitlements/{entitlement_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn update(
        &self,
        entitlement_id: &str,
        update_entitlement_request: crate::models::UpdateEntitlementRequest,
    ) -> crate::api::Call<crate::models::Entitlement> {
        crate::request::Request::new(http::Method::PATCH, "/api/v1/entitlements/{entitlement_id}")
            .with_path_param("entitlement_id", entitlement_id)
            .with_body_param(update_entitlement_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }
}
