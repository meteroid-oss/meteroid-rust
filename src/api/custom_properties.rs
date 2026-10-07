// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`CustomProperties::list_custom_property_definitions`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CustomPropertiesListCustomPropertyDefinitionsOptions {
    /// Filter to a single entity type.
    pub entity_type: Option<CustomPropertyEntityType>,
    /// Include archived (soft-deleted) definitions. Defaults to false.
    pub include_archived: Option<bool>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl CustomPropertiesListCustomPropertyDefinitionsOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entity_type: None,
            include_archived: None,
            page: None,
            per_page: None,
        }
    }

    /// Sets the `entity_type` query parameter.
    #[must_use]
    pub fn entity_type(mut self, entity_type: impl Into<CustomPropertyEntityType>) -> Self {
        self.entity_type = Some(entity_type.into());
        self
    }

    /// Sets the `entity_type` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_entity_type(mut self, entity_type: Option<CustomPropertyEntityType>) -> Self {
        self.entity_type = entity_type;
        self
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

/// The custom properties API, from the client's
/// [`custom_properties`](crate::api::Meteroid::custom_properties).
#[derive(Clone)]
pub struct CustomProperties {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for CustomProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomProperties")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl CustomProperties {
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

    fn list_custom_property_definitions_request(
        &self,
        options: impl Into<Option<CustomPropertiesListCustomPropertyDefinitionsOptions>>,
    ) -> crate::api::Call<crate::models::CustomPropertyDefinitionListResponse> {
        let CustomPropertiesListCustomPropertyDefinitionsOptions {
            entity_type,
            include_archived,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/custom-property-definitions")
            .with_optional_query_param("entity_type", entity_type)
            .with_optional_query_param("include_archived", include_archived)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// List custom property definitions
    ///
    /// `GET /api/v1/custom-property-definitions`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`CustomPropertyDefinition`](crate::models::CustomPropertyDefinition) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 429, 500).
    pub fn list_custom_property_definitions(
        &self,
        options: impl Into<Option<CustomPropertiesListCustomPropertyDefinitionsOptions>>,
    ) -> crate::api::PageCall<
        crate::models::CustomPropertyDefinitionListResponse,
        crate::models::CustomPropertyDefinition,
    > {
        static WALK: crate::api::pagination::Walk<
            CustomPropertiesListCustomPropertyDefinitionsOptions,
        > = crate::api::pagination::Walk::Page {
            first: 0,
            get: |options| options.page.map(i64::from),
            set: |options, position| options.page = i32::try_from(position).ok(),
        };
        let this = self.clone();
        let call = move |options: CustomPropertiesListCustomPropertyDefinitionsOptions| {
            this.list_custom_property_definitions_request(options)
        };
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::CUSTOM_PROPERTIES_LIST_CUSTOM_PROPERTY_DEFINITIONS,
            call,
        )
    }

    /// Create a custom property definition
    ///
    /// `POST /api/v1/custom-property-definitions`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 409, 429, 500).
    pub fn create_custom_property_definition(
        &self,
        custom_property_definition_create_request: crate::models::CustomPropertyDefinitionCreateRequest,
    ) -> crate::api::Call<crate::models::CustomPropertyDefinition> {
        crate::request::Request::new(http::Method::POST, "/api/v1/custom-property-definitions")
            .with_body_param(custom_property_definition_create_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Get a custom property definition
    ///
    /// `GET /api/v1/custom-property-definitions/{id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (404, 429, 500).
    pub fn retrieve_custom_property_definition(
        &self,
        id: &str,
    ) -> crate::api::Call<crate::models::CustomPropertyDefinition> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/custom-property-definitions/{id}",
        )
        .with_path_param("id", id)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Update a custom property definition
    ///
    /// `PUT /api/v1/custom-property-definitions/{id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (404, 429, 500).
    pub fn update_custom_property_definition(
        &self,
        id: &str,
        custom_property_definition_update_request: crate::models::CustomPropertyDefinitionUpdateRequest,
    ) -> crate::api::Call<crate::models::CustomPropertyDefinition> {
        crate::request::Request::new(
            http::Method::PUT,
            "/api/v1/custom-property-definitions/{id}",
        )
        .with_path_param("id", id)
        .with_body_param(custom_property_definition_update_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Archive a custom property definition
    ///
    /// Soft-deletes the definition. Existing property values on entities are preserved; the definition
    /// simply stops being enforced on new writes.
    ///
    /// `DELETE /api/v1/custom-property-definitions/{id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (404, 429, 500).
    pub fn archive_definition(
        &self,
        id: &str,
    ) -> crate::api::Call<crate::models::CustomPropertyDefinition> {
        crate::request::Request::new(
            http::Method::DELETE,
            "/api/v1/custom-property-definitions/{id}",
        )
        .with_path_param("id", id)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
