use crate::layout::error_box;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

pub enum AppError {
    Internal(anyhow::Error),
    BadRequest(String),
    Unauthorized(String),
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
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, error_box(&msg)).into_response(),
            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, error_box(&msg)).into_response()
            }
            AppError::DuplicateUser => {
                (StatusCode::CONFLICT, error_box("Username already taken")).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_request_status_code() {
        let err = AppError::BadRequest("test error".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn unauthorized_status_code() {
        let err = AppError::Unauthorized("no access".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn duplicate_user_status_code() {
        let err = AppError::DuplicateUser;
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}
