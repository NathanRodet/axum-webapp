use axum_webapp::server::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

// This is the main function
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL").to_owned();
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();

    run(database_uri, jwt_secret).await;
}
