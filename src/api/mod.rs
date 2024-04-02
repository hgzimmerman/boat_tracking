use crate::{
    db::{boat::BoatAndStats, use_event::UseEvent},
    ui::state::AppState,
};

use self::wire::{BoatSummaryCsvRow, BoatUseCsvRow, CsvExportParams};

pub mod wire;

pub async fn export_uses_csv_handler(
    state: AppState,
    axum::extract::Query(params): axum::extract::Query<CsvExportParams>,
) -> Result<CsvRows<BoatUseCsvRow>, String> {
    let conn = state.pool().get().await.map_err(|e| e.to_string())?;
    let rows = conn
        .interact(move |conn| {
            UseEvent::export_events(conn, params.start, params.end, params.id.map(|x| vec![x]))
        })
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    Ok(CsvRows(rows))
}

pub async fn export_boats_csv_handler(
    state: AppState,
) -> Result<CsvRows<BoatSummaryCsvRow>, String> {
    let conn = state.pool().get().await.map_err(|e| e.to_string())?;
    let rows = conn
        .interact(move |conn| {
            BoatAndStats::get_boats(conn).map(|boats| {
                let convert_to_u64 = |x: i64| u64::try_from(x).ok();
                boats
                    .into_iter()
                    .map(
                        |BoatAndStats {
                             boat,
                             open_issues,
                             total_uses,
                             uses_last_thirty_days: _,
                         }| {
                            BoatSummaryCsvRow {
                                boat_id: boat.id,
                                boat_type: boat.boat_type(),
                                boat_name: boat.name,
                                boat_weight_class: boat.weight_class,
                                manufactured_at: boat.manufactured_at,
                                acquired_at: boat.acquired_at,
                                relinquished_at: boat.relinquished_at,
                                total_uses: total_uses.and_then(convert_to_u64).unwrap_or_default(),
                                open_issues: open_issues
                                    .and_then(convert_to_u64)
                                    .unwrap_or_default(),
                            }
                        },
                    )
                    .collect()
            })
        })
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    Ok(CsvRows(rows))
}

pub struct CsvRows<T>(Vec<T>);

impl<T> axum::response::IntoResponse for CsvRows<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match serialize(self.0) {
            Ok(string) => (
                [(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static("text/csv"),
                )],
                string,
            )
                .into_response(),
            Err(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static("text/plain;charset=UTF-8"),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

fn serialize<T: serde::Serialize>(
    rows: Vec<T>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(Vec::with_capacity(2048));

    for row in rows {
        writer.serialize(row)?;
    }
    let data = String::from_utf8(writer.into_inner()?)?;
    Ok(data)
}
