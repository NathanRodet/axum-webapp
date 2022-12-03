use crate::{router::create_routes, database::database};

pub async fn run(database_uri: &str) {
    let app = create_routes();
    let database_conn = database(database_uri);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.await.into_make_service())
        .await
        .unwrap();
}