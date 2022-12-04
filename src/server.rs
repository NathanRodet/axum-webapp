use crate::router::create_routes;
use sea_orm::{Database, DatabaseConnection};

// Create database connection
async fn create_database_conn(database_uri: &str) -> DatabaseConnection {
    Database::connect(database_uri).await.unwrap()
}

pub async fn run(database_uri: &str) {
    let app = create_routes();
    let database_conn = create_database_conn(database_uri);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.await.into_make_service())
        .await
        .unwrap();
}
