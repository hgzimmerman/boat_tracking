use axum::{
    extract::{State, Path},
    response::{Html, IntoResponse, Response},
    http::{StatusCode, header},
    Form,
};
use chrono::TimeZone;
use serde::Deserialize;
use crate::{
    db::{
        issue::{Issue, NewIssue, IssueId},
        boat::{Boat, BoatFilter, types::BoatId},
        state::AppState,
        DbOrdering,
    },
    templates,
};

/// Handler for issue list page
pub async fn issue_list_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let issues = conn
        .interact(|conn| Issue::get_all_issues_with_boats(conn, DbOrdering::Desc))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get issues: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(templates::issues::issue_list_page(&issues).into_string()))
}

/// Handler for new issue page
pub async fn new_issue_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let filter = BoatFilter {
        _x: 0,
        num_seats: None,
        coxed: None,
        oars_config: None,
    };

    let boats = conn
        .interact(move |conn| Boat::get_filtered_boats(conn, filter, None))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get boats: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(templates::issues::new_issue_page(&boats).into_string()))
}

/// Form data for creating an issue
#[derive(Debug, Deserialize)]
pub struct IssueFormInput {
    pub boat_id: Option<BoatId>,
    pub note: String,
    pub recorded_at: Option<String>,
}

/// Handler for creating a new issue
pub async fn create_issue_handler(
    State(state): State<AppState>,
    Form(input): Form<IssueFormInput>,
) -> Result<impl IntoResponse, Html<String>> {
    // Validate note is not empty
    if input.note.trim().is_empty() {
        return Err(Html("<p>Issue description cannot be empty</p>".to_string()));
    }

    // Parse datetime (local time from form) and convert to UTC, or use current time
    let recorded_at = if let Some(dt_str) = input.recorded_at {
        if dt_str.is_empty() {
            chrono::Utc::now()
        } else {
            let naive = chrono::NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%dT%H:%M")
                .map_err(|_| Html("<p>Invalid datetime format</p>".to_string()))?;
            chrono::Local.from_local_datetime(&naive).single()
                .ok_or_else(|| Html("<p>Ambiguous or invalid local datetime</p>".to_string()))?
                .with_timezone(&chrono::Utc)
        }
    } else {
        chrono::Utc::now()
    };

    // Create the issue
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            Html("<p>Database connection error</p>".to_string())
        })?;

    let new_issue = NewIssue {
        boat_id: input.boat_id,
        use_event_id: None,
        recorded_at,
        note: input.note,
        resolved_at: None,
    };

    let _issue = conn
        .interact(|conn| Issue::add_issue(conn, new_issue))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            Html("<p>Database error</p>".to_string())
        })?
        .map_err(|e| {
            tracing::error!("Failed to create issue: {}", e);
            Html("<p>Failed to create issue</p>".to_string())
        })?;

    // Redirect to issue list using HX-Redirect header
    let mut response = Response::new(String::new());
    response.headers_mut().insert(
        header::HeaderName::from_static("hx-redirect"),
        header::HeaderValue::from_static("/issues")
    );

    Ok(response)
}

/// Handler for resolving an issue
pub async fn resolve_issue_handler(
    State(state): State<AppState>,
    Path(issue_id): Path<IssueId>,
) -> Result<impl IntoResponse, Html<String>> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            Html("<p>Database connection error</p>".to_string())
        })?;

    let _issue = conn
        .interact(move |conn| Issue::resolve_issue(conn, issue_id))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            Html("<p>Database error</p>".to_string())
        })?
        .map_err(|e| {
            tracing::error!("Failed to resolve issue: {}", e);
            Html("<p>Failed to resolve issue</p>".to_string())
        })?;

    // Redirect to issue list using HX-Redirect header
    let mut response = Response::new(String::new());
    response.headers_mut().insert(
        header::HeaderName::from_static("hx-redirect"),
        header::HeaderValue::from_static("/issues")
    );

    Ok(response)
}

/// Handler for unresolving an issue
pub async fn unresolve_issue_handler(
    State(state): State<AppState>,
    Path(issue_id): Path<IssueId>,
) -> Result<impl IntoResponse, Html<String>> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            Html("<p>Database connection error</p>".to_string())
        })?;

    let _issue = conn
        .interact(move |conn| Issue::unresolve_issue(conn, issue_id))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            Html("<p>Database error</p>".to_string())
        })?
        .map_err(|e| {
            tracing::error!("Failed to unresolve issue: {}", e);
            Html("<p>Failed to unresolve issue</p>".to_string())
        })?;

    // Redirect to issue list using HX-Redirect header
    let mut response = Response::new(String::new());
    response.headers_mut().insert(
        header::HeaderName::from_static("hx-redirect"),
        header::HeaderValue::from_static("/issues")
    );

    Ok(response)
}
