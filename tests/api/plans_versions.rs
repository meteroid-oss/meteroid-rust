// this file is @generated
use crate::{decode, mock};

#[tokio::test]
async fn update_minimum() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"amount":"sample","scope":{"type":"all_components"}}"#,
    );
    client
        .plans()
        .versions()
        .update_minimum(
            "plan_version_id",
            decode::<meteroid_rs::models::MinimumCommitment>(
                r#"{"amount":"sample","scope":{"type":"all_components"}}"#,
            ),
        )
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["PUT /api/v1/plans/versions/plan_version_id/minimum"]
    );
}

#[tokio::test]
async fn delete_minimum() {
    let (client, requests) = mock(204, None, r#""#);
    client
        .plans()
        .versions()
        .delete_minimum("plan_version_id")
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["DELETE /api/v1/plans/versions/plan_version_id/minimum"]
    );
}

#[tokio::test]
async fn list() {
    let (client, requests) = mock(
        200,
        Some("application/json"),
        r#"{"data":[{"created_at":"2023-12-31T23:59:59.999-05:30","currency":"sample","id":"plan_version_id_2","is_draft":true,"version":-2147483648}],"pagination_meta":{"page":-123456789,"per_page":-123456789,"total_items":-9007199254740993,"total_pages":123456789}}"#,
    );
    client
        .plans()
        .versions()
        .list("plan_id", None)
        .await
        .unwrap();
    assert_eq!(
        *requests.lock().unwrap(),
        ["GET /api/v1/plans/plan_id/versions"]
    );
}
