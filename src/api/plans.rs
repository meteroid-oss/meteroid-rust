// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`Plans::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct PlansListOptions {
    /// The `product_family_id` query parameter.
    pub product_family_id: Option<ProductFamilyId>,
    /// Search by plan name
    pub search: Option<String>,
    /// Filter by plan status (can be repeated)
    pub status: Option<Vec<PlanStatusEnum>>,
    /// Filter by plan type (can be repeated)
    pub plan_type: Option<Vec<PlanTypeEnum>>,
    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `status`, `plan_type`, `created_at`. Direction: `asc` or `desc`. Default: `created_at.desc`.
    pub order_by: Option<String>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl PlansListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            product_family_id: None,
            search: None,
            status: None,
            plan_type: None,
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

    /// Sets the `status` query parameter.
    #[must_use]
    pub fn status(mut self, status: impl Into<Vec<PlanStatusEnum>>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Sets the `status` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_status(mut self, status: Option<Vec<PlanStatusEnum>>) -> Self {
        self.status = status;
        self
    }

    /// Sets the `plan_type` query parameter.
    #[must_use]
    pub fn plan_type(mut self, plan_type: impl Into<Vec<PlanTypeEnum>>) -> Self {
        self.plan_type = Some(plan_type.into());
        self
    }

    /// Sets the `plan_type` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_plan_type(mut self, plan_type: Option<Vec<PlanTypeEnum>>) -> Self {
        self.plan_type = plan_type;
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

/// Query and header parameters of [`Plans::retrieve`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct PlansRetrieveOptions {
    /// Filter by version: "draft", a version number, or omitted for active
    pub version: Option<String>,
}

impl PlansRetrieveOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self { version: None }
    }

    /// Sets the `version` query parameter.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets the `version` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_version(mut self, version: Option<String>) -> Self {
        self.version = version;
        self
    }
}

