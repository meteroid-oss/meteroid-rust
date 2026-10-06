// this file is @generated
use crate::{decode, mock};

#[tokio::test]
async fn retrieve() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"created_at":"2023-12-31T23:59:59.999-05:30","feature_id":"feature_id_0","id":"entitlement_id_79","updated_at":"2024-03-15T10:30:45.123+02:00","value":{"type":"BOOLEAN","enabled":true}}"#,
    );
    client
        .entitlements()
        .retrieve("entitlement_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/entitlements/entitlement_id"]
    );
}

#[tokio::test]
async fn delete() {
    let (client, requests) = mock(204, None, r#""#);
    client
        .entitlements()
        .delete("entitlement_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["DELETE /api/v1/entitlements/entitlement_id"]
    );
}

#[tokio::test]
async fn update() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"created_at":"2023-12-31T23:59:59.999-05:30","feature_id":"feature_id_0","id":"entitlement_id_79","updated_at":"2024-03-15T10:30:45.123+02:00","value":{"type":"BOOLEAN","enabled":true}}"#,
    );
    client
        .entitlements()
        .update(
            "entitlement_id",
            decode::<meteroid::models::UpdateEntitlementRequest>(r#"{}"#),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["PATCH /api/v1/entitlements/entitlement_id"]
    );
}
