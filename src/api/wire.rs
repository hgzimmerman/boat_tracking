use crate::db::{
    boat::types::{BoatId, BoatType, WeightClass},
    use_event_batch::BatchId,
};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsvExportParams {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub id: Option<BoatId>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BoatUseCsvRow {
    pub boat_id: BoatId,
    pub boat_name: String,
    pub boat_type: BoatType,
    pub boat_weight_class: WeightClass,
    pub used_at: DateTime<Utc>,
    pub batch_id: Option<BatchId>,
    pub use_scenario: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BoatSummaryCsvRow {
    pub boat_id: BoatId,
    pub boat_name: String,
    pub boat_type: Option<BoatType>,
    pub boat_weight_class: WeightClass,
    pub manufactured_at: Option<NaiveDate>,
    pub acquired_at: Option<NaiveDate>,
    pub relinquished_at: Option<NaiveDate>,
    pub total_uses: u64,
    pub open_issues: u64,
}
