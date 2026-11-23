// this file is @generated
use crate::{decode, mock};

#[tokio::test]
async fn ingest() {
    let (client, requests) = mock(200, Some("application/json"), r#"{}"#);
    client
        .events()
        .ingest(decode::<meteroid_rs::models::IngestEventsRequest>(
            r#"{"events":[{"code":"sample","customer_id":"sample","event_id":"sample"}]}"#,
        ))
        .await
        .unwrap();
    assert_eq!(*requests.lock().unwrap(), ["POST /api/v1/events/ingest"]);
}

#[tokio::test]
async fn amend() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"ingested_at":"2023-12-31T23:59:59.999-05:30"}"#,
    );
    client
        .events()
        .amend(
            "event_id",
            decode::<meteroid_rs::models::AmendEventRequest>(
                r#"{"properties":{"alpha":"sample"},"timestamp":"2023-12-31T23:59:59.999-05:30"}"#,
            ),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/events/event_id/amend"]
    );
}

#[tokio::test]
async fn void() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"ingested_at":"2023-12-31T23:59:59.999-05:30"}"#,
    );
    client
        .events()
        .void(
            "event_id",
            decode::<meteroid_rs::models::VoidEventRequest>(r#"{}"#),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["POST /api/v1/events/event_id/void"]
    );
}
