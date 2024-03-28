pub mod db;
#[cfg(feature = "ssr")]
mod schema;
pub mod ui;
#[cfg(feature = "ssr")]
pub mod api;

// pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
// pub type DynResult<T> = Result<T, DynError>;
