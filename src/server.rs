use crate::router::create_routes;
use axum_macros::FromRef;
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_conn: DatabaseConnection,
    pub(crate) jwt_secret: String,
}

pub async fn run(database_uri: String, jwt_secret: String) {
    let database_conn = Database::connect(database_uri).await.unwrap();

    let app_state = AppState {
        database_conn,
        jwt_secret,
    };

    let app = create_routes(app_state);

    // Start the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.await.into_make_service())
        .await
        .unwrap();
}
