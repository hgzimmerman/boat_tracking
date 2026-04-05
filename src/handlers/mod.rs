// HTTP request handlers
// Handler modules will be added in later phases:
// - boats.rs
// - batches.rs
// - issues.rs

use axum::response::Html;
use crate::templates;

/// Test handler for proof-of-concept page
pub async fn test_page_handler() -> Html<String> {
    Html(templates::layout::test_page().into_string())
}

/// Test handler for HTMX response
pub async fn htmx_test_response_handler() -> Html<String> {
    Html(templates::layout::htmx_test_response().into_string())
}
