// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// The add ons entitlements API, from
/// [`AddOns::entitlements`](crate::api::AddOns::entitlements).
#[derive(Clone)]
pub struct AddOnsEntitlements {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for AddOnsEntitlements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddOnsEntitlements")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl AddOnsEntitlements {
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

    /// List add-on entitlements
    ///
    /// `GET /api/v1/addons/{addon_id}/entitlements`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn list(
        &self,
        addon_id: impl Into<crate::models::AddOnId>,
    ) -> crate::api::Call<crate::models::ResolvedEntitlementListResponse> {
        crate::request::Request::new(http::Method::GET, "/api/v1/addons/{addon_id}/entitlements")
            .with_path_param("addon_id", Into::<crate::models::AddOnId>::into(addon_id))
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Create add-on entitlements
    ///
    /// Entitlements already present on this add-on are skipped.
    ///
    /// `POST /api/v1/addons/{addon_id}/entitlements`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn create(
        &self,
        addon_id: impl Into<crate::models::AddOnId>,
        create_entitlements_request: crate::models::CreateEntitlementsRequest,
    ) -> crate::api::Call<crate::models::EntitlementListResponse> {
        crate::request::Request::new(http::Method::POST, "/api/v1/addons/{addon_id}/entitlements")
            .with_path_param("addon_id", Into::<crate::models::AddOnId>::into(addon_id))
            .with_body_param(create_entitlements_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }
}
