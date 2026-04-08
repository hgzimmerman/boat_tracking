mod session;

use axum::{
    extract::{Path, Query, State, FromRequest, Request, Form},
    response::{Html, IntoResponse},
    http::StatusCode,
    body::Bytes,
};
use axum_htmx::HxRequest;
use chrono::TimeZone;
use serde::Deserialize;

/// Custom form extractor that uses serde_qs to parse form data with array notation
pub struct QsForm<T>(pub T);

impl<T, S> FromRequest<S> for QsForm<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("Failed to read body: {error}")))?;

        let body_str = std::str::from_utf8(&body)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UTF-8 in form data".to_string()))?;

        let value = serde_qs::Config::new(5, false)
            .deserialize_str(body_str)
            .map_err(|error| (StatusCode::UNPROCESSABLE_ENTITY, format!("Failed to parse form: {error}")))?;

        Ok(QsForm(value))
    }
}
use axum_extra::extract::cookie::SignedCookieJar;
use crate::{
    db::{
        boat::{Boat, types::BoatId},
        state::AppState,
        use_scenario::{UseScenario, UseScenarioId},
        use_event_batch::{BatchId, NewBatch, NewBatchArgs, UseEventBatch},
    },
    handlers::batches::session::{read_selected_boats, write_selected_boats, clear_selected_boats},
    templates,
};

