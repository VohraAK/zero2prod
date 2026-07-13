use axum::extract::Form;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeFormData {
    #[validate(length(min = 1, message = "Name cannot be empty!"))]
    pub name: String,

    #[validate(email(message = "Email cannot be empty!"))]
    pub email: String,
}

pub async fn subscribe_handler(Form(_form): Form<SubscribeFormData>) -> impl IntoResponse {
    // if let Err(errors) = form.validate() {
    //     return (StatusCode::BAD_REQUEST, errors.to_string()).into_response();
    // }

    // return StatusCode::OK.into_response();

    StatusCode::OK
}
