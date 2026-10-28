// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`Checks::check_entitlement`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ChecksCheckEntitlementOptions {
    /// Units the customer is about to consume. Defaults to 1.
    pub requested: Option<rust_decimal::Decimal>,
    /// The subscription (`sub_…`) the usage belongs to: only its own budgets and the customer-wide
    /// ones apply. Without it, a budget on one subscription refuses only once every subscription
    /// charging the feature's metric is refused by its own budget.
    pub subscription_id: Option<String>,
    /// The group to check, as `group[<property>]=<value>`: required when the entitlement is grouped
    pub group: Option<std::collections::HashMap<String, String>>,
}

impl ChecksCheckEntitlementOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            requested: None,
            subscription_id: None,
            group: None,
        }
    }

    /// Sets the `requested` query parameter.
    #[must_use]
    pub fn requested(mut self, requested: impl Into<rust_decimal::Decimal>) -> Self {
        self.requested = Some(requested.into());
        self
    }

    /// Sets the `requested` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_requested(mut self, requested: Option<rust_decimal::Decimal>) -> Self {
        self.requested = requested;
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

    /// Sets the `group` query parameter.
    #[must_use]
    pub fn group(mut self, group: impl Into<std::collections::HashMap<String, String>>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Sets the `group` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_group(mut self, group: Option<std::collections::HashMap<String, String>>) -> Self {
        self.group = group;
        self
    }
}

/// The checks API, from the client's
/// [`checks`](crate::api::Meteroid::checks).
#[derive(Clone)]
pub struct Checks {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Checks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Checks")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Checks {
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

    /// Check customer entitlement
    ///
    /// Whether the customer may consume `requested` more of a feature right now, given their
    /// entitlement, the hard budgets that count the feature's usage, and their prepaid credit.
    /// Boolean features answer from their enabled state; config features grant access and return
    /// their `value`. An `ALLOW` limit never refuses: `in_overage` tells when the requested units go
    /// beyond it. Sliding windows send no threshold webhooks. Pass the group as
    /// `group[<property>]=<value>`: it is required when the entitlement is grouped. Pass
    /// `subscription_id` when the usage belongs to one subscription, so that a hard budget on another
    /// subscription does not refuse it.
    ///
    /// The answer reflects usage up to `as_of`: typically less than a minute old, and less than 10
    /// seconds old once `near_limit` is true. A hard limit can therefore be exceeded by the usage of
    /// those few seconds. While `near_limit` is true, check before every request with its actual
    /// `requested` and do not cache the answer; `retry_after_ms` says when to check again. Use the
    /// check to gate access, not to bill.
    ///
    /// Beta: the request and response shapes of this endpoint may change before general availability.
    ///
    /// `GET /api/v1/customers/{id_or_alias}/entitlements/{feature_id_or_code}/check`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500, 503).
    pub fn check_entitlement(
        &self,
        id_or_alias: &str,
        feature_id_or_code: &str,
        options: impl Into<Option<ChecksCheckEntitlementOptions>>,
    ) -> crate::api::Call<crate::models::EntitlementCheck> {
        let ChecksCheckEntitlementOptions {
            requested,
            subscription_id,
            group,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/customers/{id_or_alias}/entitlements/{feature_id_or_code}/check",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_path_param("feature_id_or_code", feature_id_or_code)
        .with_optional_query_param("requested", requested)
        .with_optional_query_param("subscription_id", subscription_id)
        .with_structured_query_param("group", group, true, true)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
