use axum::Json;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    username: String,
    password: String,
}


pub async fn validate_user(Json(user): Json<AuthRequest>)  {
    dbg!({user.username}, {user.password});
}