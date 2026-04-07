pub mod queries;

use super::{boat::types::BoatId, use_event::UseEventId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[derive(diesel::Queryable, diesel::Selectable, diesel::Identifiable)]
#[diesel(table_name = crate::schema::issue)]
pub struct Issue {
    pub id: IssueId,
    pub boat_id: Option<BoatId>,
    pub use_event_id: Option<UseEventId>,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    pub note: String,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, diesel::Insertable)]
#[diesel(table_name = crate::schema::issue)]
pub struct NewIssue {
    pub boat_id: Option<BoatId>,
    pub use_event_id: Option<UseEventId>,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    pub note: String,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
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
pub struct IssueId(i32);

impl IssueId {
    pub fn new(new: i32) -> Self {
        Self(new)
    }
    pub fn as_int(&self) -> i32 {
        self.0
    }
}
