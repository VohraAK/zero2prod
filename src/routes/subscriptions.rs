use axum::extract::{Form, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeFormData {
    #[validate(length(min = 1, message = "Name cannot be empty!"))]
    pub name: String,

    #[validate(email(message = "Email cannot be empty!"))]
    pub email: String,
}

pub async fn subscribe_handler(
    State(connection): State<PgPool>,
    Form(form): Form<SubscribeFormData>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&connection)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
