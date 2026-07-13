use tokio::net::TcpListener;

use crate::routes::{health_check_handler, subscribe_handler};

use axum::response::IntoResponse;
use axum::serve::Serve;
use axum::{
    Router,
    extract::Path,
    routing::{get, post},
};

async fn root(path: Option<Path<String>>) -> impl IntoResponse {
    match path {
        Some(Path(name)) => format!("Hello, {}!", name),
        None => "Hello, World!".to_string(),
    }
}

pub fn run(
    listener: std::net::TcpListener,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(listener)?;

    let app = Router::new()
        .route("/", get(root))
        .route("/{name}", get(root))
        .route("/health_check", get(health_check_handler))
        .route("/subscriptions", post(subscribe_handler));

    Ok(axum::serve(listener, app))
}
