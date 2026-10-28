// this file is @generated
use crate::mock;

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"credit_on_account":9007199254740993,"credit_on_account_usage_only":-9007199254740993,"low_balance_thresholds":[-9007199254740993],"pauses_at_zero":false,"subscription_credit":[{"credit_on_account":-9007199254740993,"subscription_id":"sample"}],"upcoming_credit":9007199254740993,"usage_at_zero":"FOLLOW_PLAN"}]}"#,
    );
    client
        .credits()
        .wallets()
        .list("id_or_alias", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/customers/id_or_alias/wallets"]
    );
}

#[tokio::test]
async fn check() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"as_of":"2023-12-31T23:59:59.999-05:30","balance":9007199254740993,"exact":true,"exhausted":false,"has_access":true,"lag_ms":-9007199254740993,"pauses_at_zero":true}"#,
    );
    client
        .credits()
        .wallets()
        .check("id_or_alias", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/customers/id_or_alias/wallets/check"]
    );
}