/// Query and header parameters of [`Plans::list_versions`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct PlansListVersionsOptions {
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl PlansListVersionsOptions {
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

/// The plans API, from the client's
/// [`plans`](crate::api::Meteroid::plans).
#[derive(Clone)]
pub struct Plans {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Plans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plans")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Plans {
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

    /// List plan version entitlements
    ///
    /// `GET /api/v1/plan-versions/{plan_version_id}/entitlements`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn list_plan_version_entitlements(
        &self,
        plan_version_id: impl Into<crate::models::PlanVersionId>,
    ) -> crate::api::Call<crate::models::ResolvedEntitlementListResponse> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/plan-versions/{plan_version_id}/entitlements",
        )
        .with_path_param(
            "plan_version_id",
            Into::<crate::models::PlanVersionId>::into(plan_version_id),
        )
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Create plan version entitlements
    ///
    /// Entitlements already present on this plan version are skipped.
    ///
    /// `POST /api/v1/plan-versions/{plan_version_id}/entitlements`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn create_plan_version_entitlement(
        &self,
        plan_version_id: impl Into<crate::models::PlanVersionId>,
        create_entitlements_request: crate::models::CreateEntitlementsRequest,
    ) -> crate::api::Call<crate::models::EntitlementListResponse> {
        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/plan-versions/{plan_version_id}/entitlements",
        )
        .with_path_param(
            "plan_version_id",
            Into::<crate::models::PlanVersionId>::into(plan_version_id),
        )
        .with_body_param(create_entitlements_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    fn list_request(
        &self,
        options: impl Into<Option<PlansListOptions>>,
    ) -> crate::api::Call<crate::models::PlanListResponse> {
        let PlansListOptions {
            product_family_id,
            search,
            status,
            plan_type,
            order_by,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/plans")
            .with_optional_query_param("product_family_id", product_family_id)
            .with_optional_query_param("search", search)
            .with_optional_exploded_query_param("status", status)
            .with_optional_exploded_query_param("plan_type", plan_type)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// List plans
    ///
    /// `GET /api/v1/plans`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`Plan`](crate::models::Plan) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 429).
    pub fn list(
        &self,
        options: impl Into<Option<PlansListOptions>>,
    ) -> crate::api::PageCall<crate::models::PlanListResponse, crate::models::Plan> {
        static WALK: crate::api::pagination::Walk<PlansListOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let this = self.clone();
        let call = move |options: PlansListOptions| this.list_request(options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::PLANS_LIST,
            call,
        )
    }

    /// Create a plan
    ///
    /// Create a new plan with components and pricing. Set `status` to `ACTIVE` to
    /// publish immediately, or `DRAFT` to stage for review.
    ///
    /// `POST /api/v1/plans`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 409, 429).
    pub fn create(
        &self,
        create_plan_request: crate::models::CreatePlanRequest,
    ) -> crate::api::Call<crate::models::Plan> {
        crate::request::Request::new(http::Method::POST, "/api/v1/plans")
            .with_body_param(create_plan_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Set or replace the plan-level minimum commitment for a draft plan version.
    ///
    /// `PUT /api/v1/plans/versions/{plan_version_id}/minimum`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn update_version_minimum(
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
    pub fn delete_version_minimum(
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

    /// Get plan details
    ///
    /// Retrieve a specific plan. Use `?version=draft` for the draft version,
    /// `?version=2` for a specific version number, or omit for the active version.
    ///
    /// `GET /api/v1/plans/{plan_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn retrieve(
        &self,
        plan_id: impl Into<crate::models::PlanId>,
        options: impl Into<Option<PlansRetrieveOptions>>,
    ) -> crate::api::Call<crate::models::Plan> {
        let PlansRetrieveOptions { version } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/plans/{plan_id}")
            .with_path_param("plan_id", Into::<crate::models::PlanId>::into(plan_id))
            .with_optional_query_param("version", version)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Replace a plan
    ///
    /// Full replacement of a plan's version. On a draft plan, updates in-place.
    /// On a published plan, creates a new version. Set `status` to `DRAFT` to
    /// stage as a new draft without publishing.
    ///
    /// `PUT /api/v1/plans/{plan_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn replace(
        &self,
        plan_id: impl Into<crate::models::PlanId>,
        replace_plan_request: crate::models::ReplacePlanRequest,
    ) -> crate::api::Call<crate::models::Plan> {
        crate::request::Request::new(http::Method::PUT, "/api/v1/plans/{plan_id}")
            .with_path_param("plan_id", Into::<crate::models::PlanId>::into(plan_id))
            .with_body_param(replace_plan_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Update plan metadata
    ///
    /// Partially update plan-level fields (name, description, self_service_rank).
    /// Does not modify version-level configuration or components.
    ///
    /// `PATCH /api/v1/plans/{plan_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429).
    pub fn update(
        &self,
        plan_id: impl Into<crate::models::PlanId>,
        patch_plan_request: crate::models::PatchPlanRequest,
    ) -> crate::api::Call<crate::models::Plan> {
        crate::request::Request::new(http::Method::PATCH, "/api/v1/plans/{plan_id}")
            .with_path_param("plan_id", Into::<crate::models::PlanId>::into(plan_id))
            .with_body_param(patch_plan_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Archive a plan
    ///
    /// `POST /api/v1/plans/{plan_id}/archive`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn archive(&self, plan_id: impl Into<crate::models::PlanId>) -> crate::api::Call<()> {
        crate::request::Request::new(http::Method::POST, "/api/v1/plans/{plan_id}/archive")
            .with_path_param("plan_id", Into::<crate::models::PlanId>::into(plan_id))
            .with_options(&self.options)
            .empty(&self.cfg)
    }

    /// Publish a draft plan version
    ///
    /// Publishes the current draft version, making it the active version.
    ///
    /// `POST /api/v1/plans/{plan_id}/publish`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 409, 429).
    pub fn publish(
        &self,
        plan_id: impl Into<crate::models::PlanId>,
    ) -> crate::api::Call<crate::models::Plan> {
        crate::request::Request::new(http::Method::POST, "/api/v1/plans/{plan_id}/publish")
            .with_path_param("plan_id", Into::<crate::models::PlanId>::into(plan_id))
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Unarchive a plan
    ///
    /// `POST /api/v1/plans/{plan_id}/unarchive`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429).
    pub fn unarchive(&self, plan_id: impl Into<crate::models::PlanId>) -> crate::api::Call<()> {
        crate::request::Request::new(http::Method::POST, "/api/v1/plans/{plan_id}/unarchive")
            .with_path_param("plan_id", Into::<crate::models::PlanId>::into(plan_id))
            .with_options(&self.options)
            .empty(&self.cfg)
    }

    fn list_versions_request(
        &self,
        plan_id: impl Into<crate::models::PlanId>,
        options: impl Into<Option<PlansListVersionsOptions>>,
    ) -> crate::api::Call<crate::models::PlanVersionListResponse> {
        let PlansListVersionsOptions { page, per_page } = options.into().unwrap_or_default();

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
    pub fn list_versions(
        &self,
        plan_id: impl Into<crate::models::PlanId>,
        options: impl Into<Option<PlansListVersionsOptions>>,
    ) -> crate::api::PageCall<
        crate::models::PlanVersionListResponse,
        crate::models::PlanVersionSummary,
    > {
        static WALK: crate::api::pagination::Walk<PlansListVersionsOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let plan_id: crate::models::PlanId = plan_id.into();
        let this = self.clone();
        let call =
            move |options: PlansListVersionsOptions| this.list_versions_request(&plan_id, options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::PLANS_LIST_VERSIONS,
            call,
        )
    }
}
