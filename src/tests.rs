use uuid::Uuid;
use warp::test;

#[tokio::test]
async fn test_category_creation() {
    let value = test::request()
        .method("POST")
        .path("/api/v1/categories")
        .json(&serde_json::json!({
            "name": "Test Category",
        }))
        .reply(&crate::routes::get_routes(
            crate::db::create_pool().unwrap(),
        ))
        .await;

    assert_eq!(value.status(), 201);
}

#[tokio::test]
async fn test_category_wrong_creation() {
    let value = test::request()
        .method("POST")
        .path("/api/v1/categories")
        .json(&serde_json::json!({
            "name": "4",
        }))
        .reply(&crate::routes::get_routes(
            crate::db::create_pool().unwrap(),
        ))
        .await;

    assert_eq!(value.status(), 400);
}

#[tokio::test]
async fn test_category_query() {
    let value = test::request()
        .method("GET")
        .path("/api/v1/categories")
        .reply(&crate::routes::get_routes(
            crate::db::create_pool().unwrap(),
        ))
        .await;

    assert_eq!(value.status(), 200);
}

#[tokio::test]
async fn test_habit_query() {
    let value = test::request()
        .method("GET")
        .path("/api/v1/habits/user/jfadsfdsf")
        .reply(&crate::routes::get_routes(
            crate::db::create_pool().unwrap(),
        ))
        .await;

    assert_eq!(value.status(), 200);
}
