use deadpool_diesel::sqlite::Pool;
use std::sync::Arc;

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
