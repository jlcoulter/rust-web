# rust-web Learning Path

A bitesized, step-by-step guide to building a web server with axum + htmx + maud + sqlx + bcrypt.

## Completed Steps

- [x] **1. Hello server** — axum + tokio, basic route returning plain text
- [x] **2. maud templates** — switch handler to `maud::html!`, add axum feature for `IntoResponse`
- [x] **3. Layout wrapper** — `layout()` function wrapping content in full HTML doc with `<head>`, `<title>`, `<body>`
- [x] **4. htmx script** — added `<script src="/static/htmx.min.js">`, served via `tower-http::ServeDir`
- [x] **5. Live clock** — `/time` fragment endpoint, `hx-get` + `hx-trigger="every 1s"` polling
- [x] **6. Shared state** — `AppState { db: SqlitePool }` with `#[derive(Clone)]`, `.with_state(state)`
- [x] **7. Migrations** — `sqlx::migrate!()`, `migrations/0001_init.sql` with users table
- [x] **8. bcrypt dep** — added to Cargo.toml
- [ ] **9. Signup form (GET)** — `/signup` route renders HTML form with username + password fields

## Current Step

Start with step 9: add a GET `/signup` route that renders a form.

## Remaining Steps

- [ ] **10. Signup handler (POST)** — receives form data, hashes password with bcrypt, inserts into DB

## Remaining Steps

- [ ] **11. Login form + handler** — GET `/login`, POST `/login` with bcrypt verify
- [ ] **12. Sessions/cookies** — set a session cookie on login, add auth middleware or extractor
- [ ] **13. Protected route** — a page that only shows if logged in (e.g. a dashboard)
- [ ] **14. Logout** — clear session, redirect home
- [ ] **15. Styling** — add CSS (file or inline), make it look decent
- [ ] **16. Error handling** — show flash messages for bad login, duplicate username, etc.

## Architecture Notes

- **Stack**: axum 0.8, maud 0.27 (axum feature), htmx 2.0.4, sqlx 0.9 (sqlite), bcrypt 0.17, tower-http 0.7
- **Pattern**: `layout()` wraps full pages, fragment handlers return just the piece htmx swaps in
- **State**: `AppState { db: SqlitePool }` passed via `.with_state()`, handlers extract with `State(_state): State<AppState>`
- **Static files**: `/static/*` served from `src/static/` via `tower_http::services::ServeDir`
- **DB**: SQLite via `sqlite:app.db`, migrations in `migrations/`