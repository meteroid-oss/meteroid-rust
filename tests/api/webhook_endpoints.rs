// this file is @generated
use crate::mock;

#[tokio::test]
async fn resend_webhook_delivery() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"attempt_count":-2147483648,"created_at":"2023-12-31T23:59:59.999-05:30","endpoint_id":"webhook_endpoint_id_44","event_type":"sample","id":"webhook_delivery_id_90","manual":false,"message_id":"event_id_90","status":"IN_FLIGHT"}"#,
    );
    client
        .webhook_endpoints()
        .resend_webhook_delivery("delivery_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/webhooks/deliveries/delivery_id/resend"]
    );
}
