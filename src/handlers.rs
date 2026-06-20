use crate::AppState;

use axum::extract::Form;
use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::Deserialize;

pub async fn hello(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    layout(
        "Home",
        maud::html! {
        h1 {"Hello"}
        div id="clock" hx-get="/time" hx-trigger="every 1s" {
            "Loading..."
        }
                    },
    )
}

fn layout(title: &str, content: maud::Markup) -> maud::Markup {
    maud::html! {
        html {
            head {
                title { (title) }
                script src="/static/htmx.min.js" {}
            }
            body {
                (content)
            }
        }
    }
}

pub struct LoggedInUser(pub String);

impl FromRequestParts<AppState> for LoggedInUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, _state)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        jar.get("username")
            .map(|c| LoggedInUser(c.value().to_string()))
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

pub async fn dashboard(user: LoggedInUser) -> impl IntoResponse {
    layout(
        "Dashboard",
        maud::html! {
            h1 { "Welcome " (user.0) }
        },
    )
}

pub async fn time(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    maud::html! { p { "Time: " (chrono::Local::now().format("%H:%M:%S")) } }
}

pub async fn login(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    maud::html! {
        form action = "/login" method = "post" {
                        label { "Username" input type = "text" name = "username"; }
                        label { "Password" input type = "password" name = "password";}
                        button type="submit" { "Login" }
                }
    }
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login_post(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<axum::response::Response, StatusCode> {
    let result =
        sqlx::query_scalar::<_, String>("SELECT password_hash FROM users WHERE username = ?")
            .bind(&form.username)
            .fetch_one(&state.db)
            .await;
    let stored_hash = result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let valid = bcrypt::verify(&form.password, &stored_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if valid {
        let cookie: Cookie = Cookie::build(("username", form.username.clone()))
            .http_only(true)
            .path("/")
            .build();
        let mut resp = axum::response::Redirect::to("/").into_response();
        resp.headers_mut().insert(
            axum::http::header::SET_COOKIE,
            cookie.to_string().parse().unwrap(),
        );

        Ok(resp)
    } else {
        Ok(layout(
            "Login",
            maud::html! {
                p { "Invalid username or password" }
                a href = "/login" {"Try again"}
            },
        )
        .into_response())
    }
}

pub async fn signup(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    maud::html! {
        form action = "/signup" method = "post" {
                        label { "Username" input type = "text" name = "username"; }
                        label { "Password" input type = "password" name = "password";}
                        button type="submit" { "Sign up" }
                }
    }
}

#[derive(Deserialize)]
pub struct SignupForm {
    username: String,
    password: String,
}

pub async fn signup_post(
    State(state): State<AppState>,
    Form(form): Form<SignupForm>,
) -> Result<axum::response::Response, StatusCode> {
    let hash = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?,?)")
        .bind(&form.username)
        .bind(&hash)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Ok(axum::response::Redirect::to("/").into_response()),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("UNIQUE constraint") {
                Ok(layout(
                    "Signup",
                    maud::html! {
                        p { "Username already taken" }
                        a href="/signup" { "Try again" }
                    },
                )
                .into_response())
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
