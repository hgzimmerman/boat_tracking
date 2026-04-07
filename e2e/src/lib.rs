use fantoccini::{Client, ClientBuilder};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Child, Command};
use tokio::time::{sleep, Duration};

/// Finds an available TCP port by binding to port 0 and returning the assigned port.
fn available_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("should bind to ephemeral port")
        .local_addr()
        .expect("should have local addr")
        .port()
}

/// Returns the path to the built Tauri binary.
fn app_binary_path() -> std::path::PathBuf {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let target_dir = manifest_dir.parent().unwrap().join("target").join("debug");

    let binary = target_dir.join("boat_tracking");
    assert!(
        binary.exists(),
        "App binary not found at {binary:?}. Run `cargo build --features tauri` first."
    );
    binary
}

/// An isolated test instance with its own tauri-driver, app process, and ports.
pub struct TestInstance {
    tauri_driver: Child,
    app_port: u16,
    driver_port: u16,
    db_path: PathBuf,
}

impl TestInstance {
    /// Spawns a new tauri-driver and app instance with unique ports and a fresh
    /// in-memory database.
    pub async fn start() -> Self {
        let app_port = available_port();
        let driver_port = available_port();
        let native_port = available_port();

        let db_path = std::env::temp_dir().join(format!("boat_tracking_e2e_{app_port}.db"));
        // Ensure no leftover DB from a previous run.
        let _ = std::fs::remove_file(&db_path);

        let tauri_driver = Command::new("tauri-driver")
            .arg("--port")
            .arg(driver_port.to_string())
            .arg("--native-port")
            .arg(native_port.to_string())
            .env("PORT", app_port.to_string())
            .env("DATABASE_URL", db_path.to_str().unwrap())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("tauri-driver must be installed (`cargo install tauri-driver --locked`)");

        // Give tauri-driver a moment to start listening.
        sleep(Duration::from_millis(500)).await;

        Self {
            tauri_driver,
            app_port,
            driver_port,
            db_path,
        }
    }

    /// Connects a fantoccini client to this instance's tauri-driver, which
    /// launches the app binary via the `tauri:options` capability.
    pub async fn connect(&self) -> Client {
        let caps = serde_json::json!({
            "tauri:options": {
                "application": app_binary_path().to_str().unwrap(),
            }
        });

        let client = ClientBuilder::native()
            .capabilities(caps.as_object().unwrap().clone())
            .connect(&format!("http://localhost:{}", self.driver_port))
            .await
            .expect("should connect to tauri-driver");

        // Wait for the app's Axum server to be ready.
        self.wait_for_server().await;

        client
    }

    pub fn base_url(&self) -> String {
        format!("http://localhost:{}", self.app_port)
    }

    async fn wait_for_server(&self) {
        for _ in 0..60 {
            if tokio::net::TcpStream::connect(format!("127.0.0.1:{}", self.app_port))
                .await
                .is_ok()
            {
                return;
            }
            sleep(Duration::from_millis(250)).await;
        }
        panic!(
            "App server did not start on port {} within 15 seconds",
            self.app_port
        );
    }
}

impl Drop for TestInstance {
    fn drop(&mut self) {
        let _ = self.tauri_driver.kill();
        let _ = std::fs::remove_file(&self.db_path);
    }
}
