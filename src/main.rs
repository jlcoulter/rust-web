use rust_web_template::AppState;

use std::str::FromStr;

use axum_extra::extract::cookie::Key;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("rust_web=debug".parse().unwrap()),
        )
        .init();

    let connection_string = "sqlite://data.db";
    let options = SqliteConnectOptions::from_str(connection_string)?.create_if_missing(true);
    let db = SqlitePool::connect_with(options).await?;
    sqlx::migrate!().run(&db).await?;

    let key = Key::generate();
    let state = AppState { db, key };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, rust_web_template::app(state)).await?;

    Ok(())
}
