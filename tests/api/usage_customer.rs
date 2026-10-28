// this file is @generated
use crate::mock;

#[tokio::test]
async fn retrieve_unattributed() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"period_end":"1999-12-31","period_start":"2024-02-29","usage":[{"from":"2024-03-15T10:30:45.123+02:00","metric_code":"sample","metric_id":"billable_metric_id_44","metric_name":"sample","total_value":"12345.6789","until":"2023-12-31T23:59:59.999-05:30"}]}"#,
    );
    client
        .usage()
        .customer()
        .retrieve_unattributed("customer_id", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/usage/customer/customer_id/unattributed"]
    );
}
