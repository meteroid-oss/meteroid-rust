// this file is @generated
use crate::{decode, mock};

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"archived":false,"code":"sample","id":"sample","name":"sample","plural_name":"sample","precision":123456789,"prices":[{"created_at":"2023-12-31T23:59:59.999-05:30","currency":"sample","id":"sample","unit_price":"sample"}]}]}"#,
    );
    client.credit_units().list(None).await.unwrap();
    assert_eq!(*requests.lock().unwrap(), ["GET /api/v1/credit-units"]);
}

#[tokio::test]
async fn create() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"archived":false,"code":"sample","id":"sample","name":"sample","plural_name":"sample","precision":2147483647,"prices":[{"created_at":"2024-03-15T10:30:45.123+02:00","currency":"sample","id":"sample","unit_price":"sample"}]}"#,
    );
    client
        .credit_units()
        .create(decode::<meteroid_rs::models::CreditUnitCreateRequest>(
            r#"{"code":"sample","name":"sample","plural_name":"sample","precision":123456789}"#,
        ))
        .await
        .unwrap();
    assert_eq!(*requests.lock().unwrap(), ["POST /api/v1/credit-units"]);
}

#[tokio::test]
async fn retrieve() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"archived":false,"code":"sample","id":"sample","name":"sample","plural_name":"sample","precision":2147483647,"prices":[{"created_at":"2024-03-15T10:30:45.123+02:00","currency":"sample","id":"sample","unit_price":"sample"}]}"#,
    );
    client.credit_units().retrieve("id").await.unwrap();
    assert_eq!(*requests.lock().unwrap(), ["GET /api/v1/credit-units/id"]);
}

#[tokio::test]
async fn update() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"archived":false,"code":"sample","id":"sample","name":"sample","plural_name":"sample","precision":2147483647,"prices":[{"created_at":"2024-03-15T10:30:45.123+02:00","currency":"sample","id":"sample","unit_price":"sample"}]}"#,
    );
    client
        .credit_units()
        .update(
            "id",
            decode::<meteroid_rs::models::CreditUnitUpdateRequest>(r#"{}"#),
        )
        .await
        .unwrap();
    assert_eq!(*requests.lock().unwrap(), ["PATCH /api/v1/credit-units/id"]);
}
