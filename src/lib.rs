use axum::routing::get;
use axum::Router;
use db::state::AppState;

pub mod api;
pub mod db;
mod schema;
pub mod templates;
pub mod handlers;

/// Builds the full application router with the given database connection string.
pub fn build_router(conn_string: &str) -> Router {
    let state = AppState::new(conn_string);

    Router::new()
        .merge(handlers::create_router())
        .route(
            "/uses_export.csv",
            get(api::export_uses_csv_handler),
        )
        .route(
            "/boats_export.csv",
            get(api::export_boats_csv_handler),
        )
        .fallback_service({
            let exe_relative = std::env::current_exe()
                .expect("should resolve current exe")
                .parent()
                .expect("executable must have a parent directory")
                .join("public");
            let public_dir = if exe_relative.is_dir() {
                exe_relative
            } else {
                std::path::PathBuf::from("public")
            };
            tower_http::services::ServeDir::new(public_dir)
        })
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
