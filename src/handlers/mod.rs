// HTTP request handlers
pub mod issues;
pub mod boats;
pub mod batches;

use axum::{response::Html, routing::get, Router};
use axum_htmx::HxRequest;
use maud::Markup;
use crate::{templates, ui::state::AppState};

/// Helper to conditionally wrap content in full page layout
/// Uses axum-htmx HxRequest extractor to detect HTMX requests
pub fn maybe_page(title: &str, content: Markup, HxRequest(is_htmx): HxRequest) -> Html<String> {
    if is_htmx {
        // Return just the content for HTMX requests
        Html(content.into_string())
    } else {
        // Return full page for direct browser requests
        Html(templates::layout::page(title, content).into_string())
    }
}

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
    Router::new()
        // Test routes
        .route("/test", get(test_page_handler))
        .route("/test/htmx-response", get(htmx_test_response_handler))
        // Issues routes
        .route("/issues", get(issues::issue_list_handler).post(issues::create_issue_handler))
        .route("/issues/new", get(issues::new_issue_handler))
        .route("/issues/{id}/resolve", axum::routing::post(issues::resolve_issue_handler))
        .route("/issues/{id}/unresolve", axum::routing::post(issues::unresolve_issue_handler))
        // Boats routes
        .route("/boats", get(boats::boat_list_handler).post(boats::create_boat_handler))
        .route("/boats/new", get(boats::new_boat_handler))
        .route("/boats/{id}", get(boats::boat_detail_handler).post(boats::update_boat_handler))
        .route("/boats/{id}/edit", get(boats::edit_boat_handler))
        // Chart routes
        .route("/boats/{id}/chart/daily", get(boats::daily_chart_handler))
        .route("/boats/{id}/chart/monthly", get(boats::monthly_chart_handler))
        // Batch routes
        .route("/batches", get(batches::batch_list_handler).post(batches::create_batch_handler))
        .route("/batches/new", get(batches::new_batch_handler))
        .route("/batches/{id}", get(batches::batch_detail_handler))
        // Batch API routes (HTMX endpoints)
        .route("/api/batches/boats", get(batches::list_boats_handler))
        .route("/api/batches/search", axum::routing::post(batches::search_boats_handler))
        .route("/api/batches/session/add/{id}", axum::routing::post(batches::add_boat_to_session_handler))
        .route("/api/batches/session/remove/{id}", axum::routing::post(batches::remove_boat_from_session_handler))
}
