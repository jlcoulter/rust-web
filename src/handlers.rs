use crate::AppState;
use axum::extract::State;

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

pub async fn time(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    maud::html! { p { "Time: " (chrono::Local::now().format("%H:%M:%S")) } }
}