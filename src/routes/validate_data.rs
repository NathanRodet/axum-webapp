use axum::Json;
use serde::{Deserialize, Serialize};

// use axum::{
//     async_trait,
//     extract::FromRequest,
//     http::{self, Request, StatusCode},
//     Json, RequestExt,
// };

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthRequest {
    username: String,
    password: String,
}

pub async fn validate_user(Json(user): Json<AuthRequest>) {
    dbg!({ user.username }, { user.password });
}

// #[async_trait]
// impl<S, B> FromRequest<S, B> for AuthRequest
// where
//     // these bounds are required by `async_trait`
//     B: Send + 'static,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let user = req.extract::<Json<AuthRequest>>().await;
//     }
// }
// https://www.youtube.com/watch?v=MCctt60aeNk&list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS&index=25&ab_channel=BrooksBuilds

pub async fn custom_json_extractor() {}
