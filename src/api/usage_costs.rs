// this file is @generated
#![allow(clippy::doc_markdown, clippy::default_trait_access)]

#[allow(unused_imports, clippy::wildcard_imports)]
use crate::{error::Result, models::*, Configuration};

/// The usage costs API, from the client's
/// [`usage_costs`](crate::api::Meteroid::usage_costs).
#[derive(Clone)]
pub struct UsageCosts {
    cfg: std::sync::Arc<Configuration>,
    options: crate::api::RequestOptions,
}

impl std::fmt::Debug for UsageCosts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UsageCosts")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl UsageCosts {
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

    /// The customer API, with the options of this one.
    #[must_use]
    pub fn customer(&self) -> super::UsageCostsCustomer {
        super::UsageCostsCustomer::new(self.cfg.clone()).with_options(self.options.clone())
    }

    /// The subscription API, with the options of this one.
    #[must_use]
    pub fn subscription(&self) -> super::UsageCostsSubscription {
        super::UsageCostsSubscription::new(self.cfg.clone()).with_options(self.options.clone())
    }
}
