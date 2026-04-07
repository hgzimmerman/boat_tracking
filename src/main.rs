#![cfg_attr(feature = "tauri", windows_subsystem = "windows")]

use anyhow::Error;
use rolling_file::{BasicRollingFileAppender, RollingConditionBasic};
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn_string = std::env::var("DATABASE_URL").unwrap_or_else(|_| "db.sql".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(std::io::stdout)
        .with_filter(tracing::level_filters::LevelFilter::DEBUG);

    let log_dir = directories::ProjectDirs::from("", "", "boat_tracking")
        .map(|dirs| dirs.state_dir().unwrap_or_else(|| dirs.data_local_dir()).join("logs"))
        .unwrap_or_else(|| std::path::PathBuf::from("logs"));
    std::fs::create_dir_all(&log_dir)?;

    let log_file = BasicRollingFileAppender::new(
        log_dir.join("boat_tracking.log"),
        RollingConditionBasic::new().max_size(10 * 1024 * 1024), // 10 MiB
        5, // keep up to 5 rotated files
    )?;
    let (non_blocking, _guard) = tracing_appender::non_blocking(log_file);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_filter(tracing::level_filters::LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    let app = boat_tracking::build_router(&conn_string);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
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
            .setup(move |app| {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    if port != 3000 {
                        let url: tauri::Url = format!("http://localhost:{port}").parse().unwrap();
                        window.navigate(url)?;
                        window.set_fullscreen(false)?;
                    }
                }
                Ok(())
            })
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
