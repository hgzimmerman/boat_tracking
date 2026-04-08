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
            let exe_dir = std::env::current_exe()
                .expect("should resolve current exe")
                .parent()
                .expect("executable must have a parent directory")
                .to_path_buf();
            // Check in order: next to exe, Tauri resource dir, CWD
            let public_dir = [
                exe_dir.join("public"),
                exe_dir.join("resources").join("public"),
            ]
            .into_iter()
            .find(|p| p.is_dir())
            .unwrap_or_else(|| std::path::PathBuf::from("public"));
            tracing::info!(?public_dir, "Serving static files");
            tower_http::services::ServeDir::new(public_dir)
        })
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
