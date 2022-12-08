use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

// This is the JSON body
#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct UserRequest {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

// This is the route handler
pub async fn validate_user(Json(user): Json<UserRequest>) {
    dbg!({ user.username }, { user.password });
}

// This is the custom extractor
#[async_trait]
impl<S, B> FromRequest<S, B> for UserRequest
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    // This is the method that is called when the request is received
    async fn from_request(request: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the JSON body from the request
        let Json(user) = request
            .extract::<Json<UserRequest>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;
        // Validate the JSON body
        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
        // Return the JSON body
        Ok(user)
    }
}
// This is the route handler
pub async fn custom_json_extractor(user: UserRequest) {
    dbg!(user);
}
