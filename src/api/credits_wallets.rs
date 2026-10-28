// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`CreditsWallets::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreditsWalletsListOptions {
    /// Only the credit this subscription (`sub_…`) can use: the credit that belongs to it and
    /// the customer's credit that belongs to no subscription. Every wallet is still listed.
    pub subscription_id: Option<SubscriptionId>,
}

impl CreditsWalletsListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            subscription_id: None,
        }
    }

    /// Sets the `subscription_id` query parameter.
    #[must_use]
    pub fn subscription_id(mut self, subscription_id: impl Into<SubscriptionId>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    /// Sets the `subscription_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_subscription_id(mut self, subscription_id: Option<SubscriptionId>) -> Self {
        self.subscription_id = subscription_id;
        self
    }
}

/// Query and header parameters of [`CreditsWallets::check`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreditsWalletsCheckOptions {
    /// Defaults to the customer's currency.
    pub currency: Option<Currency>,
    /// Checks the wallet of this credit unit (its code) instead of a currency.
    pub credit_unit: Option<String>,
    /// Checks only the credit that can pay for this product: credit not restricted to products,
    /// and credit restricted to it. Absent checks all credit.
    pub product_id: Option<ProductId>,
    /// Checks only the credit this subscription (`sub_…`) can spend, net of its own usage no
    /// credit pays: the credit that belongs to it and the customer's credit that belongs to no
    /// subscription, once every subscription's unbilled usage is paid. `has_access` is then the
    /// decision the access gate makes for that subscription's usage. Absent checks the
    /// customer's credit as a whole.
    pub subscription_id: Option<SubscriptionId>,
}

impl CreditsWalletsCheckOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            currency: None,
            credit_unit: None,
            product_id: None,
            subscription_id: None,
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

    /// Sets the `product_id` query parameter.
    #[must_use]
    pub fn product_id(mut self, product_id: impl Into<ProductId>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }

    /// Sets the `product_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_product_id(mut self, product_id: Option<ProductId>) -> Self {
        self.product_id = product_id;
        self
    }

    /// Sets the `subscription_id` query parameter.
    #[must_use]
    pub fn subscription_id(mut self, subscription_id: impl Into<SubscriptionId>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    /// Sets the `subscription_id` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_subscription_id(mut self, subscription_id: Option<SubscriptionId>) -> Self {
        self.subscription_id = subscription_id;
        self
    }
}

/// Query and header parameters of [`CreditsWallets::update`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreditsWalletsUpdateOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl CreditsWalletsUpdateOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// The credits wallets API, from
/// [`Credits::wallets`](crate::api::Credits::wallets).
#[derive(Clone)]
pub struct CreditsWallets {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for CreditsWallets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreditsWallets")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl CreditsWallets {
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

    /// List wallets
    ///
    /// A customer's credit per currency and credit unit: the credit on account, the usage not
    /// invoiced yet and the balance left after it, with the wallet's alerts and automatic top-up. The
    /// customer's own currency comes first, even when empty. With `subscription_id`, each wallet
    /// counts only the credit that subscription can use.
    ///
    /// `GET /api/v1/customers/{id_or_alias}/wallets`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn list(
        &self,
        id_or_alias: &str,
        options: impl Into<Option<CreditsWalletsListOptions>>,
    ) -> crate::api::Call<crate::models::WalletListResponse> {
        let CreditsWalletsListOptions { subscription_id } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/customers/{id_or_alias}/wallets")
            .with_path_param("id_or_alias", id_or_alias)
            .with_optional_query_param("subscription_id", subscription_id)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Check wallet access
    ///
    /// Whether the customer's prepaid credit in a currency or credit unit still covers them, net of
    /// usage not invoiced yet. Denies only when the wallet blocks usage once its credit is used up;
    /// otherwise access stays allowed and `exhausted` signals the empty balance on its own. With
    /// `subscription_id`, only the credit that subscription can use counts: pass it when the
    /// customer has several subscriptions, since one subscription's credit cannot pay another's
    /// usage. The balance follows usage within a few seconds near zero or a threshold, within a
    /// minute otherwise, and the answer is cached for one second: use it to gate consumption, not
    /// to bill.
    ///
    /// `GET /api/v1/customers/{id_or_alias}/wallets/check`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn check(
        &self,
        id_or_alias: &str,
        options: impl Into<Option<CreditsWalletsCheckOptions>>,
    ) -> crate::api::Call<crate::models::WalletCheck> {
        let CreditsWalletsCheckOptions {
            currency,
            credit_unit,
            product_id,
            subscription_id,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/customers/{id_or_alias}/wallets/check",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_optional_query_param("currency", currency)
        .with_optional_query_param("credit_unit", credit_unit)
        .with_optional_query_param("product_id", product_id)
        .with_optional_query_param("subscription_id", subscription_id)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Configure a wallet
    ///
    /// Settings of a customer's wallet in one currency or credit unit: whether usage is refused
    /// once its credit is used up, the balances that fire `wallet.balance_low`, and its automatic
    /// top-up. Opens the wallet if it does not exist yet. Partial update: omit a field to leave it
    /// unchanged.
    ///
    /// `PATCH /api/v1/customers/{id_or_alias}/wallets/{currency}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 409, 429, 500).
    pub fn update(
        &self,
        id_or_alias: &str,
        currency: &str,
        patch_wallet_request: crate::models::PatchWalletRequest,
        options: CreditsWalletsUpdateOptions,
    ) -> crate::api::Call<crate::models::Wallet> {
        let CreditsWalletsUpdateOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/customers/{id_or_alias}/wallets/{currency}",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_path_param("currency", currency)
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_body_param(patch_wallet_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
