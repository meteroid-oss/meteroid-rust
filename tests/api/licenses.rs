// this file is @generated
use crate::{decode, mock};

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"auto_issued":false,"created_at":"2023-12-31T23:59:59.999-05:30","customer_id":"customer_id_2","customer_name":"sample","has_activation_code":false,"id":"license_id_0","offline":false,"revoked_immediately":false,"status":"REVOKED","validity_days":-123456789}]}"#,
    );
    client.licenses().list(None).await.unwrap();
    assert_eq!(*requests.lock().unwrap(), ["GET /api/v1/licenses"]);
}

#[tokio::test]
async fn create() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"license":{"auto_issued":false,"created_at":"2023-12-31T23:59:59.999-05:30","customer_id":"customer_id_37","customer_name":"sample","has_activation_code":true,"id":"license_id_40","offline":true,"revoked_immediately":true,"status":"PENDING","validity_days":123456789}}"#,
    );
    client
        .licenses()
        .create(decode::<meteroid_rs::models::IssueLicenseRequest>(
            r#"{"customer_id":"sample"}"#,
        ))
        .await
        .unwrap();
    assert_eq!(*requests.lock().unwrap(), ["POST /api/v1/licenses"]);
}

#[tokio::test]
async fn retrieve() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"auto_issued":false,"created_at":"2024-03-15T10:30:45.123+02:00","customer_id":"customer_id_79","customer_name":"sample","has_activation_code":false,"id":"license_id_90","offline":false,"revoked_immediately":true,"status":"PENDING","validity_days":-123456789}"#,
    );
    client.licenses().retrieve("license_id").await.unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/licenses/license_id"]
    );
}

#[tokio::test]
async fn retrieve_activation_code() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"activation_code":"sample"}"#,
    );
    client
        .licenses()
        .retrieve_activation_code("license_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/licenses/license_id/activation-code"]
    );
}

#[tokio::test]
async fn offline() {
    let (client, requests) = mock(200, Some("application/json"), r#"{"license":"sample"}"#);
    client
        .licenses()
        .offline(
            "license_id",
            decode::<meteroid_rs::models::IssueOfflineLicenseRequest>(
                r#"{"valid_days":123456789}"#,
            ),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/licenses/license_id/offline"]
    );
}

#[tokio::test]
async fn revoke() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"auto_issued":false,"created_at":"2024-03-15T10:30:45.123+02:00","customer_id":"customer_id_79","customer_name":"sample","has_activation_code":false,"id":"license_id_90","offline":false,"revoked_immediately":true,"status":"PENDING","validity_days":-123456789}"#,
    );
    client
        .licenses()
        .revoke(
            "license_id",
            decode::<meteroid_rs::models::RevokeLicenseRequest>(r#"{}"#),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/licenses/license_id/revoke"]
    );
}

#[tokio::test]
async fn list_licensing_signing_keys() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"endpoints":{"activate_url":"sample","events_url":"sample","jwks_url":"sample","refresh_url":"sample"},"keys":[{"created_at":"2023-12-31T23:59:59.999-05:30","kid":"sample","public_key_pem":"sample","status":"ACTIVE"}]}"#,
    );
    client
        .licenses()
        .list_licensing_signing_keys()
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/licensing/signing-keys"]
    );
}
