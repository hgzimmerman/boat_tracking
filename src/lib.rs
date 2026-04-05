#[cfg(feature = "ssr")]
pub mod api;
pub mod db;
#[cfg(feature = "ssr")]
mod schema;
pub mod ui;

// HTMX + Maud migration modules
#[cfg(feature = "ssr")]
pub mod templates;
#[cfg(feature = "ssr")]
pub mod handlers;

// pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
// pub type DynResult<T> = Result<T, DynError>;
