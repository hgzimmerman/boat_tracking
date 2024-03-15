
#[cfg(feature = "ssr")]
pub mod queries;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use super::boat::types::BoatId;

/// Whenever the equipment is used, it can be recorded that it was used
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(diesel::Queryable, diesel::Selectable, diesel::Identifiable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::use_event))]
pub struct UseEvent {
    id: UseEventId,
    boat_id: BoatId,
    recorded_at: chrono::NaiveDateTime,
    use_scenario: UseScenario,
    note: Option<String>,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[cfg_attr(feature = "ssr", derive(diesel_derive_enum::DbEnum))]
#[cfg_attr(feature = "ssr", DbValueStyle = "verbatim")]
pub enum UseScenario {
    AM,
    PM,
    Regatta,
    Other,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(diesel::Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::use_event))]
pub struct NewUseEvent {
    pub boat_id: BoatId,
    pub recorded_at: chrono::NaiveDateTime,
    pub use_scenario: UseScenario,
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
