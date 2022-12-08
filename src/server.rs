use crate::router::create_routes;
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database_conn = Database::connect(database_uri).await.unwrap();
    let app = create_routes(database_conn);

    // Start the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.await.into_make_service())
        .await
        .unwrap();
}
