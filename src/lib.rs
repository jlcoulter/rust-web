pub mod auth;
pub mod cookies;
pub mod error;
pub mod layout;
pub mod models;
pub mod pages;

use axum::Router;
use axum_extra::extract::cookie::Key;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub key: Key,
}

impl AppState {
    pub fn db(&self) -> &SqlitePool {
        &self.db
    }
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", axum::routing::get(pages::hello))
        .route("/time", axum::routing::get(pages::time))
        .route("/signup", axum::routing::get(auth::signup))
        .route("/signup", axum::routing::post(auth::signup_post))
        .route("/login", axum::routing::get(auth::login))
        .route("/login", axum::routing::post(auth::login_post))
        .route("/dashboard", axum::routing::get(pages::dashboard))
        .route("/logout", axum::routing::post(auth::logout_post))
        .nest_service("/static", ServeDir::new("src/static"))
        .fallback(pages::not_found)
        .with_state(state)
}
