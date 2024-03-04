pub mod types;

use axum::extract::FromRef;
use diesel::{
    prelude::Insertable, BoolExpressionMethods, ExpressionMethods, JoinOnDsl,
    NullableExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use types::{HasCox, OarsPerSeat, SeatCount, WeightClass};

use crate::schema::boat::relinquished_at;

use self::types::{BoatAttributes, BoatId, BoatType, OarConfiguration};

use super::DbOrdering;

#[derive(Debug, Clone, diesel::Queryable, diesel::Selectable, diesel::Identifiable)]
#[diesel(table_name = crate::schema::boat)]
pub struct Boat {
    pub id: BoatId,
    /// The name of the boat.
    pub name: String,
    /// The sort of rowers the boat is suitable for
    pub weight_class: WeightClass,
    /// the number of seats in the boat
    pub seat_count: SeatCount,
    /// Is the boat coxed
    pub has_cox: HasCox,
    /// Scull vs sweep
    pub oars_per_seat: OarsPerSeat,
    /// When we acquired the boat
    pub acquired_at: Option<chrono::NaiveDate>,
    /// When we belive the boat was manufactured
    pub manufactured_at: Option<chrono::NaiveDate>,
    /// when the boat is sold, given away, or otherwise taken out of service.
    pub relinquished_at: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::boat)]
pub struct NewBoat {
    pub name: String,
    pub weight_class: WeightClass,
    pub seat_count: SeatCount,
    pub has_cox: HasCox,
    pub oars_per_seat: OarsPerSeat,
    pub acquired_at: Option<chrono::NaiveDate>,
    pub manufactured_at: Option<chrono::NaiveDate>,
}

impl NewBoat {
    pub fn new(
        name: String,
        weight: WeightClass,
        ty: BoatType,
        acquired_at: Option<chrono::NaiveDate>,
        manufactured_at: Option<chrono::NaiveDate>,
    ) -> Self {
        let (has_cox, seat_count, oars_per_seat) = ty.into_values();
        Self {
            name,
            weight_class: weight,
            seat_count,
            has_cox,
            oars_per_seat,
            acquired_at,
            manufactured_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoatOrder {
    Size(DbOrdering),
}

pub struct BoatFilter {
    boat_type: Option<BoatType>,
    coxed: Option<bool>,
    oars: Option<OarsPerSeat>,
}

pub enum BoatFilter2 {
    None,
    ByType(BoatType),
    OarConfig(OarConfiguration),
}

impl Boat {

    pub fn boat_type(&self) -> Option<BoatType> {
        BoatType::from_boat_attributes(BoatAttributes { has_cox: self.has_cox, seats: self.seat_count, oar_configuation: self.oars_per_seat })
    }

    pub fn new_boat(
        conn: &mut SqliteConnection,
        boat: NewBoat,
    ) -> Result<Boat, diesel::result::Error> {
        diesel::insert_into(crate::schema::boat::table)
            .values(boat)
            .get_result(conn)
    }

    pub fn get_boats(
        conn: &mut SqliteConnection,
        filter: BoatFilter2,
    ) -> Result<Vec<Boat>, diesel::result::Error> {
        use crate::schema::boat;
        match filter {
            BoatFilter2::None => boat::table.get_results(conn),
            BoatFilter2::ByType(ty) => {
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
            BoatFilter2::OarConfig(config) => {
                let oars = config.num_oars();
                boat::table
                    .filter(boat::oars_per_seat.eq(oars))
                    .get_results(conn)
            }
        }
    }

    pub fn get_boat(
        conn: &mut SqliteConnection,
        id: BoatId,
    ) -> Result<Boat, diesel::result::Error> {
        use crate::schema::boat;
        boat::table.filter(boat::id.eq(id)).get_result(conn)
    }

    /// Gets rid of the boat by setting its relinquished_at value to today's date
    pub fn get_rid_of_boat(
        conn: &mut SqliteConnection,
        id: BoatId,
    ) -> Result<Boat, diesel::result::Error> {
        use crate::schema::boat;
        let now = chrono::Utc::now().naive_utc().date();
        let target = boat::table.filter(boat::id.eq(id));
        diesel::update(target)
            .set(relinquished_at.eq(Some(now)))
            .get_result(conn)
    }
}

#[derive(Debug, Clone, diesel::Queryable)]
pub struct BoatAndStats {
    #[diesel(embed)]
    pub boat: Boat,
    pub open_issues: Option<i64>,
    pub total_uses: Option<i64>,
    pub uses_last_thirty_days: Option<i64>,
}

impl BoatAndStats {
    pub fn get_boat(
        conn: &mut SqliteConnection,
        id: BoatId,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::{boat, issue, use_event};
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
        use crate::schema::{boat, issue, use_event};
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
