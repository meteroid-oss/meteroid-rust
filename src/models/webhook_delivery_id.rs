// this file is @generated

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(transparent)]
pub struct WebhookDeliveryId(String);

impl WebhookDeliveryId {
    /// The value, as sent on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The value as a `String`.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::ops::Deref for WebhookDeliveryId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WebhookDeliveryId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for WebhookDeliveryId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for WebhookDeliveryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for WebhookDeliveryId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for WebhookDeliveryId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<&String> for WebhookDeliveryId {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<&WebhookDeliveryId> for WebhookDeliveryId {
    fn from(value: &WebhookDeliveryId) -> Self {
        value.clone()
    }
}

impl From<WebhookDeliveryId> for String {
    fn from(value: WebhookDeliveryId) -> Self {
        value.0
    }
}

impl PartialEq<str> for WebhookDeliveryId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for WebhookDeliveryId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for WebhookDeliveryId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl crate::request::QueryParamValue for WebhookDeliveryId {
    fn encode(&self) -> String {
        self.0.clone()
    }
}
