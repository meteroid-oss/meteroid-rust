// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`CreditsGrants::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreditsGrantsListOptions {
    /// Only grants in this currency.
    pub currency: Option<Currency>,
    /// Only grants in this credit unit (its code); excludes `currency`.
    pub credit_unit: Option<String>,
    /// The `status` query parameter.
    pub status: Option<CreditGrantStatus>,
    /// Only the grants that belong to this subscription (`sub_…`); excludes
    /// `usable_by_subscription_id`.
    pub subscription_id: Option<String>,
    /// Only the credit this subscription (`sub_…`) can use: its own grants and the customer's
    /// grants that belong to no subscription. Excludes `subscription_id`.
    pub usable_by_subscription_id: Option<String>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl CreditsGrantsListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            currency: None,
            credit_unit: None,
            status: None,
            subscription_id: None,
            usable_by_subscription_id: None,
            page: None,
            per_page: None,
        }
    }

    /// Sets the `currency` query parameter.
    #[must_use]
    pub fn currency(mut self, currency: impl Into<Currency>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Sets the `currency` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_currency(mut self, currency: Option<Currency>) -> Self {
        self.currency = currency;
        self
    }

    /// Sets the `credit_unit` query parameter.
    #[must_use]
    pub fn credit_unit(mut self, credit_unit: impl Into<String>) -> Self {
        self.credit_unit = Some(credit_unit.into());
        self
    }

    /// Sets the `credit_unit` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_credit_unit(mut self, credit_unit: Option<String>) -> Self {
        self.credit_unit = credit_unit;
        self
    }

    /// Sets the `status` query parameter.
    #[must_use]
    pub fn status(mut self, status: impl Into<CreditGrantStatus>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Sets the `status` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_status(mut self, status: Option<CreditGrantStatus>) -> Self {
        self.status = status;
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

    /// Sets the `usable_by_subscription_id` query parameter.
    #[must_use]
    pub fn usable_by_subscription_id(
        mut self,
        usable_by_subscription_id: impl Into<String>,
    ) -> Self {
        self.usable_by_subscription_id = Some(usable_by_subscription_id.into());
        self
    }

    /// Sets the `usable_by_subscription_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_usable_by_subscription_id(
        mut self,
        usable_by_subscription_id: Option<String>,
    ) -> Self {
        self.usable_by_subscription_id = usable_by_subscription_id;
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

/// Query and header parameters of [`CreditsGrants::create`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreditsGrantsCreateOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl CreditsGrantsCreateOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// The credits grants API, from
/// [`Credits::grants`](crate::api::Credits::grants).
#[derive(Clone)]
pub struct CreditsGrants {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for CreditsGrants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreditsGrants")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl CreditsGrants {
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
        id_or_alias: &str,
        options: impl Into<Option<CreditsGrantsListOptions>>,
    ) -> crate::api::Call<crate::models::CreditGrantListResponse> {
        let CreditsGrantsListOptions {
            currency,
            credit_unit,
            status,
            subscription_id,
            usable_by_subscription_id,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/customers/{id_or_alias}/credit-grants",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_optional_query_param("currency", currency)
        .with_optional_query_param("credit_unit", credit_unit)
        .with_optional_query_param("status", status)
        .with_optional_query_param("subscription_id", subscription_id)
        .with_optional_query_param("usable_by_subscription_id", usable_by_subscription_id)
        .with_optional_query_param("page", page)
        .with_optional_query_param("per_page", per_page)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// List credit grants
    ///
    /// A customer's credit grants, newest first, in every currency and credit unit unless filtered.
    ///
    /// `GET /api/v1/customers/{id_or_alias}/credit-grants`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`CreditGrant`](crate::models::CreditGrant) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn list(
        &self,
        id_or_alias: &str,
        options: impl Into<Option<CreditsGrantsListOptions>>,
    ) -> crate::api::PageCall<crate::models::CreditGrantListResponse, crate::models::CreditGrant>
    {
        static WALK: crate::api::pagination::Walk<CreditsGrantsListOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let id_or_alias = id_or_alias.to_string();
        let this = self.clone();
        let call =
            move |options: CreditsGrantsListOptions| this.list_request(&id_or_alias, options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::CREDITS_GRANTS_LIST,
            call,
        )
    }

    /// Create a credit grant
    ///
    /// Grants credit without charging or invoicing: PROMOTIONAL credit for free, or PAID credit the
    /// customer paid for outside Meteroid ("Paid outside Meteroid"). A PAID currency grant whose
    /// `cost` is below its `amount` is split into a PAID grant for the cost and a PROMOTIONAL bonus
    /// grant for the rest, returned as `bonus_grant_id`; a `cost` above the `amount` is refused.
    /// For a customer of a French invoicing entity, PAID credit that cost more than zero needs a
    /// `reference` to the invoice or receipt of its payment. Sell credit through Meteroid with a
    /// top-up.
    ///
    /// `POST /api/v1/customers/{id_or_alias}/credit-grants`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 409, 429, 500).
    pub fn create(
        &self,
        id_or_alias: &str,
        create_credit_grant_request: crate::models::CreateCreditGrantRequest,
        options: CreditsGrantsCreateOptions,
    ) -> crate::api::Call<crate::models::CreditGrant> {
        let CreditsGrantsCreateOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/customers/{id_or_alias}/credit-grants",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_body_param(create_credit_grant_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
