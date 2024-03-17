pub mod boat;
pub mod issue;
pub mod use_event;
pub mod use_event_batch;

#[cfg(feature = "ssr")]
pub mod sql_types {
    pub use super::boat::types::WeightClassMapping;
    pub use super::use_event::UseScenarioMapping;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DbOrdering {
    Asc,
    Desc,
}
