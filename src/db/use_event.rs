#[cfg(feature = "ssr")]
pub mod queries;

use super::{boat::types::BoatId, use_event_batch::BatchId};

/// Whenever the equipment is used, it can be recorded that it was used
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::Selectable, diesel::Identifiable,)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::use_event))]
pub struct UseEvent {
    pub id: UseEventId,
    pub boat_id: BoatId,
    pub batch_id: Option<BatchId>,
    pub recorded_at: chrono::NaiveDateTime,
    pub use_scenario: UseScenario,
    pub note: Option<String>,
}

// use_scenario TEXT CHECK( use_scenario IN ('Youth', 'Adult', 'LearnToRow', 'ScullingSaturday', 'PrivateSession', 'Regatta', 'Other') ) NOT NULL
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
#[cfg_attr(feature = "ssr", derive(diesel_derive_enum::DbEnum))]
#[cfg_attr(feature = "ssr", DbValueStyle = "verbatim")]
pub enum UseScenario {
    YouthGgrc,
    YouthSomerville,
    Adult,
    LearnToRow,
    ScullingSaturday,
    PrivateSession,
    Regatta,
    Other,
}
impl std::fmt::Display for UseScenario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UseScenario::YouthGgrc => "Youth-GGRC",
            UseScenario::YouthSomerville => "Youth-Somerville",
            UseScenario::Adult => "Adult",
            UseScenario::LearnToRow => "Learn To Row",
            UseScenario::ScullingSaturday => "Sculling Saturday",
            UseScenario::PrivateSession => "Private Session",
            UseScenario::Regatta => "Regatta",
            UseScenario::Other => "Other",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(diesel::Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::use_event))]
pub struct NewUseEvent {
    pub boat_id: BoatId,
    pub batch_id: Option<BatchId>,
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
