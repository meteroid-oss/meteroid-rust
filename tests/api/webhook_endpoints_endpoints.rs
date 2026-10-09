// this file is @generated
use crate::{decode, mock};

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"consecutive_failures":-123456789,"created_at":"2023-12-31T23:59:59.999-05:30","disabled":true,"event_types":["sample"],"headers":[{"name":"sample","sensitive":false,"set":true}],"id":"webhook_endpoint_id_25","max_in_flight":-2147483648,"needs_setup":false,"url":"sample"}]}"#,
    );
    client.webhook_endpoints().endpoints().list().await.unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/webhooks/endpoints"]
    );
}

#[tokio::test]
async fn create() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"consecutive_failures":-2147483648,"created_at":"2024-03-15T10:30:45.123+02:00","disabled":true,"event_types":["sample"],"headers":[{"name":"sample","sensitive":true,"set":true}],"id":"webhook_endpoint_id_40","max_in_flight":-123456789,"needs_setup":true,"url":"sample","secret":"sample"}"#,
    );
    client
        .webhook_endpoints()
        .endpoints()
        .create(decode::<meteroid_rs::models::CreateWebhookEndpointRequest>(
            r#"{"url":"sample"}"#,
        ))
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/webhooks/endpoints"]
    );
}

#[tokio::test]
async fn retrieve() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"consecutive_failures":-2147483648,"created_at":"2024-03-15T10:30:45.123+02:00","disabled":true,"event_types":["sample"],"headers":[{"name":"sample","sensitive":true,"set":true}],"id":"webhook_endpoint_id_40","max_in_flight":-123456789,"needs_setup":true,"url":"sample"}"#,
    );
    client
        .webhook_endpoints()
        .endpoints()
        .retrieve("endpoint_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/webhooks/endpoints/endpoint_id"]
    );
}

#[tokio::test]
async fn delete() {
    let (client, requests) = mock(204, None, r#""#);
    client
        .webhook_endpoints()
        .endpoints()
        .delete("endpoint_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["DELETE /api/v1/webhooks/endpoints/endpoint_id"]
    );
}

#[tokio::test]
async fn update() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"consecutive_failures":-2147483648,"created_at":"2024-03-15T10:30:45.123+02:00","disabled":true,"event_types":["sample"],"headers":[{"name":"sample","sensitive":true,"set":true}],"id":"webhook_endpoint_id_40","max_in_flight":-123456789,"needs_setup":true,"url":"sample"}"#,
    );
    client
        .webhook_endpoints()
        .endpoints()
        .update(
            "endpoint_id",
            decode::<meteroid_rs::models::UpdateWebhookEndpointRequest>(r#"{}"#),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["PATCH /api/v1/webhooks/endpoints/endpoint_id"]
    );
}

#[tokio::test]
async fn list_deliveries() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"attempt_count":-123456789,"created_at":"2024-03-15T10:30:45.123+02:00","endpoint_id":"webhook_endpoint_id_90","event_type":"sample","id":"webhook_delivery_id_0","manual":false,"message_id":"event_id_67","status":"SUCCEEDED"}],"pagination_meta":{"page":-123456789,"per_page":-123456789,"total_items":-9007199254740993,"total_pages":123456789}}"#,
    );
    client
        .webhook_endpoints()
        .endpoints()
        .list_deliveries("endpoint_id", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/webhooks/endpoints/endpoint_id/deliveries"]
    );
}

#[tokio::test]
async fn rotate_secret() {
    let (client, requests) = mock(200, Some("application/json"), r#"{"secret":"sample"}"#);
    client
        .webhook_endpoints()
        .endpoints()
        .rotate_secret("endpoint_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/webhooks/endpoints/endpoint_id/rotate-secret"]
    );
}

#[tokio::test]
async fn retrieve_secret() {
    let (client, requests) = mock(200, Some("application/json"), r#"{"secret":"sample"}"#);
    client
        .webhook_endpoints()
        .endpoints()
        .retrieve_secret("endpoint_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/webhooks/endpoints/endpoint_id/secret"]
    );
}
