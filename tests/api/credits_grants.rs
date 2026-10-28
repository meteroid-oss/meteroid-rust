// this file is @generated
use crate::mock;

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"amount":-9007199254740993,"applicable_product_ids":["product_id_39"],"applies_to":"ALL","created_at":"2024-03-15T10:30:45.123+02:00","effective_at":"2023-12-31T23:59:59.999-05:30","id":"sample","kind":"ACCOUNT_BALANCE","kind_label":"sample","priority":123456789,"reason":"sample","remaining":-9007199254740993,"source":"OVERPAYMENT","status":"ACTIVE","type":"PAID"}],"pagination_meta":{"page":-123456789,"per_page":-123456789,"total_items":-9007199254740993,"total_pages":123456789}}"#,
    );
    client
        .credits()
        .grants()
        .list("id_or_alias", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/customers/id_or_alias/credit-grants"]
    );
}
