// HTTP request handlers
pub mod issues;
pub mod boats;
pub mod batches;
pub mod scenarios;

use axum::{response::{Html, Redirect}, routing::get, Router};
use axum_htmx::HxRequest;
use maud::Markup;
use serde::Deserialize;
use crate::{templates, db::state::AppState};

/// Query parameters for paginated list endpoints.
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

fn default_page() -> u64 { 1 }
fn default_per_page() -> u64 { 50 }

impl PaginationParams {
    /// Computes the SQL OFFSET from the 1-indexed page number.
    pub fn offset(&self) -> u64 {
        self.page.saturating_sub(1) * self.per_page
    }

    /// Computes pagination metadata from the total count of items.
    pub fn metadata(&self, total_count: u64) -> PaginationMeta {
        let total_pages = total_count.div_ceil(self.per_page).max(1);
        PaginationMeta {
            current_page: self.page,
            per_page: self.per_page,
            total_count,
            total_pages,
        }
    }
}

/// Pagination metadata for templates to render controls.
pub struct PaginationMeta {
    pub current_page: u64,
    pub per_page: u64,
    pub total_count: u64,
    pub total_pages: u64,
}

impl PaginationMeta {
    pub fn has_previous(&self) -> bool { self.current_page > 1 }
    pub fn has_next(&self) -> bool { self.current_page < self.total_pages }
}

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
        .route("/", get(|| async { Redirect::permanent("/batches") }))
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
        .route("/boats/{id}/issues", get(boats::boat_issues_handler))
        .route("/boats/{id}/edit", get(boats::edit_boat_handler))
        // Chart routes
        .route("/boats/{id}/chart/daily", get(boats::daily_chart_handler))
        .route("/boats/{id}/chart/monthly", get(boats::monthly_chart_handler))
        // Batch routes
        .route("/batches", get(batches::batch_list_handler).post(batches::create_batch_handler))
        .route("/batches/new", get(batches::new_batch_handler))
        .route("/batches/{id}", get(batches::batch_detail_handler))
        // Batch API routes
        .route("/api/batches/{id}/boats", get(batches::batch_boats_preview_handler))
        // Scenario routes
        .route("/scenarios", get(scenarios::scenario_list_handler).post(scenarios::create_scenario_handler))
        .route("/scenarios/new", get(scenarios::new_scenario_handler))
        .route("/scenarios/{id}/edit", get(scenarios::edit_scenario_handler))
        .route("/scenarios/{id}", axum::routing::post(scenarios::update_scenario_handler))
        // Batch API routes (HTMX endpoints)
        .route("/api/batches/boats", get(batches::list_boats_handler))
        .route("/api/batches/search", axum::routing::post(batches::search_boats_handler))
        .route("/api/batches/session/add/{id}", axum::routing::post(batches::add_boat_to_session_handler))
        .route("/api/batches/session/remove/{id}", axum::routing::post(batches::remove_boat_from_session_handler))
}
