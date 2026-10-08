// this file is @generated
//! The [`Meteroid`] client, its resources and what their calls take and return.
#[path = "../call.rs"]
mod call;
pub mod client;
#[path = "../event_stream.rs"]
mod event_stream;
#[path = "../middleware.rs"]
pub mod middleware;
#[path = "../upload.rs"]
pub mod upload;
pub use crate::connector::{http_client, HttpClient};
pub use call::{ApiResponse, Call};
pub use event_stream::{EventStream, SseEvent};
pub use upload::{RequestBody, Upload};
#[path = "../auth_schemes.rs"]
pub(crate) mod auth_schemes;
pub use auth_schemes::{BasicAuth, TokenProvider};
#[path = "../pagination.rs"]
pub(crate) mod pagination;
pub use pagination::{Page, PageCall, Pages, Paginator};
#[path = "../request_options.rs"]
mod request_options;
pub use request_options::RequestOptions;

/// The `http` crate, whose `HeaderMap`, `StatusCode` and `Method` the API uses.
pub use ::http;
pub use bytes::Bytes;

/// The error body nearly every operation documents.
pub type ErrorBody = crate::models::RestErrorResponse;

mod add_ons;
mod add_ons_entitlements;
mod batch_jobs;
mod checkout_sessions;
mod connect;
mod coupons;
mod credit_notes;
mod custom_properties;
mod customers;
mod entitlements;
mod events;
mod features;
mod invoices;
mod metrics;
mod oauth;
mod oauth_apps;
mod plans;
mod plans_versions;
mod product_families;
mod products;
mod products_entitlements;
mod subscriptions;
mod usage;

pub use self::{
    add_ons::{AddOns, AddOnsListOptions},
    add_ons_entitlements::AddOnsEntitlements,
    batch_jobs::{BatchJobs, BatchJobsListFailuresOptions, BatchJobsListOptions},
    checkout_sessions::{CheckoutSessions, CheckoutSessionsListOptions},
    client::{Meteroid, MeteroidBuilder},
    connect::Connect,
    coupons::{Coupons, CouponsListOptions},
    credit_notes::{CreditNotes, CreditNotesListOptions},
    custom_properties::{CustomProperties, CustomPropertiesListCustomPropertyDefinitionsOptions},
    customers::{Customers, CustomersListOptions},
    entitlements::Entitlements,
    events::Events,
    features::{Features, FeaturesListOptions},
    invoices::{Invoices, InvoicesListOptions},
    metrics::{Metrics, MetricsListOptions},
    oauth::Oauth,
    oauth_apps::OauthApps,
    plans::{Plans, PlansListOptions, PlansRetrieveOptions},
    plans_versions::{PlansVersions, PlansVersionsListOptions},
    product_families::{ProductFamilies, ProductFamiliesListOptions},
    products::{Products, ProductsListOptions},
    products_entitlements::ProductsEntitlements,
    subscriptions::{Subscriptions, SubscriptionsListOptions},
    usage::{
        Usage, UsageRetrieveCustomerOptions, UsageRetrieveSubscriptionOptions,
        UsageRetrieveSummaryOptions,
    },
};

/// Typed accessors of the paging fields of each paginated operation's response.
pub(crate) mod pages {
    use super::pagination::Fields;
    use crate::models;
    pub(crate) static ADD_ONS_LIST: Fields<models::AddOnListResponse, models::AddOn> = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
    pub(crate) static BATCH_JOBS_LIST: Fields<
        models::BatchJobListResponse,
        models::BatchJobResponse,
    > = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
    pub(crate) static BATCH_JOBS_LIST_FAILURES: Fields<
        models::BatchJobFailuresResponse,
        models::BatchJobItemFailureResponse,
    > = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(page.total_count)),
    };
    pub(crate) static COUPONS_LIST: Fields<models::CouponListResponse, models::Coupon> = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
    pub(crate) static CREDIT_NOTES_LIST: Fields<
        models::CreditNoteListResponse,
        models::CreditNote,
    > = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
    pub(crate) static CUSTOM_PROPERTIES_LIST_CUSTOM_PROPERTY_DEFINITIONS: Fields<
        models::CustomPropertyDefinitionListResponse,
        models::CustomPropertyDefinition,
    > = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
    pub(crate) static CUSTOMERS_LIST: Fields<models::CustomerListResponse, models::Customer> =
        Fields {
            items: |page| Some(&page.data),
            items_mut: |page| Some(&mut page.data),
            next_cursor: None,
            item_cursor: None,
            has_more: None,
            total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
        };
    pub(crate) static FEATURES_LIST: Fields<models::FeatureListResponse, models::Feature> =
        Fields {
            items: |page| Some(&page.data),
            items_mut: |page| Some(&mut page.data),
            next_cursor: None,
            item_cursor: None,
            has_more: None,
            total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
        };
    pub(crate) static INVOICES_LIST: Fields<models::InvoiceListResponse, models::Invoice> =
        Fields {
            items: |page| Some(&page.data),
            items_mut: |page| Some(&mut page.data),
            next_cursor: None,
            item_cursor: None,
            has_more: None,
            total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
        };
    pub(crate) static METRICS_LIST: Fields<models::MetricListResponse, models::MetricSummary> =
        Fields {
            items: |page| Some(&page.data),
            items_mut: |page| Some(&mut page.data),
            next_cursor: None,
            item_cursor: None,
            has_more: None,
            total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
        };
    pub(crate) static PLANS_LIST: Fields<models::PlanListResponse, models::Plan> = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
    pub(crate) static PLANS_VERSIONS_LIST: Fields<
        models::PlanVersionListResponse,
        models::PlanVersionSummary,
    > = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
    pub(crate) static PRODUCT_FAMILIES_LIST: Fields<
        models::ProductFamilyListResponse,
        models::ProductFamily,
    > = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
    pub(crate) static PRODUCTS_LIST: Fields<models::ProductListResponse, models::Product> =
        Fields {
            items: |page| Some(&page.data),
            items_mut: |page| Some(&mut page.data),
            next_cursor: None,
            item_cursor: None,
            has_more: None,
            total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
        };
    pub(crate) static SUBSCRIPTIONS_LIST: Fields<
        models::SubscriptionListResponse,
        models::Subscription,
    > = Fields {
        items: |page| Some(&page.data),
        items_mut: |page| Some(&mut page.data),
        next_cursor: None,
        item_cursor: None,
        has_more: None,
        total: Some(|page| Some(i64::from(page.pagination_meta.total_pages))),
    };
}
