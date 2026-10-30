// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`Licenses::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct LicensesListOptions {
    /// Filter by customer ID or alias
    pub customer_id: Option<String>,
    /// The `subscription_id` query parameter.
    pub subscription_id: Option<String>,
    /// Include revoked licenses. Default: false.
    pub include_revoked: Option<bool>,
}

impl LicensesListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            customer_id: None,
            subscription_id: None,
            include_revoked: None,
        }
    }

    /// Sets the `customer_id` query parameter.
    #[must_use]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    /// Sets the `customer_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_customer_id(mut self, customer_id: Option<String>) -> Self {
        self.customer_id = customer_id;
        self
    }

    /// Sets the `subscription_id` query parameter.
    #[must_use]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    /// Sets the `subscription_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_subscription_id(mut self, subscription_id: Option<String>) -> Self {
        self.subscription_id = subscription_id;
        self
    }

    /// Sets the `include_revoked` query parameter.
    #[must_use]
    pub fn include_revoked(mut self, include_revoked: impl Into<bool>) -> Self {
        self.include_revoked = Some(include_revoked.into());
        self
    }

    /// Sets the `include_revoked` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_include_revoked(mut self, include_revoked: Option<bool>) -> Self {
        self.include_revoked = include_revoked;
        self
    }
}

/// The licenses API, from the client's
/// [`licenses`](crate::api::Meteroid::licenses).
#[derive(Clone)]
pub struct Licenses {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Licenses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Licenses")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Licenses {
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

    /// List licenses
    ///
    /// `GET /api/v1/licenses`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn list(
        &self,
        options: impl Into<Option<LicensesListOptions>>,
    ) -> crate::api::Call<crate::models::LicenseListResponse> {
        let LicensesListOptions {
            customer_id,
            subscription_id,
            include_revoked,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/licenses")
            .with_optional_query_param("customer_id", customer_id)
            .with_optional_query_param("subscription_id", subscription_id)
            .with_optional_query_param("include_revoked", include_revoked)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Issue a license
    ///
    /// Issues a license to a customer, optionally tied to one of its subscriptions. Online, the
    /// license is pending until their installation activates it with the returned one-time code.
    /// Offline, it is returned signed at once, for an installation that can't reach the server.
    ///
    /// `POST /api/v1/licenses`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 403, 404, 409, 429).
    pub fn create(
        &self,
        issue_license_request: crate::models::IssueLicenseRequest,
    ) -> crate::api::Call<crate::models::IssueLicenseResponse> {
        crate::request::Request::new(http::Method::POST, "/api/v1/licenses")
            .with_body_param(issue_license_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get a license
    ///
    /// `GET /api/v1/licenses/{license_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn retrieve(
        &self,
        license_id: impl Into<crate::models::LicenseId>,
    ) -> crate::api::Call<crate::models::License> {
        crate::request::Request::new(http::Method::GET, "/api/v1/licenses/{license_id}")
            .with_path_param(
                "license_id",
                Into::<crate::models::LicenseId>::into(license_id),
            )
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get a license's activation code
    ///
    /// Available until the license is activated or revoked.
    ///
    /// `GET /api/v1/licenses/{license_id}/activation-code`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn retrieve_activation_code(
        &self,
        license_id: impl Into<crate::models::LicenseId>,
    ) -> crate::api::Call<crate::models::LicenseActivationCode> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/licenses/{license_id}/activation-code",
        )
        .with_path_param(
            "license_id",
            Into::<crate::models::LicenseId>::into(license_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Issue an offline license
    ///
    /// Signs a long-lived license for an air-gapped installation from an existing license, e.g. to
    /// renew an offline license before it expires. Given an instance id, binds the license to it.
    ///
    /// `POST /api/v1/licenses/{license_id}/offline`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 403, 404, 409, 429).
    pub fn offline(
        &self,
        license_id: impl Into<crate::models::LicenseId>,
        issue_offline_license_request: crate::models::IssueOfflineLicenseRequest,
    ) -> crate::api::Call<crate::models::OfflineLicense> {
        crate::request::Request::new(http::Method::POST, "/api/v1/licenses/{license_id}/offline")
            .with_path_param(
                "license_id",
                Into::<crate::models::LicenseId>::into(license_id),
            )
            .with_body_param(issue_offline_license_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Revoke a license
    ///
    /// `POST /api/v1/licenses/{license_id}/revoke`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn revoke(
        &self,
        license_id: impl Into<crate::models::LicenseId>,
        revoke_license_request: crate::models::RevokeLicenseRequest,
    ) -> crate::api::Call<crate::models::License> {
        crate::request::Request::new(http::Method::POST, "/api/v1/licenses/{license_id}/revoke")
            .with_path_param(
                "license_id",
                Into::<crate::models::LicenseId>::into(license_id),
            )
            .with_body_param(revoke_license_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get the licensing signing keys
    ///
    /// The keys licenses are signed with, and the public endpoints installations talk to.
    ///
    /// `GET /api/v1/licensing/signing-keys`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 403, 429).
    pub fn list_licensing_signing_keys(
        &self,
    ) -> crate::api::Call<crate::models::LicenseSigningKeysResponse> {
        crate::request::Request::new(http::Method::GET, "/api/v1/licensing/signing-keys")
            .with_options(&self.options)
            .json(&self.cfg)
    }
}
