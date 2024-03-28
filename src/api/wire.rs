use chrono::{NaiveDate, NaiveDateTime};
use crate::db::{boat::types::{BoatId, BoatType, WeightClass}, use_event::UseScenario, use_event_batch::BatchId};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsvExportParams {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub id: Option<BoatId>
}


#[derive(Debug, Clone, serde::Serialize)]
pub struct CsvRow {
    pub boat_id: BoatId,
    pub boat_name: String,
    pub boat_type: BoatType,
    pub boat_weight_class: WeightClass,
    pub acquired_at: Option<NaiveDate>,
    pub used_at: NaiveDateTime,
    pub batch_id: Option<BatchId>,
    pub use_scenario: UseScenario,
}
