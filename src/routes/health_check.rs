use axum::response::IntoResponse;
use axum::http::StatusCode;

pub async fn health_check_handler() -> impl IntoResponse {
    let health = true;

    if health {
        Ok("Healthy!".to_string())
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Error!"))
    }
}
