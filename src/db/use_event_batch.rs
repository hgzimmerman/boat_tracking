use super::{boat::types::BoatId, use_event::UseScenario};

pub mod queries;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[derive(diesel::Queryable, diesel::Selectable, diesel::Identifiable)]
#[diesel(table_name = crate::schema::use_event_batch)]
pub struct UseEventBatch {
    pub id: BatchId,
    pub recorded_at: chrono::NaiveDateTime,
    pub use_scenario: UseScenario,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(diesel::AsChangeset, diesel::Identifiable, diesel::Queryable)]
#[diesel(table_name = crate::schema::use_event_batch)]
pub struct UseEventBatchChangeset {
    pub id: BatchId,
    pub recorded_at: Option<chrono::NaiveDateTime>,
    pub use_scenario: Option<UseScenario>,
}

#[derive(
    Clone,
    Copy,
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
pub struct BatchId(i32);

impl BatchId {
    pub fn new(new: i32) -> Self {
        Self(new)
    }
    pub fn as_int(&self) -> i32 {
        self.0
    }
}
impl std::str::FromStr for BatchId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s).map(Self)
    }
}
impl std::fmt::Display for BatchId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct NewBatchArgs {
    pub boat_ids: Vec<BoatId>,
    pub batch: NewBatch,
}

#[derive(Debug, Clone, diesel::Insertable)]
#[diesel(table_name = crate::schema::use_event_batch)]
pub struct NewBatch {
    pub use_scenario: UseScenario,
    pub recorded_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[derive(diesel::Selectable, diesel::Queryable)]
#[diesel(table_name = crate::schema::use_event_batch)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BatchAndCounts {
    #[diesel(embed)]
    pub batch: UseEventBatch,
    #[diesel(select_expression = diesel::dsl::count(crate::schema::use_event::id))]
    #[diesel(select_expression_type = diesel::dsl::count<crate::schema::use_event::id>)]
    pub use_counts: i64,
}
