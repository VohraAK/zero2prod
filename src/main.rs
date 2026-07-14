use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read config!");
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres!");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&address)?;

    println!("Listening to port {}", config.application_port);

    run(listener, connection)?.await
}
