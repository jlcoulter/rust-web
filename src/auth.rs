use crate::AppState;
use crate::cookies::login_cookie;
use crate::cookies::redirect_with_cookie;
use crate::error::AppError;
use crate::layout::layout;
use crate::models::user;

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
    match user::get_password_hash(&state.db, &form.username).await? {
        Some(stored_hash) => {
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
        None => Ok(layout(
            "Login",
            maud::html! {
                p {"Invalid username or password"}
                a hred = "/login" {"Try again"}
            },
            None,
        )
        .into_response()),
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
    if form.username.trim().is_empty() {
        return Err(AppError::BadRequest("Username is required".to_string()));
    }
    if form.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    let hash = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST)?;

    match user::create_user(&state.db, &form.username.trim(), &hash).await {
        Ok(()) => Ok(redirect_with_cookie(
            "/dashboard",
            login_cookie(&form.username),
        )),
        Err(AppError::DuplicateUser) => Ok(layout(
            "Signup",
            maud::html! {
                p { "Username already taken" }
                a href="/signup" { "Try again" }
            },
            None,
        )
        .into_response()),
        Err(AppError::BadRequest(msg)) => Ok(layout(
            "Signup",
            maud::html! {
                p { (msg) }
                a href="/signup" { "Try again" }
            },
            None,
        )
        .into_response()),
        Err(e) => Err(e),
    }
}
