use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use crate::{
    db::{issue::Issue, DbOrdering},
    ui::state::AppState,
    templates,
};

/// Handler for issue list page
pub async fn issue_list_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let issues = conn
        .interact(|conn| Issue::get_all_issues(conn, DbOrdering::Desc))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(templates::issues::issue_list_page(&issues).into_string()))
}

/// Handler for new issue page (placeholder)
pub async fn new_issue_handler() -> Html<String> {
    Html(templates::issues::new_issue_page().into_string())
}
