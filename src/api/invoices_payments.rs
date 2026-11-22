// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`InvoicesPayments::create`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct InvoicesPaymentsCreateOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl InvoicesPaymentsCreateOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// Query and header parameters of [`InvoicesPayments::reverse`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct InvoicesPaymentsReverseOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl InvoicesPaymentsReverseOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// The invoices payments API, from
/// [`Invoices::payments`](crate::api::Invoices::payments).
#[derive(Clone)]
pub struct InvoicesPayments {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for InvoicesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InvoicesPayments")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl InvoicesPayments {
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

    /// Record an invoice payment
    ///
    /// Record a partial or full payment received outside Meteroid against a finalized invoice.
    /// Paying the remaining balance marks the invoice paid.
    ///
    /// `POST /api/v1/invoices/{invoice_id}/payments`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 403, 404, 409, 429, 500).
    pub fn create(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
        record_invoice_payment_request: crate::models::RecordInvoicePaymentRequest,
        options: InvoicesPaymentsCreateOptions,
    ) -> crate::api::Call<crate::models::Transaction> {
        let InvoicesPaymentsCreateOptions { idempotency_key } = options;

        crate::request::Request::new(http::Method::POST, "/api/v1/invoices/{invoice_id}/payments")
            .with_path_param(
                "invoice_id",
                Into::<crate::models::InvoiceId>::into(invoice_id),
            )
            .with_header_param("Idempotency-Key", &idempotency_key)
            .with_body_param(record_invoice_payment_request)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Reverse an invoice payment
    ///
    /// Record the full reversal of a settled manual payment.
    ///
    /// `POST /api/v1/invoices/{invoice_id}/payments/{transaction_id}/reverse`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 403, 404, 409, 429, 500).
    pub fn reverse(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
        transaction_id: impl Into<crate::models::PaymentTransactionId>,
        reverse_invoice_payment_request: crate::models::ReverseInvoicePaymentRequest,
        options: InvoicesPaymentsReverseOptions,
    ) -> crate::api::Call<crate::models::Transaction> {
        let InvoicesPaymentsReverseOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/invoices/{invoice_id}/payments/{transaction_id}/reverse",
        )
        .with_path_param(
            "invoice_id",
            Into::<crate::models::InvoiceId>::into(invoice_id),
        )
        .with_path_param(
            "transaction_id",
            Into::<crate::models::PaymentTransactionId>::into(transaction_id),
        )
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_body_param(reverse_invoice_payment_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }
}
