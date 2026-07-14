use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health_check_handler() -> impl IntoResponse {
    let health = true;

    if health {
        Ok("Healthy!".to_string())
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Error!"))
    }
}
