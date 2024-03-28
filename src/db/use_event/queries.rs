use std::collections::HashMap;

use super::*;
use crate::schema::use_event;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::SqliteConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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

    /// Gets the counts per day of uses for a specified boat.
    /// 
    /// The returned list will have empty 0s for dates
     pub fn timeseries_for_boat(
        conn: &mut SqliteConnection,
        boat_id: BoatId,
        date_start: NaiveDateTime,
        date_end: Option<NaiveDateTime>,
    ) -> Result<Vec<(NaiveDate, usize)>, diesel::result::Error> {
        let mut filter = use_event::table
            .filter(use_event::boat_id.eq(boat_id))
            .filter(use_event::recorded_at.ge(date_start))
            .into_boxed();
        if let Some(date_end) = date_end {
            filter = filter.filter(use_event::recorded_at.lt(date_end));
        }

        // kind of a lame strategy, but the idea is to grab the dates, and then do the correlation server-side 
        let datetimes: Vec<NaiveDateTime> = filter
            .order_by(use_event::recorded_at.asc()) // oldest first
            .select(use_event::recorded_at)
            .get_results::<NaiveDateTime>(conn)?;
        
        let ts_map = datetimes.into_iter()
        .map(|datetime: NaiveDateTime| datetime.date())
        .fold(HashMap::new(), |mut acc, next| {
            *acc.entry(next).or_default() += 1usize;
            acc
        });

        let start = date_start.date();
        let end = date_end.as_ref().map(chrono::NaiveDateTime::date).unwrap_or_else(|| chrono::Utc::now().naive_utc().date());

        let list = start.iter_days().take_while(|d| d <= &end)
            .map(|date| (date, ts_map.get(&date).cloned().unwrap_or(0)))
            .collect::<Vec<_>>();

        Ok(list)
    }
}