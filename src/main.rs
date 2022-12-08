use axum_webapp::server::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

// This is the main function
#[tokio::main]
async fn main() {
    // Load the environment variables
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");

    // Run the server
    run(database_uri).await;
}