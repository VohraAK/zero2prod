use tokio::net::TcpListener;

pub mod types;

use crate::types::SubscribeFormData;

use axum::response::IntoResponse;
use axum::serve::Serve;
use axum::{
    Router,
    extract::{Path, Form},
    http::StatusCode,
    routing::{get, post},
};


// LATER: make an error enum covering all the things that can go wrong, then impl IntoResponse

// handlers (temp setup)
async fn root(path: Option<Path<String>>) -> impl IntoResponse {
    match path {
        Some(Path(name)) => format!("Hello, {}!", name),
        None => "Hello, World!".to_string(),
    }
}

async fn health_check_handler() -> impl IntoResponse {
    let health = true;

    if health {
        Ok("Healthy!".to_string())
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Error!"))
    }
}

async fn subscribe_handler(Form(_form): Form<SubscribeFormData>) -> impl IntoResponse {
    // if let Err(errors) = form.validate() {
    //     return (StatusCode::BAD_REQUEST, errors.to_string()).into_response();
    // }

    // return StatusCode::OK.into_response();

    StatusCode::OK
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

    println!("Listening to port 3000");
    
    Ok(axum::serve(listener, app))
}
