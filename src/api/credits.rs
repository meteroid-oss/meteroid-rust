// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`Credits::list_customer_credit_ledger`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreditsListCustomerCreditLedgerOptions {
    /// Defaults to the customer's currency.
    pub currency: Option<Currency>,
    /// The ledger of this credit unit (its code) instead of a currency; excludes `currency`.
    pub credit_unit: Option<String>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl CreditsListCustomerCreditLedgerOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            currency: None,
            credit_unit: None,
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

/// Query and header parameters of [`Credits::create_customer_top_up`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreditsCreateCustomerTopUpOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl CreditsCreateCustomerTopUpOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// The credits API, from the client's
/// [`credits`](crate::api::Meteroid::credits).
#[derive(Clone)]
pub struct Credits {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Credits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credits")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Credits {
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

    /// The grants API, with the options of this one.
    #[must_use]
    pub fn grants(&self) -> super::CreditsGrants {
        super::CreditsGrants::new(self.cfg.clone()).with_options(self.options.clone())
    }

    /// The wallets API, with the options of this one.
    #[must_use]
    pub fn wallets(&self) -> super::CreditsWallets {
        super::CreditsWallets::new(self.cfg.clone()).with_options(self.options.clone())
    }

    fn list_customer_credit_ledger_request(
        &self,
        id_or_alias: &str,
        options: impl Into<Option<CreditsListCustomerCreditLedgerOptions>>,
    ) -> crate::api::Call<crate::models::LedgerListResponse> {
        let CreditsListCustomerCreditLedgerOptions {
            currency,
            credit_unit,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/customers/{id_or_alias}/credit-ledger",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_optional_query_param("currency", currency)
        .with_optional_query_param("credit_unit", credit_unit)
        .with_optional_query_param("page", page)
        .with_optional_query_param("per_page", per_page)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// List credit ledger entries
    ///
    /// Every movement of a customer's credit in one currency or credit unit, newest first, with the
    /// credit on account after it: credit granted, applied to invoices, given back, expired or
    /// rolled over.
    ///
    /// `GET /api/v1/customers/{id_or_alias}/credit-ledger`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`LedgerEntry`](crate::models::LedgerEntry) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn list_customer_credit_ledger(
        &self,
        id_or_alias: &str,
        options: impl Into<Option<CreditsListCustomerCreditLedgerOptions>>,
    ) -> crate::api::PageCall<crate::models::LedgerListResponse, crate::models::LedgerEntry> {
        static WALK: crate::api::pagination::Walk<CreditsListCustomerCreditLedgerOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let id_or_alias = id_or_alias.to_string();
        let this = self.clone();
        let call = move |options: CreditsListCustomerCreditLedgerOptions| {
            this.list_customer_credit_ledger_request(&id_or_alias, options)
        };
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::CREDITS_LIST_CUSTOMER_CREDIT_LEDGER,
            call,
        )
    }

    /// Create a top-up
    ///
    /// Sells credit: CHARGE bills a saved payment method now, SEND_INVOICE grants it once paid. The
    /// credit is PAID and applies to ALL charges. Currency credit is sold one for one, untaxed, as
    /// its usage is taxed when invoiced. With `credit_unit`, sells credits of that unit at its
    /// selling price in the customer's currency, taxed at purchase under the unit's product; refused
    /// when the unit has no price in that currency or no product. Without a saved payment method,
    /// CHARGE returns a hosted `checkout_url` where the customer pays.
    ///
    /// `POST /api/v1/customers/{id_or_alias}/top-ups`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 409, 429, 500).
    pub fn create_customer_top_up(
        &self,
        id_or_alias: &str,
        create_top_up_request: crate::models::CreateTopUpRequest,
        options: CreditsCreateCustomerTopUpOptions,
    ) -> crate::api::Call<crate::models::TopUpResponse> {
        let CreditsCreateCustomerTopUpOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/customers/{id_or_alias}/top-ups",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_body_param(create_top_up_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
