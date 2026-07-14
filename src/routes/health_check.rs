use axum::http::StatusCode;
use axum::response::IntoResponse;

#[tracing::instrument(name = "Health check")]
pub async fn health_check_handler() -> impl IntoResponse {
    let health = true;

    if health {
        tracing::info!("Health check passed");
        Ok("Healthy!".to_string())
    } else {
        tracing::error!("Health check failed");
        Err((StatusCode::SERVICE_UNAVAILABLE, "Error!"))
    }
}
