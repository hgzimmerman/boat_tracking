use std::collections::HashMap;

use super::*;
use crate::api::wire::BoatUseCsvRow;
use crate::schema::{boat, use_event};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use diesel::SqliteConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

impl UseEvent {
    #[tracing::instrument(level = "debug", skip(conn), err)]
    pub fn new_event(
        conn: &mut SqliteConnection,
        event: NewUseEvent,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(use_event::table)
            .values(event)
            .get_result(conn)
    }

    #[tracing::instrument(level = "debug", skip_all, err)]
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
    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn daily_timeseries_for_boat(
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

        let ts_map = datetimes
            .into_iter()
            .map(|datetime: NaiveDateTime| datetime.date())
            .fold(HashMap::new(), |mut acc, next| {
                *acc.entry(next).or_default() += 1usize;
                acc
            });

        let start = date_start.date();
        let end = date_end
            .as_ref()
            .map(chrono::NaiveDateTime::date)
            .unwrap_or_else(|| chrono::Utc::now().naive_utc().date());

        let list = start
            .iter_days()
            .take_while(|d| d <= &end)
            .map(|date| (date, ts_map.get(&date).cloned().unwrap_or(0)))
            .collect::<Vec<_>>();

        Ok(list)
    }

    /// Sums uses over a month for a specified boat
    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn monthly_timeseries_for_boat(
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

        let ts_map = datetimes
            .into_iter()
            .map(|datetime: NaiveDateTime| datetime.date())
            .fold(HashMap::new(), |mut acc, next| {
                let key = NaiveDate::from_ymd_opt(next.year(), next.month(), 1)
                    .expect("Should be valid date");
                *acc.entry(key).or_default() += 1usize;
                acc
            });

        let start = date_start.date();
        let start =
            NaiveDate::from_ymd_opt(start.year(), start.month(), 1).expect("should be valid date");
        let end = date_end
            .as_ref()
            .map(chrono::NaiveDateTime::date)
            .unwrap_or_else(|| chrono::Utc::now().naive_utc().date());
        let end =
            NaiveDate::from_ymd_opt(end.year(), end.month(), 1).expect("should be valid date");

        let list = start
            .iter_days()
            .take_while(|d| d <= &end)
            .filter(|d| d.day() == 1)
            .map(|date| (date, ts_map.get(&date).cloned().unwrap_or(0)))
            .collect::<Vec<_>>();

        Ok(list)
    }

    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn export_events(
        conn: &mut SqliteConnection,
        date_start: Option<NaiveDateTime>,
        date_end: Option<NaiveDateTime>,
        boat_ids: Option<Vec<BoatId>>,
    ) -> Result<Vec<BoatUseCsvRow>, diesel::result::Error> {
        let mut query = use_event::table.inner_join(boat::table).into_boxed();
        if let Some(date_start) = date_start {
            query = query.filter(use_event::recorded_at.ge(date_start));
        }
        if let Some(date_end) = date_end {
            query = query.filter(use_event::recorded_at.lt(date_end));
        }
        if let Some(boats) = boat_ids {
            query = query.filter(use_event::boat_id.eq_any(boats));
        }

        query
            .order_by(use_event::recorded_at.desc()) // newest first
            .get_results::<(UseEvent, crate::db::boat::Boat)>(conn)
            .map(|results| {
                results.into_iter().filter_map(|(event, boat)| {
                    let boat_type = {
                        let bt = boat.boat_type();
                        if bt.is_none() {
                            tracing::error!(?boat.name, ?boat.id, "boat type cant be known, filtering events") 
                        }
                        bt?
                    };
                    Some(BoatUseCsvRow{
                        boat_name: boat.name,
                        boat_type,
                        boat_weight_class: boat.weight_class,
                        used_at: event.recorded_at,
                        use_scenario: event.use_scenario,
                        boat_id: boat.id,
                        batch_id: event.batch_id,
                    })
                })
                .collect()
            })
    }
}
