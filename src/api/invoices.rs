// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// Query and header parameters of [`Invoices::list`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct InvoicesListOptions {
    /// Filter by customer ID or alias
    pub customer_id: Option<String>,
    /// The `subscription_id` query parameter.
    pub subscription_id: Option<SubscriptionId>,
    /// The `statuses` query parameter.
    pub statuses: Option<Vec<InvoiceStatus>>,
    /// Only invoices whose e-invoice was generated, or failed. Invoices from entities that
    /// had not opted in carry no status and match neither.
    pub einvoicing_status: Option<EInvoicingStatus>,
    /// Sort order. Format: `column.direction`. Allowed columns: `invoice_number`, `customer_name`, `amount`, `invoice_date`, `status`, `payment_status`. Direction: `asc` or `desc`. Default: `invoice_date.desc`.
    pub order_by: Option<String>,
    /// Page number (0-indexed)
    pub page: Option<i32>,
    /// Number of items per page
    pub per_page: Option<i32>,
}

impl InvoicesListOptions {
    /// Options with no parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            customer_id: None,
            subscription_id: None,
            statuses: None,
            einvoicing_status: None,
            order_by: None,
            page: None,
            per_page: None,
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

    /// Sets the `statuses` query parameter.
    #[must_use]
    pub fn statuses(mut self, statuses: impl Into<Vec<InvoiceStatus>>) -> Self {
        self.statuses = Some(statuses.into());
        self
    }

    /// Sets the `statuses` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_statuses(mut self, statuses: Option<Vec<InvoiceStatus>>) -> Self {
        self.statuses = statuses;
        self
    }

    /// Sets the `einvoicing_status` query parameter.
    #[must_use]
    pub fn einvoicing_status(mut self, einvoicing_status: impl Into<EInvoicingStatus>) -> Self {
        self.einvoicing_status = Some(einvoicing_status.into());
        self
    }

    /// Sets the `einvoicing_status` query parameter, or unsets it with `None`.
    #[must_use]
    pub fn maybe_einvoicing_status(mut self, einvoicing_status: Option<EInvoicingStatus>) -> Self {
        self.einvoicing_status = einvoicing_status;
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

/// Query and header parameters of [`Invoices::mark_paid`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct InvoicesMarkPaidOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl InvoicesMarkPaidOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// Query and header parameters of [`Invoices::mark_uncollectible`], built
/// with the required ones by [`new`](Self::new).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct InvoicesMarkUncollectibleOptions {
    /// The `Idempotency-Key` header parameter.
    pub idempotency_key: String,
}

impl InvoicesMarkUncollectibleOptions {
    /// Options with the required parameters set.
    #[must_use]
    pub fn new(idempotency_key: impl Into<String>) -> Self {
        Self {
            idempotency_key: idempotency_key.into(),
        }
    }
}

/// The invoices API, from the client's
/// [`invoices`](crate::api::Meteroid::invoices).
#[derive(Clone)]
pub struct Invoices {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for Invoices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Invoices")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Invoices {
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

    /// The payments API, with the options of this one.
    #[must_use]
    pub fn payments(&self) -> super::InvoicesPayments {
        super::InvoicesPayments::new(self.cfg.clone()).with_options(self.options.clone())
    }

    fn list_request(
        &self,
        options: impl Into<Option<InvoicesListOptions>>,
    ) -> crate::api::Call<crate::models::InvoiceListResponse> {
        let InvoicesListOptions {
            customer_id,
            subscription_id,
            statuses,
            einvoicing_status,
            order_by,
            page,
            per_page,
        } = options.into().unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/invoices")
            .with_optional_query_param("customer_id", customer_id)
            .with_optional_query_param("subscription_id", subscription_id)
            .with_optional_exploded_query_param("statuses", statuses)
            .with_optional_query_param("einvoicing_status", einvoicing_status)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// List invoices with optional filtering by customer, subscription, or status.
    ///
    /// `GET /api/v1/invoices`, a page at a time: awaiting the call gives the first
    /// [`Page`](crate::api::Page), [`items`](crate::api::PageCall::items) every
    /// [`Invoice`](crate::models::Invoice) across pages
    /// and [`pages`](crate::api::PageCall::pages) every page.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 429, 500).
    pub fn list(
        &self,
        options: impl Into<Option<InvoicesListOptions>>,
    ) -> crate::api::PageCall<crate::models::InvoiceListResponse, crate::models::Invoice> {
        static WALK: crate::api::pagination::Walk<InvoicesListOptions> =
            crate::api::pagination::Walk::Page {
                first: 0,
                get: |options| options.page.map(i64::from),
                set: |options, position| options.page = i32::try_from(position).ok(),
            };
        let this = self.clone();
        let call = move |options: InvoicesListOptions| this.list_request(options);
        crate::api::PageCall::new(
            options.into().unwrap_or_default(),
            WALK,
            crate::api::pages::INVOICES_LIST,
            call,
        )
    }

    /// Get invoice
    ///
    /// Retrieve a single invoice with its payment transactions.
    ///
    /// `GET /api/v1/invoices/{invoice_id}`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn retrieve(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
    ) -> crate::api::Call<crate::models::Invoice> {
        crate::request::Request::new(http::Method::GET, "/api/v1/invoices/{invoice_id}")
            .with_path_param(
                "invoice_id",
                Into::<crate::models::InvoiceId>::into(invoice_id),
            )
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Update invoice custom properties
    ///
    /// Merge custom property values onto an invoice (send a key with `null` to remove it).
    /// Values are validated against the tenant's `INVOICE` property definitions. Allowed at any
    /// status — custom properties are external workflow metadata and stay editable after the invoice
    /// is finalized.
    ///
    /// `PATCH /api/v1/invoices/{invoice_id}/custom-properties`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn update_custom_properties(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
        invoice_custom_properties_request: crate::models::InvoiceCustomPropertiesRequest,
    ) -> crate::api::Call<crate::models::Invoice> {
        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/invoices/{invoice_id}/custom-properties",
        )
        .with_path_param(
            "invoice_id",
            Into::<crate::models::InvoiceId>::into(invoice_id),
        )
        .with_body_param(invoice_custom_properties_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Download invoice PDF
    ///
    /// Download the PDF document for an invoice.
    ///
    /// `GET /api/v1/invoices/{invoice_id}/download`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn download(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
    ) -> crate::api::Call<bytes::Bytes> {
        crate::request::Request::new(http::Method::GET, "/api/v1/invoices/{invoice_id}/download")
            .with_path_param(
                "invoice_id",
                Into::<crate::models::InvoiceId>::into(invoice_id),
            )
            .with_options(&self.options)
            .binary(&self.cfg)
    }

