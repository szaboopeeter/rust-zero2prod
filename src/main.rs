//! src/main.rs
use std::net::TcpListener;
use sqlx::{PgPool};
use zero2prod::{startup::run, configuration::get_configuration};
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // if RUST_LOG env variable is not set, the default is info-or-above
    // eg: `RUST_LOG=debug cargo run`
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connecto to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
