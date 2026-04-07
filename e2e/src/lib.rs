use fantoccini::{Client, ClientBuilder};
use std::net::TcpListener;
use std::os::unix::process::CommandExt;
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

/// Returns the project root directory (parent of the e2e crate).
fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Returns the path to the built Tauri binary.
fn app_binary_path() -> PathBuf {
    let binary = project_root().join("target").join("debug").join("boat_tracking");
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

        // Copy public/ next to the binary so the exe-relative static file
        // lookup works the same way it does in a real deployment.
        let binary = app_binary_path();
        let target_public = binary.parent().unwrap().join("public");
        let source_public = project_root().join("public");
        if !target_public.exists() {
            let _ = std::fs::create_dir(&target_public);
            if let Ok(entries) = std::fs::read_dir(&source_public) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                        let _ = std::fs::copy(entry.path(), target_public.join(entry.file_name()));
                    }
                }
            }
        }

        let tauri_driver = unsafe {
            Command::new("tauri-driver")
                .arg("--port")
                .arg(driver_port.to_string())
                .arg("--native-port")
                .arg(native_port.to_string())
                .env("PORT", app_port.to_string())
                .env("DATABASE_URL", db_path.to_str().unwrap())
                .env("FULLSCREEN", "false")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                // Start as a new process group so we can kill the entire tree
                // (tauri-driver + the app it spawns) on cleanup.
                .pre_exec(|| {
                    libc::setpgid(0, 0);
                    Ok(())
                })
                .spawn()
                .expect("tauri-driver must be installed (`cargo install tauri-driver --locked`)")
        };

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

        // Set a consistent window size so elements are interactable.
        client
            .set_window_size(1280, 1024)
            .await
            .unwrap_or_else(|_| {
                // WebKitWebDriver may not support this; ignore errors.
            });

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

/// Sets a `<select>` element's value via JavaScript, bypassing visibility
/// constraints that cause `ElementNotInteractable` in WebKitWebDriver.
pub async fn select_value(client: &Client, css_selector: &str, value: &str) {
    let js = format!(
        r#"document.querySelector("{}").value = "{}";"#,
        css_selector, value
    );
    client.execute(&js, vec![]).await.unwrap();
}

/// Scrolls an element into view and clicks it. Works around
/// `ElementNotInteractable` errors when elements are off-screen.
pub async fn scroll_and_click(client: &Client, css_selector: &str) {
    let js = format!(
        r#"var el = document.querySelector("{}"); el.scrollIntoView(); el.click();"#,
        css_selector
    );
    client.execute(&js, vec![]).await.unwrap();
}

/// Sets an input element's value via JavaScript, clearing it first.
/// Works around `ElementNotInteractable` for off-screen inputs.
pub async fn set_input_value(client: &Client, css_selector: &str, value: &str) {
    let js = format!(
        r#"var el = document.querySelector("{}"); el.scrollIntoView(); el.value = "{}"; el.dispatchEvent(new Event("input", {{ bubbles: true }}));"#,
        css_selector, value
    );
    client.execute(&js, vec![]).await.unwrap();
}

impl Drop for TestInstance {
    fn drop(&mut self) {
        // Kill the entire process group (tauri-driver + the app binary it spawned).
        let pid = self.tauri_driver.id() as i32;
        unsafe {
            libc::kill(-pid, libc::SIGKILL);
        }
        let _ = self.tauri_driver.wait();
        let _ = std::fs::remove_file(&self.db_path);
    }
}
