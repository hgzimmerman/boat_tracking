use anyhow::Error;
use axum::routing::get;
use axum::Router;
use boat_tracking::db::state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn_string = "db.sql";

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(std::io::stdout)
        .with_filter(tracing::level_filters::LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .init();

    let state = AppState::new(conn_string);

    let app = Router::new()
        // HTMX + Maud routes
        .merge(boat_tracking::handlers::create_router())
        // CSV export routes
        .route(
            "/uses_export.csv",
            get(boat_tracking::api::export_uses_csv_handler),
        )
        .route(
            "/boats_export.csv",
            get(boat_tracking::api::export_boats_csv_handler),
        )
        // Serve static files from public/
        .fallback_service(tower_http::services::ServeDir::new("public"))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("running at http://{addr}");

    #[cfg(feature = "tauri")]
    {
        // Spawn Axum server in background, launch Tauri window
        tokio::spawn(async move {
            let tcp_listener = TcpListener::bind(addr).await.unwrap();
            axum::serve(tcp_listener, app.into_make_service())
                .await
                .unwrap();
        });

        tauri::Builder::default()
            .run(tauri::generate_context!())
            .expect("error running tauri application");
    }

    #[cfg(not(feature = "tauri"))]
    {
        // Run Axum server directly
        let tcp_listener = TcpListener::bind(addr).await?;
        axum::serve(tcp_listener, app.into_make_service()).await?;
    }

    Ok(())
}
