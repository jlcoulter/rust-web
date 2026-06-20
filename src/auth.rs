use crate::AppState;
use crate::cookies::login_cookie;
use crate::cookies::redirect_with_cookie;
use crate::error::AppError;
use crate::layout::layout;

use axum::extract::Form;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub struct LoggedInUser(pub String);

pub async fn login(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    layout(
        "Login",
        maud::html! {
            form action = "/login" method = "post" {
                            label { "Username" input type = "text" name = "username"; }
                            label { "Password" input type = "password" name = "password";}
                            button type="submit" { "Login" }
                    }
        },
        None,
    )
}

pub async fn login_post(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<axum::response::Response, AppError> {
    let result =
        sqlx::query_scalar::<_, String>("SELECT password_hash FROM users WHERE username = ?")
            .bind(&form.username)
            .fetch_one(&state.db)
            .await;
    let stored_hash = result?;

    let valid = bcrypt::verify(&form.password, &stored_hash)?;

    if valid {
        Ok(redirect_with_cookie(
            "/dashboard",
            login_cookie(&form.username),
        ))
    } else {
        Ok(layout(
            "Login",
            maud::html! {
                p { "Invalid username or password" }
                a href = "/login" {"Try again"}
            },
            None,
        )
        .into_response())
    }
}

pub async fn logout_post(State(_state): State<AppState>) -> impl IntoResponse {
    let cookie = Cookie::build(("username", ""))
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();
    let mut resp = axum::response::Redirect::to("/").into_response();
    resp.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );
    resp
}

pub async fn signup(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    layout(
        "Sign up",
        maud::html! {
            form action = "/signup" method = "post" {
                            label { "Username" input type = "text" name = "username"; }
                            label { "Password" input type = "password" name = "password";}
                            button type="submit" { "Sign up" }
                    }
        },
        None,
    )
}

#[derive(Deserialize)]
pub struct SignupForm {
    username: String,
    password: String,
}

pub async fn signup_post(
    State(state): State<AppState>,
    Form(form): Form<SignupForm>,
) -> Result<axum::response::Response, AppError> {
    let hash = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST)?;

    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?,?)")
        .bind(&form.username)
        .bind(&hash)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(redirect_with_cookie(
            "/dashboard",
            login_cookie(&form.username),
        )),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("UNIQUE constraint") {
                Ok(layout(
                    "Signup",
                    maud::html! {
                        p { "Username already taken" }
                        a href="/signup" { "Try again" }
                    },
                    None,
                )
                .into_response())
            } else {
                Err(AppError::Internal(e.into()))
            }
        }
    }
}
