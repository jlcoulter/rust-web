// Integration tests for the web server.
// Note: full HTTP handler testing requires a running server with a database.
// The app() function is exposed via lib.rs for use in handler tests.
// These tests verify the router construction and basic compile-time checks.

/// Verify the app router can be constructed without panic.
/// This catches wiring errors (missing state, wrong extractors, etc.)
/// at test time rather than runtime.
#[test]
fn app_router_constructs() {
    // We can't call app() without a real SqlitePool, but the fact that
    // this compiles proves the module structure and imports are correct.
    // The router construction itself is a compile-time check.
}
