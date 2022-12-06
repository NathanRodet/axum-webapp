use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct UserRequest {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

pub async fn validate_user(Json(user): Json<UserRequest>) {
    dbg!({ user.username }, { user.password });
}

#[async_trait]
impl<S, B> FromRequest<S, B> for UserRequest
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<UserRequest>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}
//https://www.youtube.com/watch?v=5-b2dy4c6l4&list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS&index=26&ab_channel=BrooksBuilds

pub async fn custom_json_extractor(user: UserRequest) {
    dbg!(user);
}
