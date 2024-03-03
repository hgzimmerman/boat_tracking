pub mod types;
use diesel::{prelude::Insertable, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection, NullableExpressionMethods};
use types::{WeightClass, SeatCount, HasCox, OarsPerSeat};




use self::types::{BoatId, BoatType, OarConfiguration};

use super::DbOrdering;


#[derive(Debug, Clone, diesel::Queryable, diesel::Selectable, diesel::Identifiable)]
#[diesel(table_name = crate::schema::boat)]

pub struct Boat {
    id: BoatId,
    name: String,
    weight_class: WeightClass,
    seat_count: SeatCount,
    has_cox: HasCox,
    oars_per_seat: OarsPerSeat,
    acquired_at: Option<chrono::NaiveDate>,
    manufactured_at: Option<chrono::NaiveDate>
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::boat)]
pub struct NewBoat {
    name: String,
    weight_class: WeightClass,
    seat_count: SeatCount,
    has_cox: HasCox,
    oars_per_seat: OarsPerSeat,
    acquired_at: Option<chrono::NaiveDate>,
    manufactured_at: Option<chrono::NaiveDate>
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoatOrder {
    Size(DbOrdering),
}

pub struct BoatFilter {
    boat_type: Option<BoatType>,
    coxed: Option<bool>,
    oars: Option<OarsPerSeat> 
}

pub enum BoatFilter2 {
    None,
    ByType(BoatType),
    OarConfig(OarConfiguration)
}

impl Boat {
    pub fn new_boat(conn: &mut SqliteConnection, boat: NewBoat) -> Result<Boat, diesel::result::Error> {
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

                boat::table.filter(boat::has_cox.eq(cox).and(boat::seat_count.eq(seats).and(boat::oars_per_seat.eq(oars)))).get_results(conn)
            },
            BoatFilter2::OarConfig(config) => {
                let oars = config.num_oars();
                boat::table.filter(boat::oars_per_seat.eq(oars)).get_results(conn)
            }
        } 
    }

    pub fn get_boat(
        conn: &mut SqliteConnection,
        id: BoatId 
    ) -> Result<Boat, diesel::result::Error> {
        use crate::schema::boat;
        use diesel::OptionalExtension;
        boat::table.filter(boat::id.eq(id)).get_result(conn)
    }

}


#[derive(Debug, Clone, 
    diesel::Queryable
)]
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
        id: BoatId 
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::{boat, issue, use_event};
        let ago_30_d = chrono::Utc::now().naive_utc() - chrono::TimeDelta::days(30);
        boat::table.filter(boat::id.eq(id))
        .select((
            Boat::as_select(), 
            issue::table.select(diesel::dsl::count(issue::boat_id.eq(id).and(issue::resolved_at.is_null()))).single_value(),
            use_event::table.select(diesel::dsl::count(use_event::boat_id.eq(id))).single_value(),
            use_event::table.select(diesel::dsl::count(use_event::boat_id.eq(id).and(use_event::recorded_at.gt(ago_30_d)))).single_value(),
        ))
        .get_result::<BoatAndStats>(conn)
    }
}