    /// Mark an invoice as paid
    ///
    /// Record an external payment for the entire remaining balance of a finalized invoice. The amount
    /// must match the remaining balance.
    ///
    /// `POST /api/v1/invoices/{invoice_id}/mark-paid`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 403, 404, 409, 429, 500).
    pub fn mark_paid(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
        record_invoice_payment_request: crate::models::RecordInvoicePaymentRequest,
        options: InvoicesMarkPaidOptions,
    ) -> crate::api::Call<crate::models::Transaction> {
        let InvoicesMarkPaidOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/invoices/{invoice_id}/mark-paid",
        )
        .with_path_param(
            "invoice_id",
            Into::<crate::models::InvoiceId>::into(invoice_id),
        )
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_body_param(record_invoice_payment_request)
        .with_options(&self.options)
        .json(&self.cfg)
    }

    /// Mark an invoice as uncollectible
    ///
    /// Transition a finalized invoice to uncollectible and stop its payment reminders.
    ///
    /// `POST /api/v1/invoices/{invoice_id}/mark-uncollectible`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 403, 404, 409, 429, 500).
    pub fn mark_uncollectible(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
        options: InvoicesMarkUncollectibleOptions,
    ) -> crate::api::Call<()> {
        let InvoicesMarkUncollectibleOptions { idempotency_key } = options;

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/invoices/{invoice_id}/mark-uncollectible",
        )
        .with_path_param(
            "invoice_id",
            Into::<crate::models::InvoiceId>::into(invoice_id),
        )
        .with_header_param("Idempotency-Key", &idempotency_key)
        .with_options(&self.options)
        .empty(&self.cfg)
    }

    /// Refresh invoice
    ///
    /// Recompute a draft invoice against current usage, credits, coupons and tax, and return it.
    /// Drafts are also refreshed periodically in the background; use this to force it, e.g. after
    /// ingesting late events. Rejected while a payment for the invoice is in progress or when the
    /// invoice was merged into a consolidated parent.
    ///
    /// `POST /api/v1/invoices/{invoice_id}/refresh`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (400, 401, 404, 429, 500).
    pub fn refresh(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
    ) -> crate::api::Call<crate::models::Invoice> {
        crate::request::Request::new(http::Method::POST, "/api/v1/invoices/{invoice_id}/refresh")
            .with_path_param(
                "invoice_id",
                Into::<crate::models::InvoiceId>::into(invoice_id),
            )
            .with_options(&self.options)
            .json(&self.cfg)
    }

    /// Download invoice e-invoice XML
    ///
    /// Download the structured e-invoice (EN 16931 XML) issued with an invoice. For
    /// Factur-X the same XML is also embedded in the PDF.
    ///
    /// `GET /api/v1/invoices/{invoice_id}/xml`.
    ///
    /// # Errors
    ///
    /// An API error's body is [`RestErrorResponse`](crate::models::RestErrorResponse) (401, 404, 429, 500).
    pub fn download_xml(
        &self,
        invoice_id: impl Into<crate::models::InvoiceId>,
    ) -> crate::api::Call<bytes::Bytes> {
        crate::request::Request::new(http::Method::GET, "/api/v1/invoices/{invoice_id}/xml")
            .with_path_param(
                "invoice_id",
                Into::<crate::models::InvoiceId>::into(invoice_id),
            )
            .with_options(&self.options)
            .binary(&self.cfg)
    }
}
