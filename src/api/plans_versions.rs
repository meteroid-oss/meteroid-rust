// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`PlansVersions::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct PlansVersionsListOptions {
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl PlansVersionsListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            page: None,
            per_page: None,
        }
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

/// The plans versions API, from
/// [`Plans::versions`](crate::api::Plans::versions).
#[derive(Clone)]
pub struct PlansVersions {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for PlansVersions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlansVersions")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl PlansVersions {
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

    /// Set or replace the plan-level minimum commitment for a draft plan version.
    ///
    /// `PUT /api/v1/plans/versions/{plan_version_id}/minimum`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn update_minimum(
        &self,
        plan_version_id: impl Into<crate::models::PlanVersionId>,
        minimum_commitment: crate::models::MinimumCommitment,
    ) -> crate::api::Call<crate::models::MinimumCommitment> {
        crate::request::Request::new(
            http::Method::PUT,
            "/api/v1/plans/versions/{plan_version_id}/minimum",
        )
        .with_path_param(
            "plan_version_id",
            Into::<crate::models::PlanVersionId>::into(plan_version_id),
        )
        .with_body_param(minimum_commitment)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Remove the plan-level minimum commitment for a draft plan version.
    ///
    /// `DELETE /api/v1/plans/versions/{plan_version_id}/minimum`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn delete_minimum(
        &self,
        plan_version_id: impl Into<crate::models::PlanVersionId>,
    ) -> crate::api::Call<()> {
        crate::request::Request::new(
            http::Method::DELETE,
            "/api/v1/plans/versions/{plan_version_id}/minimum",
        )
        .with_path_param(
            "plan_version_id",
            Into::<crate::models::PlanVersionId>::into(plan_version_id),
        )
        .with_options(&self.options)
        .empty(&self.cfg)
    }

    fn list_request(
        &self,
        plan_id: impl Into<crate::models::PlanId>,
        options: impl Into<Option<PlansVersionsListOptions>>,
    ) -> crate::api::Call<crate::models::PlanVersionListResponse> {
        let PlansVersionsListOptions { page, per_page } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/plans/{plan_id}/versions")
            .with_path_param("plan_id", Into::<crate::models::PlanId>::into(plan_id))
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// List plan versions
    ///
    /// `GET /api/v1/plans/{plan_id}/versions`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`PlanVersionSummary`](crate::models::PlanVersionSummary) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn list(
        &self,
        plan_id: impl Into<crate::models::PlanId>,
        options: impl Into<Option<PlansVersionsListOptions>>,
    ) -> crate::api::PageCall<
        crate::models::PlanVersionListResponse,
        crate::models::PlanVersionSummary,
    > {
        static WALK: crate::api::pagination::Walk<PlansVersionsListOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let plan_id: crate::models::PlanId = plan_id.into();
        let this = self.clone();
        let call = move |options: PlansVersionsListOptions| this.list_request(&plan_id, options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::PLANS_VERSIONS_LIST,
            call,
        )
    }
}
