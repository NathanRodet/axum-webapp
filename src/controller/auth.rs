
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

pub struct AuthResponse {
    pub token: String,
}

pub async fn authentication() -> std::io::Result<i32, futures::future::err> {
    todo!()
}