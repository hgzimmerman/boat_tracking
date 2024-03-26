use chrono::{NaiveDate, NaiveDateTime};
use crate::db::{boat::types::{BoatType, WeightClass}, use_event::UseScenario};


#[derive(Debug, Clone, serde::Serialize)]
pub struct CsvRow {
    pub boat_name: String,
    pub boat_type: BoatType,
    pub boat_weight_class: WeightClass,
    pub acquired_at: NaiveDate,
    pub used_at: NaiveDateTime,
    pub use_scenario: UseScenario
}
