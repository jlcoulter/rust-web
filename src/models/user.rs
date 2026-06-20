use crate::error::AppError;
use sqlx::SqlitePool;

pub async fn get_password_hash(pool: &SqlitePool, username: &str) -> Result<String, AppError> {
    let hash =
        sqlx::query_scalar::<_, String>("SELECT password_hash FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(pool)
            .await?;
    Ok(hash)
}

pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    password_hash: &str,
) -> Result<(), AppError> {
    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(password_hash)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.to_string().contains("UNIQUE constraint") {
                Err(AppError::DuplicateUser)
            } else {
                Err(AppError::Internal(e.into()))
            }
        }
    }
}