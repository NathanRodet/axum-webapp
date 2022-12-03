use sea_orm::{Database, DatabaseConnection};

pub async fn database(database_uri: &str) -> DatabaseConnection {
    Database::connect(database_uri).await.unwrap()
}
