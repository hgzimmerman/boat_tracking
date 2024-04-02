use diesel::{
    Connection, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, OptionalExtension,
    QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};

use crate::{
    db::{
        boat::{types::BoatId, Boat},
        use_event::{NewUseEvent, UseEvent, UseScenario},
    },
    schema::boat,
};

use super::{BatchAndCounts, BatchId, NewBatchArgs, UseEventBatch, UseEventBatchChangeset};
use crate::schema::{use_event, use_event_batch};

impl UseEventBatch {
    pub fn create_batch(
        conn: &mut SqliteConnection,
        new_batch: NewBatchArgs,
    ) -> Result<BatchId, diesel::result::Error> {
        conn.transaction(|conn| {
            let NewBatchArgs { boat_ids, batch } = new_batch;
            let batch_id: BatchId = diesel::insert_into(crate::schema::use_event_batch::table)
                .values(&batch)
                .returning(crate::schema::use_event_batch::id)
                .get_result(conn)?;
            let use_events = boat_ids
                .into_iter()
                .map(|boat_id| NewUseEvent {
                    boat_id,
                    batch_id: Some(batch_id),
                    recorded_at: batch.recorded_at,
                    use_scenario: batch.use_scenario,
                    note: None,
                })
                .collect::<Vec<_>>();
            diesel::insert_into(crate::schema::use_event::table)
                .values(use_events)
                .execute(conn)?;
            Ok(batch_id)
        })
    }


    /// Gets a list of batches and their use count.
    pub fn get_most_recent_batches_and_their_use_count(
        conn: &mut SqliteConnection,
        scenario: Option<UseScenario>,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<BatchAndCounts>, diesel::result::Error> {
        match scenario {
            Some(scenario) => {
                // This inner_join should exclude batches without any events.
                // This is desirable because:
                // * A we don't allow creation of empty batches based on rules in the UI.
                // * An empty batch is useless anyways.
                use_event_batch::table
                    .filter(use_event_batch::use_scenario.eq(scenario))
                    .inner_join(use_event::table)
                    .order_by(use_event_batch::recorded_at.desc())
                    .group_by(use_event_batch::id)
                    .select(BatchAndCounts::as_select())
                    .offset(i64::try_from(offset).unwrap_or_default())
                    .limit(i64::try_from(limit).unwrap_or(20))
                    .get_results::<BatchAndCounts>(conn)
            }
            None => use_event_batch::table
                .inner_join(
                    use_event::table.on(use_event::batch_id.eq(use_event_batch::id.nullable())),
                )
                .order_by(use_event_batch::recorded_at.desc())
                .group_by(use_event_batch::id)
                .select(BatchAndCounts::as_select())
                .offset(i64::try_from(offset).unwrap_or_default())
                .limit(i64::try_from(limit).unwrap_or(20))
                .get_results::<BatchAndCounts>(conn),
        }
    }

    pub fn get_events_and_boats_for_batch(
        conn: &mut SqliteConnection,
        batch_id: BatchId,
    ) -> Result<Vec<(UseEvent, Boat)>, diesel::result::Error> {
        use_event::table
            .filter(use_event::batch_id.eq(batch_id))
            .inner_join(boat::table.on(boat::id.eq(use_event::boat_id)))
            .select((UseEvent::as_select(), Boat::as_select()))
            .get_results(conn)
    }
    
    pub fn get_batch(conn: &mut SqliteConnection, id: BatchId) -> Result<Option<UseEventBatch>, diesel::result::Error> {
        use_event_batch::table
            .filter(use_event_batch::id.eq(id))
            .get_result::<UseEventBatch>(conn)
            .optional()
    }

    /// Removes all events in a given batch, then creates new events to replace them.
    pub fn replace_batch_uses(
        conn: &mut SqliteConnection,
        batch_id: BatchId,
        boat_ids: Vec<BoatId>,
        use_type: Option<UseScenario>,
        recorded_at: Option<chrono::NaiveDateTime>,
    ) -> Result<BatchId, diesel::result::Error> {
        conn.transaction(|conn| {
            let target = use_event::table.filter(use_event::batch_id.eq(batch_id));
            diesel::delete(target).execute(conn)?;

            let batch: UseEventBatch = if use_type.is_some() || recorded_at.is_some() {
                let changeset = UseEventBatchChangeset {
                    id: batch_id,
                    recorded_at,
                    use_scenario: use_type,
                };
                diesel::update(&changeset)
                    .set(&changeset)
                    .get_result(conn)?
            } else {
                use_event_batch::table
                    .filter(use_event_batch::id.eq(batch_id))
                    .get_result(conn)?
            };
            let use_events = boat_ids
                .into_iter()
                .map(|boat_id| NewUseEvent {
                    boat_id,
                    batch_id: Some(batch_id),
                    recorded_at: recorded_at.unwrap_or(batch.recorded_at),
                    use_scenario: use_type.unwrap_or(batch.use_scenario),
                    note: None,
                })
                .collect::<Vec<_>>();
            diesel::insert_into(crate::schema::use_event::table)
                .values(use_events)
                .execute(conn)
        })?;
        Ok(batch_id)
    }
}
