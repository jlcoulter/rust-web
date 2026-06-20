use crate::auth::LoggedInUser;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;

use crate::AppState;
use axum::extract::FromRequestParts;
use axum::extract::OptionalFromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;

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

pub fn redirect_with_cookie(uri: &str, cookie: Cookie) -> axum::response::Response {
    let mut resp = axum::response::Redirect::to(uri).into_response();
    resp.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );
    resp
}

pub fn login_cookie(username: &str) -> Cookie<'static> {
    Cookie::build(("username", username.to_string()))
        .http_only(true)
        .path("/")
        .build()
}
