// this file is @generated
use crate::mock;

#[tokio::test]
async fn check_entitlement() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"as_of":"2023-12-31T23:59:59.999-05:30","exact":true,"feature":{"code":"sample","id":"feature_id_19"},"has_access":true,"in_overage":false,"lag_ms":9007199254740993,"limits":[{"behavior":"HARD","has_access":false,"limit":"-0.000123","remaining":"12345.6789","usage_limit_id":"usage_limit_id_23","used":"12345.6789"}],"near_limit":false,"reason":"INSUFFICIENT_BALANCE"}"#,
    );
    client
        .checks()
        .check_entitlement("id_or_alias", "feature_id_or_code", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/customers/id_or_alias/entitlements/feature_id_or_code/check"]
    );
}
