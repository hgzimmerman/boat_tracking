use axum::{
    extract::{Path, State, FromRequest, Request, Form},
    response::{Html, IntoResponse},
    http::StatusCode,
    body::Bytes,
};
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
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read body: {}", e)))?;

        let body_str = std::str::from_utf8(&body)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UTF-8 in form data".to_string()))?;

        let value = serde_qs::Config::new(5, false)
            .deserialize_str(body_str)
            .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("Failed to parse form: {}", e)))?;

        Ok(QsForm(value))
    }
}
use crate::{
    db::{
        boat::types::BoatId,
        state::AppState,
        use_scenario::{UseScenario, UseScenarioId},
        use_event_batch::{BatchId, NewBatch, NewBatchArgs, UseEventBatch},
    },
    templates,
};

/// Handler for batch list page
pub async fn batch_list_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let (batches, scenarios) = conn
        .interact(|conn| {
            let batches = UseEventBatch::get_most_recent_batches_and_their_use_count(conn, None, 0, 100)?;
            let scenarios = UseScenario::get_all(conn)?;
            Ok::<_, diesel::result::Error>((batches, scenarios))
        })
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get batches: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::debug!("Retrieved {} batches", batches.len());
    Ok(Html(templates::batches::list::batch_list_page(&batches, &scenarios).into_string()))
}

/// Query parameters for batch creation page
#[derive(Debug, Deserialize)]
pub struct NewBatchQuery {
    pub template: Option<BatchId>,
}

/// Handler for new batch creation page
pub async fn new_batch_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<NewBatchQuery>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // If template ID is provided, fetch boats from that batch
    let template_boats = if let Some(batch_id) = query.template {
        conn.interact(move |conn| {
            UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)
        })
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok() // Convert Result to Option, ignoring errors for template
    } else {
        None
    };

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

    Ok(Html(templates::batches::creation::batch_creation_page(&scenarios, template_boats.as_deref()).into_string()))
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
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
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
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            Html("<p>Database error</p>".to_string())
        })?
        .map_err(|e| {
            tracing::error!("Failed to create batch: {}", e);
            Html("<p>Failed to create batch</p>".to_string())
        })?;

    // Redirect to batch list using HX-Redirect header
    use axum::response::Response;
    use axum::http::header;

    let mut response = Response::new(String::new());
    response.headers_mut().insert(
        header::HeaderName::from_static("hx-redirect"),
        header::HeaderValue::from_static("/batches")
    );

    Ok(response)
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
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let boats = conn
        .interact(crate::db::boat::BoatAndStats::get_boats)
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get boats: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(templates::batches::creation::boat_search_results(&boats, None).into_string()))
}

/// Handler for searching boats (HTMX endpoint)
pub async fn search_boats_handler(
    State(state): State<AppState>,
    Form(input): Form<BoatSearchInput>,
) -> Result<Html<String>, StatusCode> {

    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let boats = conn
        .interact(crate::db::boat::BoatAndStats::get_boats)
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get boats: {}", e);
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

            matches_search && matches_weight && matches_oars && matches_cox && matches_boat_type
        })
        .collect();

    Ok(Html(templates::batches::creation::boat_search_results(
        &filtered_boats,
        input.search.as_deref(),
    ).into_string()))
}

/// Handler for adding a boat to the session (HTMX endpoint)
pub async fn add_boat_to_session_handler(
    State(_state): State<AppState>,
    Path(boat_id): Path<BoatId>,
) -> Result<Html<String>, StatusCode> {
    // TODO: Implement session management with cookies/session storage
    // For now, return a placeholder
    Ok(Html(format!("<p>Added boat {} to session</p>", boat_id)))
}

/// Handler for removing a boat from the session (HTMX endpoint)
pub async fn remove_boat_from_session_handler(
    State(_state): State<AppState>,
    Path(boat_id): Path<BoatId>,
) -> Result<Html<String>, StatusCode> {
    // TODO: Implement session management with cookies/session storage
    Ok(Html(format!("<p>Removed boat {} from session</p>", boat_id)))
}

/// Handler for batch detail page
pub async fn batch_detail_handler(
    State(state): State<AppState>,
    Path(batch_id): Path<BatchId>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
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
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get batch details: {}", e);
            if matches!(e, diesel::result::Error::NotFound) {
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
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch boats for this batch
    let boats = conn
        .interact(move |conn| {
            UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)
        })
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get boats for batch: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Extract just the boat names
    let boat_names: Vec<String> = boats.into_iter()
        .map(|(_event, boat)| boat.name)
        .collect();

    Ok(Html(templates::batches::list::boats_preview_popup(&boat_names).into_string()))
}
