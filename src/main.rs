mod auth;
mod cookies;
mod error;
mod layout;
mod pages;

use axum::Router;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!().run(&db).await?;

    let state = crate::AppState { db };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = Router::new()
        .route("/", axum::routing::get(pages::hello))
        .route("/time", axum::routing::get(pages::time))
        .route("/signup", axum::routing::get(auth::signup))
        .route("/signup", axum::routing::post(auth::signup_post))
        .route("/login", axum::routing::get(auth::login))
        .route("/login", axum::routing::post(auth::login_post))
        .route("/dashboard", axum::routing::get(pages::dashboard))
        .route("/logout", axum::routing::post(auth::logout_post))
        .nest_service("/static", ServeDir::new("src/static"))
        .with_state(state);

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}
