use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::schema::use_event;
use super::boat::types::BoatId;

/// Whenever the equipment is used, it can be recorded that it was used
#[derive(Debug, Clone, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = crate::schema::use_event)]

pub struct UseEvent {
    id: UseEventId,
    boat_id: BoatId,
    recorded_at: chrono::NaiveDateTime,
    use_scenario: UseScenario,
    note: Option<String>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, diesel_derive_enum::DbEnum)]
pub enum UseScenario {
    AM,
    PM,
    Regatta,
    Other
}


#[derive(Debug, Clone, diesel::Insertable)]
#[diesel(table_name = crate::schema::use_event)]

pub struct NewUseEvent {
    pub boat_id: BoatId,
    pub recorded_at: chrono::NaiveDateTime,
    pub use_scenario: UseScenario,
    pub note: Option<String>
}

impl UseEvent {
    pub fn new_event(conn: &mut SqliteConnection, event: NewUseEvent) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(use_event::table)  
            .values(event)
            .get_result(conn)
    }

    pub fn events_for_boat(conn: &mut SqliteConnection, boat_id: BoatId) -> Result<Vec<UseEvent>, diesel::result::Error> {
        use_event::table.filter(
            use_event::boat_id.eq(boat_id)
        )
        .order_by(use_event::recorded_at.desc()) // newest first
        .get_results(conn)
    }

}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize, diesel_derive_newtype::DieselNewType)]
pub struct UseEventId(i32);

impl UseEventId {
    pub fn new(new:i32) -> Self {
        Self(new)
    }
    pub fn as_int(&self) -> i32 {
        self.0
    }
}