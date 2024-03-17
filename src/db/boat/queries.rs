use super::*;

use crate::schema::{boat, issue, use_event};
use diesel::{SqliteConnection, TextExpressionMethods};

impl Boat {


    pub fn new_boat(
        conn: &mut SqliteConnection,
        boat: NewBoat,
    ) -> Result<Boat, diesel::result::Error> {
        diesel::insert_into(crate::schema::boat::table)
            .values(boat)
            .get_result(conn)
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn get_boats(
        conn: &mut SqliteConnection,
        filter: BoatFilter2,
        search: Option<String>
    ) -> Result<Vec<Boat>, diesel::result::Error> {
        tracing::debug!(?search);
        match (filter, search.map(|x| format!("%{x}%"))) {
            (BoatFilter2::None, None) => boat::table.get_results(conn),
            (BoatFilter2::ByType(ty), None) => {
                let (has_cox, seats, oars_per_seat) = ty.into_values();
                let cox = has_cox.as_value();
                let seats = seats.count();
                let oars = oars_per_seat.count();

                boat::table
                    .filter(
                        boat::has_cox
                            .eq(cox)
                            .and(boat::seat_count.eq(seats).and(boat::oars_per_seat.eq(oars))),
                    )
                    .get_results(conn)
            }
            (BoatFilter2::OarConfig(config), None) => {
                let oars = config.num_oars();
                boat::table
                    .filter(boat::oars_per_seat.eq(oars))
                    .get_results(conn)
            }
            (BoatFilter2::None, Some(search)) => boat::table.filter(boat::name.like(search)).get_results(conn),
            (BoatFilter2::ByType(ty), Some(search)) => {
                let (has_cox, seats, oars_per_seat) = ty.into_values();
                let cox = has_cox.as_value();
                let seats = seats.count();
                let oars = oars_per_seat.count();

                boat::table
                    .filter(boat::name.like(search))
                    .filter(
                        boat::has_cox
                            .eq(cox)
                            .and(boat::seat_count.eq(seats).and(boat::oars_per_seat.eq(oars))),
                    )
                    .get_results(conn)
            }
            (BoatFilter2::OarConfig(config), Some(search)) => {
                let oars = config.num_oars();
                boat::table
                    .filter(boat::name.like(search))
                    .filter(boat::oars_per_seat.eq(oars))
                    .get_results(conn)
            }
        }
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
