// this file is @generated
use crate::{decode, mock};

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"created_at":"2023-12-31T23:59:59.999-05:30","currency":"sample","id":"sample","unit_price":"sample"}]}"#,
    );
    client
        .credit_units()
        .prices()
        .list("id", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/credit-units/id/prices"]
    );
}

#[tokio::test]
async fn update() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"created_at":"2024-03-15T10:30:45.123+02:00","currency":"sample","id":"sample","unit_price":"sample"}"#,
    );
    client
        .credit_units()
        .prices()
        .update(
            "id",
            "currency",
            decode::<meteroid_rs::models::CreditUnitPriceSetRequest>(r#"{"unit_price":"sample"}"#),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["PUT /api/v1/credit-units/id/prices/currency"]
    );
}

#[tokio::test]
async fn archive() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"created_at":"2024-03-15T10:30:45.123+02:00","currency":"sample","id":"sample","unit_price":"sample"}"#,
    );
    client
        .credit_units()
        .prices()
        .archive("id", "currency")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["DELETE /api/v1/credit-units/id/prices/currency"]
    );
}
