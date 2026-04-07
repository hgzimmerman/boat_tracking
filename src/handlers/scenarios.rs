use axum::{
    extract::{Path, State, Form},
    response::Html,
    http::StatusCode,
};
use chrono::NaiveTime;
use serde::Deserialize;

use crate::{
    db::{
        state::AppState,
        use_scenario::{NewUseScenario, UseScenario, UseScenarioChangeset, UseScenarioId},
    },
    templates,
};

/// Handler for scenario list page
pub async fn scenario_list_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let scenarios = conn
        .interact(UseScenario::get_all)
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get scenarios: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(templates::scenarios::list::scenario_list_page(&scenarios).into_string()))
}

/// Handler for new scenario form
pub async fn new_scenario_handler() -> Html<String> {
    Html(templates::scenarios::form::new_scenario_page().into_string())
}

/// Form data for creating/updating a scenario
#[derive(Debug, Deserialize)]
pub struct ScenarioFormInput {
    pub name: String,
    pub default_time: Option<String>,
}

fn parse_default_time(input: &Option<String>) -> Option<NaiveTime> {
    input
        .as_deref()
        .filter(|t| !t.is_empty())
        .and_then(|t| NaiveTime::parse_from_str(t, "%H:%M").ok())
}

/// Handler for creating a new scenario
pub async fn create_scenario_handler(
    State(state): State<AppState>,
    Form(input): Form<ScenarioFormInput>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let default_time = parse_default_time(&input.default_time);

    conn
        .interact(move |conn| {
            UseScenario::create(
                conn,
                NewUseScenario {
                    name: input.name,
                    default_time,
                },
            )
        })
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to create scenario: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Re-fetch and render the list
    let scenarios = conn
        .interact(UseScenario::get_all)
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get scenarios: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(templates::scenarios::list::scenario_list_page(&scenarios).into_string()))
}

/// Handler for edit scenario form
pub async fn edit_scenario_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>, StatusCode> {
    let scenario_id = UseScenarioId::new(id);

    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let scenario = conn
        .interact(move |conn| UseScenario::get_by_id(conn, scenario_id))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get scenario: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Html(templates::scenarios::form::edit_scenario_page(&scenario).into_string()))
}

/// Handler for updating an existing scenario
pub async fn update_scenario_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(input): Form<ScenarioFormInput>,
) -> Result<Html<String>, StatusCode> {
    let scenario_id = UseScenarioId::new(id);
    let default_time = parse_default_time(&input.default_time);

    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    conn
        .interact(move |conn| {
            UseScenario::update(
                conn,
                scenario_id,
                UseScenarioChangeset {
                    name: Some(input.name),
                    default_time: Some(default_time),
                },
            )
        })
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to update scenario: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Re-fetch and render the list
    let scenarios = conn
        .interact(UseScenario::get_all)
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get scenarios: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(templates::scenarios::list::scenario_list_page(&scenarios).into_string()))
}
