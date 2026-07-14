use crate::routes::{health_check_handler, subscribe_handler};
use axum::response::IntoResponse;
use axum::serve::Serve;
use axum::{
    Router,
    extract::Path,
    routing::{get, post},
};
use sqlx::PgPool;
use tokio::net::TcpListener;

async fn root(path: Option<Path<String>>) -> impl IntoResponse {
    match path {
        Some(Path(name)) => format!("Hello, {}!", name),
        None => "Hello, World!".to_string(),
    }
}

pub fn run(
    listener: std::net::TcpListener,
    connection: PgPool,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(listener)?;

    let app = Router::new()
        .route("/", get(root))
        .route("/{name}", get(root))
        .route("/health_check", get(health_check_handler))
        .route("/subscriptions", post(subscribe_handler))
        .with_state(connection);

    Ok(axum::serve(listener, app))
}