/// Handler for batch list page
pub async fn batch_list_handler(
    State(state): State<AppState>,
    hx_request: HxRequest,
    Query(pagination): Query<super::PaginationParams>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|error| {
            tracing::error!(?error, "Failed to get database connection");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let offset = pagination.offset() as usize;
    let limit = pagination.per_page as usize;

    let (batches, scenarios, total_count) = conn
        .interact(move |conn| {
            let batches = UseEventBatch::get_most_recent_batches_and_their_use_count(conn, None, offset, limit)?;
            let scenarios = UseScenario::get_all(conn)?;
            let total_count = UseEventBatch::count_batches(conn, None)?;
            Ok::<_, diesel::result::Error>((batches, scenarios, total_count))
        })
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to get batches");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let meta = pagination.metadata(total_count);
    tracing::debug!("Retrieved {} batches (page {}/{})", batches.len(), meta.current_page, meta.total_pages);
    let content = templates::batches::list::batch_list_content(&batches, &scenarios, &meta);
    Ok(super::maybe_page("Boat Uses", content, hx_request))
}

/// Query parameters for batch creation page
#[derive(Debug, Deserialize)]
pub struct NewBatchQuery {
    pub template: Option<BatchId>,
}

/// Handler for new batch creation page
pub async fn new_batch_handler(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    axum::extract::Query(query): axum::extract::Query<NewBatchQuery>,
) -> Result<(SignedCookieJar, Html<String>), StatusCode> {
    let conn = state.pool().get().await
        .map_err(|error| {
            tracing::error!(?error, "Failed to get database connection");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Read existing selection from cookie
    let mut selected = read_selected_boats(&jar);

    // If template ID is provided and no boats are currently selected, populate from template
    let jar = if let Some(batch_id) = query.template {
        if selected.boat_ids.is_empty() {
            let template_boats = conn.interact(move |conn| {
                UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)
            })
            .await
            .map_err(|error| {
                tracing::error!(?error, "Database interaction error");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok();

            if let Some(boats) = template_boats {
                selected.boat_ids = boats.iter().map(|(_event, boat)| boat.id).collect();
                write_selected_boats(jar, &selected)
            } else {
                jar
            }
        } else {
            jar
        }
    } else {
        jar
    };

    // Fetch boat records for selected IDs
    let selected_boats = if selected.boat_ids.is_empty() {
        Vec::new()
    } else {
        conn.interact(move |conn| Boat::get_boats_by_ids(conn, &selected.boat_ids))
            .await
            .map_err(|error| {
                tracing::error!(?error, "Database interaction error");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .map_err(|error| {
                tracing::error!(?error, "Failed to get selected boats");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };

    let scenarios = conn
        .interact(UseScenario::get_all)
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to get scenarios");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((jar, Html(templates::batches::creation::batch_creation_page(&scenarios, &selected_boats).into_string())))
}

/// Form data for creating a batch
#[derive(Debug, Deserialize)]
pub struct BatchFormInput {
    pub use_scenario_id: UseScenarioId,
    pub recorded_at: Option<String>,
    pub boat_ids: Vec<BoatId>,
}

/// Handler for creating a new batch
pub async fn create_batch_handler(
    State(state): State<AppState>,
    hx_request: HxRequest,
    jar: SignedCookieJar,
    QsForm(input): QsForm<BatchFormInput>,
) -> Result<impl IntoResponse, Html<String>> {
    // Parse datetime (local time from form) and convert to UTC, or use current time
    let recorded_at = if let Some(dt_str) = input.recorded_at {
        let naive = chrono::NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%dT%H:%M")
            .map_err(|_| Html("<p>Invalid datetime format</p>".to_string()))?;
        chrono::Local.from_local_datetime(&naive).single()
            .ok_or_else(|| Html("<p>Ambiguous or invalid local datetime</p>".to_string()))?
            .with_timezone(&chrono::Utc)
    } else {
        chrono::Utc::now()
    };

    if input.boat_ids.is_empty() {
        return Err(Html("<p>At least one boat must be selected</p>".to_string()));
    }

    // Create the batch
    let conn = state.pool().get().await
        .map_err(|error| {
            tracing::error!(?error, "Failed to get database connection");
            Html("<p>Database connection error</p>".to_string())
        })?;

    let new_batch = NewBatchArgs {
        boat_ids: input.boat_ids,
        batch: NewBatch {
            use_scenario_id: input.use_scenario_id,
            recorded_at,
        },
    };

    let _batch_id = conn
        .interact(|conn| UseEventBatch::create_batch(conn, new_batch))
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            Html("<p>Database error</p>".to_string())
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to create batch");
            Html("<p>Failed to create batch</p>".to_string())
        })?;

    // Clear the selected boats cookie and return the batch list
    let cleared_jar = clear_selected_boats(jar);

    let (batches, scenarios, total_count) = conn
        .interact(|conn| {
            let batches = UseEventBatch::get_most_recent_batches_and_their_use_count(conn, None, 0, 50)?;
            let scenarios = UseScenario::get_all(conn)?;
            let total_count = UseEventBatch::count_batches(conn, None)?;
            Ok::<_, diesel::result::Error>((batches, scenarios, total_count))
        })
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            Html("<p>Database error</p>".to_string())
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to get batches");
            Html("<p>Failed to get batches</p>".to_string())
        })?;

    let meta = super::PaginationParams { page: 1, per_page: 50 }.metadata(total_count);
    let content = templates::batches::list::batch_list_content(&batches, &scenarios, &meta);

    let mut headers = axum::http::HeaderMap::new();
    headers.insert("HX-Push-Url", "/batches".parse().unwrap());

    Ok((cleared_jar, headers, super::maybe_page("Boat Uses", content, hx_request)))
}

/// Cox filter for boat search
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum CoxFilter {
    #[serde(rename = "coxed")]
    Coxed,
    #[serde(rename = "coxless")]
    Coxless,
}

/// Deserialize empty string as None for optional enum fields
fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        T::deserialize(serde::de::value::StringDeserializer::new(s)).map(Some)
    }
}

/// Form data for boat search
#[derive(Debug, Deserialize)]
pub struct BoatSearchInput {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub search: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub filter_weight: Option<crate::db::boat::types::WeightClass>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub filter_oars: Option<crate::db::boat::types::OarConfiguration>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub filter_cox: Option<CoxFilter>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub filter_boat_type: Option<crate::db::boat::types::BoatType>,
}

/// Handler for listing all boats (HTMX endpoint)
pub async fn list_boats_handler(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<Html<String>, StatusCode> {
    let selected = read_selected_boats(&jar);

    let conn = state.pool().get().await
        .map_err(|error| {
            tracing::error!(?error, "Failed to get database connection");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let boats = conn
        .interact(crate::db::boat::BoatAndStats::get_boats)
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to get boats");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let boats: Vec<_> = boats.into_iter()
        .filter(|boat| !selected.boat_ids.contains(&boat.boat.id))
        .collect();

    Ok(Html(templates::batches::creation::boat_search_results(&boats, None).into_string()))
}

/// Handler for searching boats (HTMX endpoint)
pub async fn search_boats_handler(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Form(input): Form<BoatSearchInput>,
) -> Result<Html<String>, StatusCode> {
    let selected = read_selected_boats(&jar);

    let conn = state.pool().get().await
        .map_err(|error| {
            tracing::error!(?error, "Failed to get database connection");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let boats = conn
        .interact(crate::db::boat::BoatAndStats::get_boats)
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to get boats");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Apply all filters
    let filtered_boats: Vec<_> = boats.into_iter()
        .filter(|boat| {
            // Text search filter
            let matches_search = input.search
                .as_ref()
                .map(|search| {
                    let search_lower = search.to_lowercase();
                    boat.boat.name.to_lowercase().contains(&search_lower)
                        || boat.boat.weight_class.to_string().to_lowercase().contains(&search_lower)
                        || boat.boat.boat_type()
                            .map(|bt| bt.to_string().to_lowercase().contains(&search_lower))
                            .unwrap_or(false)
                })
                .unwrap_or(true);

            // Weight class filter
            let matches_weight = input.filter_weight
                .map(|weight| boat.boat.weight_class == weight)
                .unwrap_or(true);

            // Oars config filter
            let matches_oars = input.filter_oars
                .map(|oars_config| {
                    boat.boat.boat_type()
                        .map(|bt| {
                            let (_, _, oars_per_seat) = bt.into_values();
                            oars_per_seat.configuration() == oars_config
                        })
                        .unwrap_or(false)
                })
                .unwrap_or(true);

            // Cox filter
            let matches_cox = input.filter_cox
                .map(|cox_filter| {
                    boat.boat.boat_type()
                        .map(|bt| {
                            let (has_cox, _, _) = bt.into_values();
                            match cox_filter {
                                CoxFilter::Coxed => has_cox.as_bool(),
                                CoxFilter::Coxless => !has_cox.as_bool(),
                            }
                        })
                        .unwrap_or(false)
                })
                .unwrap_or(true);

            // Boat type filter
            let matches_boat_type = input.filter_boat_type
                .map(|filter_type| {
                    boat.boat.boat_type()
                        .map(|bt| bt == filter_type)
                        .unwrap_or(false)
                })
                .unwrap_or(true);

            // Exclude already-selected boats
            let not_selected = !selected.boat_ids.contains(&boat.boat.id);

            matches_search && matches_weight && matches_oars && matches_cox && matches_boat_type && not_selected
        })
        .collect();

    Ok(Html(templates::batches::creation::boat_search_results(
        &filtered_boats,
        input.search.as_deref(),
    ).into_string()))
}

/// Handler for adding a boat to the session (HTMX endpoint)
pub async fn add_boat_to_session_handler(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Path(boat_id): Path<BoatId>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut selected = read_selected_boats(&jar);
    selected.add(boat_id);
    let updated_jar = write_selected_boats(jar, &selected);

    let conn = state.pool().get().await
        .map_err(|error| {
            tracing::error!(?error, "Failed to get database connection");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let boats = conn
        .interact(move |conn| Boat::get_boats_by_ids(conn, &selected.boat_ids))
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to get boats");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((
        updated_jar,
        [("HX-Trigger", "boats-changed")],
        Html(templates::batches::creation::selected_boats_fragment(&boats).into_string()),
    ))
}

/// Handler for removing a boat from the session (HTMX endpoint)
pub async fn remove_boat_from_session_handler(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Path(boat_id): Path<BoatId>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut selected = read_selected_boats(&jar);
    selected.remove(boat_id);
    let updated_jar = write_selected_boats(jar, &selected);

    let boats = if selected.boat_ids.is_empty() {
        Vec::new()
    } else {
        let conn = state.pool().get().await
            .map_err(|error| {
                tracing::error!(?error, "Failed to get database connection");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        conn.interact(move |conn| Boat::get_boats_by_ids(conn, &selected.boat_ids))
            .await
            .map_err(|error| {
                tracing::error!(?error, "Database interaction error");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .map_err(|error| {
                tracing::error!(?error, "Failed to get boats");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };

    Ok((
        updated_jar,
        [("HX-Trigger", "boats-changed")],
        Html(templates::batches::creation::selected_boats_fragment(&boats).into_string()),
    ))
}

/// Handler for batch detail page
pub async fn batch_detail_handler(
    State(state): State<AppState>,
    Path(batch_id): Path<BatchId>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|error| {
            tracing::error!(?error, "Failed to get database connection");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch batch metadata, boats used, and scenario name
    let (batch, boats, scenario_name) = conn
        .interact(move |conn| {
            let batch = UseEventBatch::get_batch(conn, batch_id)?
                .ok_or_else(|| diesel::result::Error::NotFound)?;
            let boats = UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)?;
            let scenario_name = UseScenario::get_by_id(conn, batch.use_scenario_id)?
                .map(|s| s.name)
                .unwrap_or_else(|| "Unknown".to_string());
            Ok::<_, diesel::result::Error>((batch, boats, scenario_name))
        })
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to get batch details");
            if matches!(error, diesel::result::Error::NotFound) {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok(Html(templates::batches::detail::batch_detail_page(&batch, &boats, &scenario_name).into_string()))
}

/// Handler for batch boats preview (HTMX hover endpoint)
pub async fn batch_boats_preview_handler(
    State(state): State<AppState>,
    Path(batch_id): Path<BatchId>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|error| {
            tracing::error!(?error, "Failed to get database connection");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch boats for this batch
    let boats = conn
        .interact(move |conn| {
            UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)
        })
        .await
        .map_err(|error| {
            tracing::error!(?error, "Database interaction error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|error| {
            tracing::error!(?error, "Failed to get boats for batch");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Extract just the boat names
    let boat_names: Vec<String> = boats.into_iter()
        .map(|(_event, boat)| boat.name)
        .collect();

    Ok(Html(templates::batches::list::boats_preview_popup(&boat_names).into_string()))
}
