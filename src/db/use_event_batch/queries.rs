use diesel::{Connection, ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

use crate::{db::{boat::Boat, use_event::{NewUseEvent, UseScenario}}, schema::boat};

use super::{BatchId, NewBatchArgs, UseEventBatch};
        use crate::schema::{use_event_batch, use_event};

impl UseEventBatch {
    pub fn create_batch(
        conn: &mut SqliteConnection,
        new_batch: NewBatchArgs
    ) -> Result<BatchId, diesel::result::Error> {
        conn.transaction(|conn| {
            let NewBatchArgs { boat_ids, batch } = new_batch;
            let batch_id: BatchId = diesel::insert_into(crate::schema::use_event_batch::table)
                .values(&batch)
                .returning(crate::schema::use_event_batch::id)
                .get_result(conn)?;
            let use_events = boat_ids.into_iter().map(|boat_id| NewUseEvent{
                boat_id,
                batch_id: Some(batch_id),
                recorded_at: batch.recorded_at,
                use_scenario: batch.use_scenario,
                note: None,
            }).collect::<Vec<_>>();
            diesel::insert_into(crate::schema::use_event::table)
            .values(use_events)
            .execute(conn)?;
            Ok(batch_id)
        })
        
    }

    pub fn get_most_recent_batch(
        conn: &mut SqliteConnection,
        scenario: Option<UseScenario>,
        offset: usize 
    ) -> Result<Option<UseEventBatch>, diesel::result::Error> {
        match scenario {
            Some(scenario) => {
                use_event_batch::table
                .filter(use_event_batch::use_scenario.eq(scenario))
                .order_by(use_event_batch::recorded_at.desc())
                .offset(i64::try_from(offset).unwrap_or_default())
                .get_result(conn)
                .optional()
            },
            None => {
                use_event_batch::table
                .order_by(use_event_batch::recorded_at.desc())
                .offset(i64::try_from(offset).unwrap_or_default())
                .get_result(conn)
                .optional()
            },
        }
    }
    pub fn get_boats_for_batch(
        conn: &mut SqliteConnection,
        batch_id: BatchId
    ) -> Result<Vec<Boat>, diesel::result::Error> {
        use_event::table
        .filter(use_event::batch_id.eq(batch_id))
        .inner_join(boat::table.on(boat::id.eq(use_event::boat_id)))
        .select(Boat::as_select())
        .get_results(conn)
    }
}