pub fn layout(title: &str, content: maud::Markup, username: Option<&str>) -> maud::Markup {
    maud::html! {
        html {
            head {
                title { (title) }
                script src="/static/htmx.min.js" {}
                link rel="stylesheet" href="/static/style.css"{}
            }
            body {
                header {
                    @if let Some(name) = username {
                        span { "Hello " (name) }
                        form action = "/logout"
                        method = "post" {
                            button type = "submit" class="btn btn-ghost" {"Logout"}
                        }
                    } @else {
                        a href = "/login" class="btn" { "Login" }
                        " "
                        a href = "/signup" class="btn" {"Sign up"}
                    }
                }
                (content)
            }
        }
    }
}
