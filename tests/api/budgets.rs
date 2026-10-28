// this file is @generated
use crate::mock;

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"amount":-9007199254740993,"behavior":"SOFT","created_at":"2023-12-31T23:59:59.999-05:30","created_by":"TENANT","exact":false,"id":"sample","thresholds":[-123456789],"updated_at":"2024-03-15T10:30:45.123+02:00","version":123456789}],"pagination_meta":{"page":-123456789,"per_page":-123456789,"total_items":-9007199254740993,"total_pages":123456789}}"#,
    );
    client.budgets().list("id_or_alias", None).await.unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/customers/id_or_alias/budgets"]
    );
}
