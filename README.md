# Rust Web Template

A GitHub template for Rust web applications — axum, htmx, maud, sqlx, and bcrypt with auth, cookies, and a clean project structure.

## Features

- **HTTP framework** with [axum](https://github.com/tokio-rs/axum) 0.8 + axum-extra (signed cookies)
- **HTML templates** with [maud](https://maud.lambda.xyz/) (axum feature, compile-time checked)
- **Database** with [sqlx](https://github.com/launchbadge/sqlx) 0.9 (SQLite, migrations)
- **Auth** with [bcrypt](https://docs.rs/bcrypt) — signup, login, logout, session cookies
- **Static assets** with [tower-http](https://github.com/tower-rs/tower-http) (ServeDir)
- **Structured logging** with [tracing](https://docs.rs/tracing) + [tracing-subscriber](https://docs.rs/tracing-subscriber)
- **Error handling** — custom `AppError` enum with `IntoResponse`
- **Tests** — unit tests in `src/`, integration tests in `tests/`
- **CI** via GitHub Actions (test, clippy, fmt, build on push/PR)
- **Docker** multi-stage build (scratch image, static binary)
- **Makefile** for common tasks

## Usage

1. Click **"Use this template"** on GitHub to create a new repo
2. Run the setup script:
   ```sh
   ./setup.sh myapp
   ```
3. Add your pages in `src/pages.rs` and routes in `src/main.rs`
4. Add models in `src/models/`

## Project Structure

```
.
├── src/
│   ├── main.rs            # App init, router, state
│   ├── lib.rs             # Re-exports for integration tests
│   ├── auth.rs            # Signup, login, logout handlers
│   ├── cookies.rs         # Cookie helpers, LoggedInUser extractor
│   ├── error.rs           # AppError enum + IntoResponse
│   ├── layout.rs          # HTML layout wrapper
│   ├── pages.rs           # Page handlers (home, dashboard, 404)
│   ├── models/
│   │   ├── mod.rs          # Module re-exports
│   │   └── user.rs         # User DB queries
│   └── static/
│       ├── style.css       # App styles
│       └── htmx.min.js     # htmx (local, no CDN)
├── migrations/
│   └── 0001_init.sql       # SQLite schema
├── tests/
│   └── integration_test.rs # Integration tests
├── .github/
│   └── workflows/
│       └── ci.yml          # Test + clippy + fmt
├── Cargo.toml
├── Dockerfile
├── Makefile
├── setup.sh
└── README.md
```

## Quick Start

```sh
# Run locally
make run

# Run tests
make test

# Build release binary
make build

# Build Docker image
make docker

# Lint
make lint
```

## Container Images

CI builds and pushes a container image to GHCR on every push to any branch.

```sh
# Pull the latest image
docker pull ghcr.io/<owner>/rust-web-template:latest

# Pull a specific commit
docker pull ghcr.io/<owner>/rust-web-template:<sha>

# Run
docker run -p 3000:3000 ghcr.io/<owner>/rust-web-template:latest
```

Replace `<owner>` with your GitHub username or org. Images are tagged with both `latest` and the commit SHA.

## Routes

| Method | Path       | Description         |
|--------|------------|---------------------|
| GET    | /          | Home (htmx clock)   |
| GET    | /time      | Time fragment       |
| GET    | /signup    | Signup form         |
| POST   | /signup    | Create account      |
| GET    | /login     | Login form          |
| POST   | /login     | Authenticate        |
| GET    | /dashboard | Protected dashboard |
| POST   | /logout    | Clear session       |
| ANY    | /*         | 404 fallback        |

## Environment

| Variable    | Default       | Description               |
|-------------|---------------|---------------------------|
| `RUST_LOG`  | `rust_web=debug` | Log level (tracing)    |
| `PORT`      | `3000`        | HTTP listen port          |

## Adding a Page

```rust
// In src/pages.rs
pub async fn mypage(user: LoggedInUser) -> impl IntoResponse {
    layout(
        "My Page",
        maud::html! { h2 { "Hello " (user.0) } },
        Some(&user),
    )
}
```

Then add a route in `src/main.rs`:
```rust
.route("/mypage", axum::routing::get(pages::mypage))
```

## License

MIT