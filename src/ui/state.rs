use deadpool_diesel::sqlite::Pool;
use dioxus_fullstack::prelude::FromServerContext;
use std::sync::Arc;
use dioxus::prelude::*;

pub type ArcPool = Arc<Pool>;

pub type ScopeState<'a> = Scope<'a, AppState>;

#[derive(Clone)]
pub struct AppState {
    // conn: std::sync::Arc<tokio::sync::Mutex<diesel::SqliteConnection>>,
    pool: ArcPool,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}


impl AppState {
    pub fn new(conn_str: &str) -> Self {
        let manager = deadpool_diesel::sqlite::Manager::new(conn_str, deadpool_diesel::Runtime::Tokio1);
        let pool = deadpool_diesel::sqlite::Pool::builder(manager).max_size(40).build().expect("should build pool");

        let pool = Arc::new(pool);
        Self { 
            pool 
        }
    }
    pub async fn conn(&self) -> Result<deadpool_diesel::sqlite::Connection, anyhow::Error> {
        self.pool.get().await.map_err(From::from)
    }
    pub fn pool(&self) ->  Arc<deadpool_diesel::sqlite::Pool> {
        self.pool.clone()
    }
}


/// A type was not found in the server context
pub struct NotFoundInServerContext<T: 'static>(std::marker::PhantomData<T>);

impl<T: 'static> std::fmt::Debug for NotFoundInServerContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        write!(f, "`{type_name}` not found in server context")
    }
}

impl<T: 'static> std::fmt::Display for NotFoundInServerContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        write!(f, "`{type_name}` not found in server context")
    }
}

impl<T: 'static> std::error::Error for NotFoundInServerContext<T> {}


#[async_trait::async_trait(?Send)]
impl FromServerContext<dioxus_fullstack::prelude::Axum> for AppState {
    type Rejection = NotFoundInServerContext<AppState>;

    async fn from_request(req: &dioxus_fullstack::prelude::DioxusServerContext) ->  Result<Self,Self::Rejection> {
        req.get().ok_or_else(|| {
            NotFoundInServerContext::<AppState>(std::marker::PhantomData::<AppState>)
        } )
    }
}
