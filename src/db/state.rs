use deadpool_diesel::sqlite::Pool;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type ArcPool = Arc<Pool>;

#[derive(Clone)]
pub struct AppState {
    pool: ArcPool,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}

impl AppState {
    pub fn new(conn_str: &str) -> Self {
        let manager =
            deadpool_diesel::sqlite::Manager::new(conn_str, deadpool_diesel::Runtime::Tokio1);
        let pool = deadpool_diesel::sqlite::Pool::builder(manager)
            .max_size(40)
            .build()
            .expect("should build pool");

        // Run pending migrations on startup
        {
            tracing::info!("Checking for pending database migrations...");
            let mut conn = diesel::SqliteConnection::establish(conn_str)
                .expect("should connect to database for migrations");
            let applied = conn.run_pending_migrations(MIGRATIONS)
                .expect("should run database migrations");
            if applied.is_empty() {
                tracing::info!("Database is up to date, no migrations needed");
            } else {
                tracing::info!("Applied {} migration(s)", applied.len());
            }
        }

        let pool = Arc::new(pool);
        Self { pool }
    }

    pub fn pool(&self) -> Arc<deadpool_diesel::sqlite::Pool> {
        self.pool.clone()
    }
}

impl axum::extract::FromRequestParts<AppState> for AppState {
    type Rejection = String;

    fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move { Ok(state.clone()) }
    }
}
