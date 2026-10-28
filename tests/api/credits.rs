// this file is @generated
use crate::mock;

#[tokio::test]
async fn list_customer_credit_ledger() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"amount":-9007199254740993,"applies_to":"USAGE","balance_after":9007199254740993,"credit_grant_id":"sample","effective_at":"2023-12-31T23:59:59.999-05:30","id":"sample","reason":"sample","recorded_at":"2023-12-31T23:59:59.999-05:30","sequence":9007199254740993,"type":"CREDIT"}],"pagination_meta":{"page":-123456789,"per_page":-123456789,"total_items":-9007199254740993,"total_pages":123456789}}"#,
    );
    client
        .credits()
        .list_customer_credit_ledger("id_or_alias", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/customers/id_or_alias/credit-ledger"]
    );
}
