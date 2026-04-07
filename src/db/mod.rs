pub mod boat;
pub mod issue;
pub mod state;
pub mod use_event;
pub mod use_event_batch;
pub mod use_scenario;

#[cfg(test)]
mod tests;

pub mod sql_types {
    pub use super::boat::types::WeightClassMapping;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DbOrdering {
    Asc,
    Desc,
}
