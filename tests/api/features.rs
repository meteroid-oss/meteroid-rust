// this file is @generated
use crate::{decode, mock};

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"code":"sample","created_at":"2023-12-31T23:59:59.999-05:30","feature_type":{"type":"BOOLEAN"},"id":"feature_id_0","name":"sample","status":"DISABLED"}],"pagination_meta":{"page":-123456789,"per_page":-123456789,"total_items":-9007199254740993,"total_pages":123456789}}"#,
    );
    client.features().list(None).await.unwrap();
    assert_eq!(*requests.lock().unwrap(), ["GET /api/v1/features"]);
}

#[tokio::test]
async fn create() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"code":"sample","created_at":"2024-03-15T10:30:45.123+02:00","feature_type":{"type":"BOOLEAN"},"id":"feature_id_90","name":"sample","status":"ARCHIVED"}"#,
    );
    client
        .features()
        .create(decode::<meteroid::models::CreateFeatureRequest>(
            r#"{"code":"sample","feature_type":{"type":"BOOLEAN"},"name":"sample"}"#,
        ))
        .await
        .unwrap();
    assert_eq!(*requests.lock().unwrap(), ["POST /api/v1/features"]);
}

#[tokio::test]
async fn retrieve() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"code":"sample","created_at":"2024-03-15T10:30:45.123+02:00","feature_type":{"type":"BOOLEAN"},"id":"feature_id_90","name":"sample","status":"ARCHIVED"}"#,
    );
    client.features().retrieve("id_or_code").await.unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/features/id_or_code"]
    );
}

#[tokio::test]
async fn update() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"code":"sample","created_at":"2024-03-15T10:30:45.123+02:00","feature_type":{"type":"BOOLEAN"},"id":"feature_id_90","name":"sample","status":"ARCHIVED"}"#,
    );
    client
        .features()
        .update(
            "id_or_code",
            decode::<meteroid::models::UpdateFeatureRequest>(r#"{}"#),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["PATCH /api/v1/features/id_or_code"]
    );
}

#[tokio::test]
async fn archive() {
    let (client, requests) = mock(204, None, r#""#);
    client.features().archive("id_or_code").await.unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/features/id_or_code/archive"]
    );
}

#[tokio::test]
async fn unarchive() {
    let (client, requests) = mock(204, None, r#""#);
    client.features().unarchive("id_or_code").await.unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/features/id_or_code/unarchive"]
    );
}
