use crate::AppState;

use axum::extract::Form;
use axum::extract::FromRequestParts;
use axum::extract::OptionalFromRequestParts;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::Deserialize;

pub async fn hello(
    State(_state): State<AppState>,
    user: Option<LoggedInUser>,
) -> impl axum::response::IntoResponse {
    layout(
        "Home",
        maud::html! {
        h1 {"Hello"}
        div id="clock" hx-get="/time" hx-trigger="every 1s" {
            "Loading..."
        }
                    },
        user.as_ref().map(|u| u.0.as_str()),
    )
}

fn layout(title: &str, content: maud::Markup, username: Option<&str>) -> maud::Markup {
    maud::html! {
        html {
            head {
                title { (title) }
                script src="/static/htmx.min.js" {}
            }
            body {
                header {
                    @if let Some(name) = username {
                        span { "Hello " (name) }
                        form action = "/logout"
                        method = "post" {
                            button type = "submit" {"Logout"}
                        }
                    } @else {
                        a href = "/login" { "Login" }
                        " "
                        a href = "/signup" {"Sign up"}
                    }
                }
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

impl OptionalFromRequestParts<AppState> for LoggedInUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(jar
            .get("username")
            .map(|c| LoggedInUser(c.value().to_string())))
    }
}

pub async fn dashboard(user: LoggedInUser) -> impl IntoResponse {
    layout(
        "Dashboard",
        maud::html! {
            h1 { "Welcome " (user.0) }
        },
        Some(&user.0),
    )
}

pub async fn time(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    maud::html! { p { "Time: " (chrono::Local::now().format("%H:%M:%S")) } }
}

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
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

fn redirect_with_cookie(uri: &str, cookie: Cookie) -> axum::response::Response {
    let mut resp = axum::response::Redirect::to(uri).into_response();
    resp.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );
    resp
}

fn login_cookie(username: &str) -> Cookie<'static> {
    Cookie::build(("username", username.to_string()))
        .http_only(true)
        .path("/")
        .build()
}
