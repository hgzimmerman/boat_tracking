pub mod queries;

use super::{boat::types::BoatId, use_event_batch::BatchId, use_scenario::UseScenarioId};

/// Whenever the equipment is used, it can be recorded that it was used
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(diesel::Queryable, diesel::Selectable, diesel::Identifiable)]
#[diesel(table_name = crate::schema::use_event)]
pub struct UseEvent {
    pub id: UseEventId,
    pub boat_id: BoatId,
    pub batch_id: Option<BatchId>,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    pub use_scenario_id: UseScenarioId,
    pub note: Option<String>,
}

#[derive(Debug, Clone, diesel::Insertable)]
#[diesel(table_name = crate::schema::use_event)]
pub struct NewUseEvent {
    pub boat_id: BoatId,
    pub batch_id: Option<BatchId>,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    pub use_scenario_id: UseScenarioId,
    pub note: Option<String>,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    diesel_derive_newtype::DieselNewType,
)]
pub struct UseEventId(i32);

impl UseEventId {
    pub fn new(new: i32) -> Self {
        Self(new)
    }
    pub fn as_int(&self) -> i32 {
        self.0
    }
}
