// HTTP request handlers
pub mod issues;
pub mod boats;
// Handler modules will be added in later phases:
// - batches.rs

use axum::{response::Html, routing::get, Router};
use crate::{templates, ui::state::AppState};

/// Test handler for proof-of-concept page
pub async fn test_page_handler() -> Html<String> {
    Html(templates::layout::test_page().into_string())
}

/// Test handler for HTMX response
pub async fn htmx_test_response_handler() -> Html<String> {
    Html(templates::layout::htmx_test_response().into_string())
}

/// Create router with all HTMX + Maud routes
pub fn create_router() -> Router<AppState> {
    use axum::routing::post;

    Router::new()
        // Test routes
        .route("/test", get(test_page_handler))
        .route("/test/htmx-response", get(htmx_test_response_handler))
        // Issues routes
        .route("/issues", get(issues::issue_list_handler))
        .route("/issues/new", get(issues::new_issue_handler))
        // Boats routes
        .route("/boats", get(boats::boat_list_handler).post(boats::create_boat_handler))
        .route("/boats/new", get(boats::new_boat_handler))
        .route("/boats/:id/edit", get(boats::edit_boat_handler))
        .route("/boats/:id", post(boats::update_boat_handler))
}
