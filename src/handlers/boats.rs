use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use crate::{
    db::boat::BoatAndStats,
    ui::state::AppState,
    templates,
};

/// Handler for boat list page
pub async fn boat_list_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let boats = conn
        .interact(|conn| BoatAndStats::get_boats(conn))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(templates::boats::boat_list_page(&boats).into_string()))
}
