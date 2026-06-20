use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AppError {
    Internal(anyhow::Error),
    BadRequest(String),
    DuplicateUser,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Internal(err.into())
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Internal(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Internal(err) => {
                tracing::error!(%err, "internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::DuplicateUser => {
                (StatusCode::CONFLICT, "Username already taken").into_response()
            }
        }
    }
}
