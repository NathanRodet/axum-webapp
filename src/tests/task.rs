#[cfg(test)]
mod tests {
    use crate::tests::app::app_test;
    use axum::body::Body;
    use axum::http;
    use axum::http::Request;
    use axum::http::StatusCode;
    use serde_json::json;
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn create_task_test() {
        let app = app_test().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/task")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!({"title": "test title", "description": "test description", "priority": "qos"})).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
