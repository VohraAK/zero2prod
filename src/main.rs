use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    routing::get,
};

use std::net::SocketAddr;

const PORT: usize = 3000;
const HOST: &str = "0.0.0.0";

// LATER: make an error enum covering all the things that can go wrong, then impl IntoResponse

// handlers (temp setup)
pub async fn root(path: Option<Path<String>>) -> String {
    return match path {
        Some(Path(name)) => format!("Hello, {}!", name),
        None => "Hello, World!".to_string(),
    };
}

pub async fn health_check_handler() -> Result<String, (StatusCode, &'static str)> {
    let health = true;

    if health {
        Ok("Healthy!".to_string())
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "Error!"))
    }
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = format!("{}:{}", HOST, PORT)
        .parse()
        .expect("Invalid address format!");

    let app = Router::new()
        .route("/", get(root))
        .route("/{name}", get(root))
        .route("/health_check", get(health_check_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening at port {PORT}...");
    axum::serve(listener, app).await.unwrap();
}
