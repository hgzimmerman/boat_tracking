use super::*;
use diesel::SqliteConnection;
use crate::schema::use_event;

impl UseEvent {
    pub fn new_event(
        conn: &mut SqliteConnection,
        event: NewUseEvent,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(use_event::table)
            .values(event)
            .get_result(conn)
    }

    pub fn events_for_boat(
        conn: &mut SqliteConnection,
        boat_id: BoatId,
    ) -> Result<Vec<UseEvent>, diesel::result::Error> {
        use_event::table
            .filter(use_event::boat_id.eq(boat_id))
            .order_by(use_event::recorded_at.desc()) // newest first
            .get_results(conn)
    }
}