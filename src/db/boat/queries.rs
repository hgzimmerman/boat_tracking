use super::*;

use crate::schema::{
    boat::{self},
    issue, use_event,
};
use diesel::{QueryDsl, SqliteConnection, TextExpressionMethods};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, RunQueryDsl,
    SelectableHelper,
};

impl Boat {
    pub fn new_boat(
        conn: &mut SqliteConnection,
        boat: NewBoat,
    ) -> Result<Boat, diesel::result::Error> {
        diesel::insert_into(crate::schema::boat::table)
            .values(boat)
            .get_result(conn)
    }
    
    /// Replaces the old boat with a new one.
    /// No selective updates (double-optional) or anything.
    pub fn update_boat(
        conn: &mut SqliteConnection,
        boat: &Boat,
    ) -> Result<Boat, diesel::result::Error> {
        diesel::update(boat)
            .set(boat)
            .get_result(conn)
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn get_boats3(
        conn: &mut SqliteConnection,
        filter: BoatFilter3,
        search: Option<String>,
    ) -> Result<Vec<Boat>, diesel::result::Error> {
        let BoatFilter3 {
            num_seats,
            coxed,
            oars_config,
            ..
        } = filter;

        let seat_count: Option<i32> = num_seats.as_ref().map(SeatCount::count);
        let oars_per_seat: Option<i32> = oars_config.as_ref().map(OarConfiguration::num_oars);
        let cox = coxed.as_ref().map(HasCox::as_value);

        let mut query = boat::table.filter(boat::relinquished_at.is_not_null()).into_boxed();

        if let Some(search) = search.map(|x| format!("%{x}%")) {
            query = query.filter(boat::name.like(search))
        }
        if let Some(num_seats) = seat_count {
            query = query.filter(boat::seat_count.eq(num_seats))
        };
        if let Some(cox) = cox {
            query = query.filter(boat::has_cox.eq(cox))
        };
        if let Some(oars_per_seat) = oars_per_seat {
            query = query.filter(boat::oars_per_seat.eq(oars_per_seat))
        };

        query.get_results(conn)
    }

    pub fn get_boat(
        conn: &mut SqliteConnection,
        id: BoatId,
    ) -> Result<Boat, diesel::result::Error> {
        boat::table.filter(boat::id.eq(id)).get_result(conn)
    }

    /// Gets rid of the boat by setting its relinquished_at value to today's date
    pub fn get_rid_of_boat(
        conn: &mut SqliteConnection,
        id: BoatId,
    ) -> Result<Boat, diesel::result::Error> {
        let now = chrono::Utc::now().naive_utc().date();
        let target = boat::table.filter(boat::id.eq(id));
        diesel::update(target)
            .set(boat::relinquished_at.eq(Some(now)))
            .get_result(conn)
    }
}

impl BoatAndStats {
    pub fn get_boat(
        conn: &mut SqliteConnection,
        id: BoatId,
    ) -> Result<Self, diesel::result::Error> {
        let ago_30_d = chrono::Utc::now().naive_utc() - chrono::TimeDelta::days(30);
        boat::table
            .filter(boat::id.eq(id))
            .select((
                Boat::as_select(),
                issue::table
                    .select(diesel::dsl::count(
                        issue::boat_id.eq(id).and(issue::resolved_at.is_null()),
                    ))
                    .single_value(),
                use_event::table
                    .select(diesel::dsl::count(use_event::boat_id.eq(id)))
                    .single_value(),
                use_event::table
                    .select(diesel::dsl::count(
                        use_event::boat_id
                            .eq(id)
                            .and(use_event::recorded_at.gt(ago_30_d)),
                    ))
                    .single_value(),
            ))
            .get_result::<BoatAndStats>(conn)
    }

    pub fn get_boats(conn: &mut SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        let ago_30_d = chrono::Utc::now().naive_utc() - chrono::TimeDelta::days(30);
        boat::table
            .left_outer_join(issue::table.on(issue::boat_id.eq(boat::id.nullable())))
            .left_outer_join(use_event::table.on(use_event::boat_id.eq(boat::id)))
            .group_by(boat::id)
            .select((
                Boat::as_select(),
                diesel::dsl::count(
                    issue::boat_id
                        .eq(boat::id.nullable())
                        .and(issue::resolved_at.is_null()),
                )
                .nullable(),
                diesel::dsl::count(use_event::boat_id.eq(boat::id)).nullable(),
                diesel::dsl::count(
                    use_event::boat_id
                        .eq(boat::id)
                        .and(use_event::recorded_at.gt(ago_30_d)),
                )
                .nullable(),
            ))
            .get_results::<BoatAndStats>(conn)
    }
}
