pub mod types;

#[cfg(feature = "ssr")]
pub mod queries;


use types::{HasCox, OarsPerSeat, SeatCount, WeightClass};

use self::types::{BoatAttributes, BoatId, BoatType, OarConfiguration};

use super::DbOrdering;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::Selectable, diesel::Identifiable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::boat))]
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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(diesel::Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::boat))]
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

impl Boat {
    pub fn boat_type(&self) -> Option<BoatType> {
        BoatType::from_boat_attributes(BoatAttributes {
            has_cox: self.has_cox,
            seats: self.seat_count,
            oar_configuation: self.oars_per_seat,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoatOrder {
    Size(DbOrdering),
}

pub struct BoatFilter {
    pub boat_type: Option<BoatType>,
    pub coxed: Option<bool>,
    pub oars: Option<OarsPerSeat>,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct BoatFilter3 {
    /// Currently needed by dioxus to avoid failing to deserialize when all items are empty.
    pub _x: usize,
    pub num_seats: Option<SeatCount>,
    pub coxed: Option<HasCox>,
    pub oars_config: Option<OarConfiguration>,
}

#[derive(Debug, Clone, diesel::Queryable, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BoatAndStats {
    #[diesel(embed)]
    pub boat: Boat,
    pub open_issues: Option<i64>,
    pub total_uses: Option<i64>,
    pub uses_last_thirty_days: Option<i64>,
}
