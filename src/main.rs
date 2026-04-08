#![cfg_attr(feature = "tauri", windows_subsystem = "windows")]

use anyhow::Error;
use rolling_file::{BasicRollingFileAppender, RollingConditionBasic};
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logging()?;

    let conn_string = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        #[cfg(all(feature = "tauri", target_os = "windows"))]
        {
            // On Windows, installed apps can't write next to the executable,
            // so put the database in %LOCALAPPDATA%.
            directories::ProjectDirs::from("", "", "boat_tracking")
                .map(|dirs| {
                    let data_dir = dirs.data_local_dir();
                    std::fs::create_dir_all(data_dir).expect("should create data directory");
                    data_dir.join("db.sql").to_string_lossy().into_owned()
                })
                .unwrap_or_else(|| "db.sql".to_string())
        }
        #[cfg(not(all(feature = "tauri", target_os = "windows")))]
        {
            "db.sql".to_string()
        }
    });
    tracing::info!(?conn_string, "Using database");
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    tracing::debug!(%port); 

    let app = boat_tracking::build_router(&conn_string);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    println!("running at http://{addr}");
    tracing::debug!(?addr);

    #[cfg(feature = "tauri")]
    {
        tracing::debug!("Starting Server");
        // Spawn Axum server in background, launch Tauri window
        tokio::spawn(async move {
            tracing::info!(?addr, "Binding Axum server");
            let tcp_listener = match TcpListener::bind(addr).await {
                Ok(l) => l,
                Err(error) => {
                    tracing::error!(?error, ?addr, "Failed to bind Axum server");
                    return;
                }
            };
            tracing::info!(?addr, "Axum server listening");
            if let Err(error) = axum::serve(tcp_listener, app.into_make_service()).await {
                tracing::error!(?error, "Axum server exited with error");
            }
        });

        let args: Vec<String> = std::env::args().collect();
        tracing::trace!(?args);
        let enable_autostart = args.iter().any(|a| a == "--enable-autostart");
        let disable_autostart = args.iter().any(|a| a == "--disable-autostart");

        tracing::debug!("Starting Tauri");
        tauri::Builder::default()
            .plugin(tauri_plugin_autostart::init(
                tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                Some(vec![]),
            ))
            .setup(move |app| {
                use tauri_plugin_autostart::ManagerExt;
                let autostart = app.autolaunch();
                if enable_autostart {
                    autostart.enable()
                        .inspect_err(|error| tracing::error!(?error, "Failed to enable autostart"))?;
                    tracing::info!("Autostart enabled");
                } else if disable_autostart {
                    autostart.disable()
                        .inspect_err(|error| tracing::error!(?error, "Failed to disable autostart"))?;
                    tracing::info!("Autostart disabled");
                }

                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    tracing::debug!("Got webview window 'main'");
                    let url: tauri::Url = format!("http://localhost:{port}").parse().unwrap();
                    tracing::debug!(%url, "Navigating window");
                    window.navigate(url)
                        .inspect_err(|error| tracing::error!(?error, "Failed to navigate window"))?;
                    let fullscreen = std::env::var("FULLSCREEN")
                        .map(|v| v != "false" && v != "0")
                        .unwrap_or(true);
                    tracing::debug!(?fullscreen, "Setting fullscreen");
                    window.set_fullscreen(fullscreen)
                        .inspect_err(|error| tracing::error!(?error, "Failed to set fullscreen"))?;
                    tracing::debug!("Window setup complete");
                } else {
                    tracing::error!("Failed to get webview window 'main'");
                }
                Ok(())
            })
            .run(tauri::generate_context!())
            .inspect_err(|error| tracing::error!(?error, "Could not run tauri app"))
            .expect("error running tauri application");
    }

    #[cfg(not(feature = "tauri"))]
    {
        tracing::info!("Starting Server");
        // Run Axum server directly
        let tcp_listener = TcpListener::bind(addr).await?;
        axum::serve(tcp_listener, app.into_make_service()).await?;
    }

    Ok(())
}


fn setup_logging() -> Result<(), Error>{
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
    Ok(())
}