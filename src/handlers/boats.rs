use axum::{
    extract::{Path, State},
    response::{Html, Response, IntoResponse},
    http::{StatusCode, header, HeaderMap},
    Form,
};
use serde::Deserialize;
use maud::html;
use crate::{
    db::{
        boat::{Boat, BoatAndStats, NewBoat, types::{BoatId, BoatType, WeightClass}},
        use_event::UseEvent,
    },
    ui::state::AppState,
    templates,
};

/// Handler for boat list page
pub async fn boat_list_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let boats = conn
        .interact(|conn| BoatAndStats::get_boats(conn))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get boats: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::debug!("Retrieved {} boats", boats.len());
    Ok(Html(templates::boats::boat_list_page(&boats).into_string()))
}

/// Form data for creating/updating a boat
#[derive(Debug, Deserialize)]
pub struct BoatFormInput {
    pub name: String,
    pub weight_class: String,
    pub boat_type: Option<String>,
    pub acquired_at: Option<String>,
    pub manufactured_at: Option<String>,
    pub relinquished_at: Option<String>,
}

/// Handler for new boat form page
pub async fn new_boat_handler() -> Html<String> {
    let data = templates::boats::form::BoatFormData::empty();
    let errors = templates::boats::form::BoatFormErrors::default();
    Html(templates::boats::form::boat_form_page(
        templates::boats::form::BoatFormMode::New,
        data,
        errors,
    ).into_string())
}

/// Handler for creating a new boat
pub async fn create_boat_handler(
    State(state): State<AppState>,
    Form(input): Form<BoatFormInput>,
) -> Result<impl IntoResponse, Html<String>> {
    // Validate input
    let mut errors = templates::boats::form::BoatFormErrors::default();

    if input.name.trim().is_empty() {
        errors.name = Some("Boat name is required".to_string());
    }

    let weight_class = if input.weight_class.is_empty() {
        errors.weight_class = Some("Weight class is required".to_string());
        None
    } else {
        match input.weight_class.as_str() {
            "Light" => Some(WeightClass::Light),
            "Medium" => Some(WeightClass::Medium),
            "Heavy" => Some(WeightClass::Heavy),
            "Tubby" => Some(WeightClass::Tubby),
            _ => {
                errors.weight_class = Some("Invalid weight class".to_string());
                None
            }
        }
    };

    let boat_type = input.boat_type.as_ref().and_then(|bt| match bt.as_str() {
        "" => None,
        "Single" => Some(BoatType::Single),
        "Double" => Some(BoatType::Double),
        "Quad" => Some(BoatType::Quad),
        "QuadPlus" => Some(BoatType::QuadPlus),
        "Pair" => Some(BoatType::Pair),
        "Four" => Some(BoatType::Four),
        "FourPlus" => Some(BoatType::FourPlus),
        "Eight" => Some(BoatType::Eight),
        _ => None,
    });

    // Parse dates
    let acquired_at = input.acquired_at.as_ref().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
        }
    });

    let manufactured_at = input.manufactured_at.as_ref().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
        }
    });

    if errors.has_errors() {
        let data = templates::boats::form::BoatFormData {
            name: input.name,
            weight_class,
            boat_type,
            acquired_at: input.acquired_at.unwrap_or_default(),
            manufactured_at: input.manufactured_at.unwrap_or_default(),
            relinquished_at: None,
        };
        return Err(Html(templates::boats::form::boat_form_page(
            templates::boats::form::BoatFormMode::New,
            data,
            errors,
        ).into_string()));
    }

    // Create boat
    let weight_class = weight_class.unwrap();
    let boat_type = boat_type.unwrap_or(BoatType::Single); // Default to Single if not specified
    let new_boat = NewBoat::new(input.name, weight_class, boat_type, acquired_at, manufactured_at);

    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            Html("Database connection error".to_string())
        })?;

    conn.interact(move |conn| {
        Boat::new_boat(conn, new_boat)
    })
    .await
    .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            Html("Database error".to_string())
        })?
    .map_err(|e| {
        tracing::error!("Failed to create boat: {}", e);
        Html("Failed to create boat".to_string())
    })?;

    tracing::info!("Successfully created boat");

    // Get boats list to return
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            Html("Database connection error".to_string())
        })?;

    let boats = conn
        .interact(|conn| BoatAndStats::get_boats(conn))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            Html("Database error".to_string())
        })?
        .map_err(|e| {
            tracing::error!("Failed to get boats: {}", e);
            Html("Failed to get boats".to_string())
        })?;

    // Return content portion with toast as OOB swap
    let mut headers = HeaderMap::new();
    headers.insert("HX-Push-Url", "/boats".parse().unwrap());

    Ok((
        headers,
        Html(html! {
            (templates::boats::list::boat_list_content(&boats))
            (crate::templates::components::toast::success_toast("Boat created successfully!"))
        }.into_string())
    ))
}

/// Handler for boat detail page
pub async fn boat_detail_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>, StatusCode> {
    let boat_id = BoatId::new(id);
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let boat = conn
        .interact(move |conn| BoatAndStats::get_boat(conn, boat_id))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::warn!("Boat not found: {}", e);
            StatusCode::NOT_FOUND
        })?;

    tracing::debug!("Retrieved boat details for {}", boat.boat.name);
    Ok(Html(templates::boats::detail::boat_detail_page(&boat).into_string()))
}

/// Handler for edit boat form page
pub async fn edit_boat_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>, StatusCode> {
    let boat_id = BoatId::new(id);
    let conn = state.pool().get().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let boat = conn
        .interact(move |conn| Boat::get_boat(conn, boat_id))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::warn!("Boat not found: {}", e);
            StatusCode::NOT_FOUND
        })?;

    let data = templates::boats::form::BoatFormData::from_boat(&boat);
    let errors = templates::boats::form::BoatFormErrors::default();

    Ok(Html(templates::boats::form::boat_form_page(
        templates::boats::form::BoatFormMode::Edit(boat_id),
        data,
        errors,
    ).into_string()))
}

/// Handler for updating an existing boat
pub async fn update_boat_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(input): Form<BoatFormInput>,
) -> Result<impl IntoResponse, Html<String>> {
    let boat_id = BoatId::new(id);

    // Validate input (similar to create)
    let mut errors = templates::boats::form::BoatFormErrors::default();

    if input.name.trim().is_empty() {
        errors.name = Some("Boat name is required".to_string());
    }

    let weight_class = if input.weight_class.is_empty() {
        errors.weight_class = Some("Weight class is required".to_string());
        None
    } else {
        match input.weight_class.as_str() {
            "Light" => Some(WeightClass::Light),
            "Medium" => Some(WeightClass::Medium),
            "Heavy" => Some(WeightClass::Heavy),
            "Tubby" => Some(WeightClass::Tubby),
            _ => {
                errors.weight_class = Some("Invalid weight class".to_string());
                None
            }
        }
    };

    let boat_type = input.boat_type.as_ref().and_then(|bt| match bt.as_str() {
        "" => None,
        "Single" => Some(BoatType::Single),
        "Double" => Some(BoatType::Double),
        "Quad" => Some(BoatType::Quad),
        "QuadPlus" => Some(BoatType::QuadPlus),
        "Pair" => Some(BoatType::Pair),
        "Four" => Some(BoatType::Four),
        "FourPlus" => Some(BoatType::FourPlus),
        "Eight" => Some(BoatType::Eight),
        _ => None,
    });

    let acquired_at = input.acquired_at.as_ref().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
        }
    });

    let manufactured_at = input.manufactured_at.as_ref().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
        }
    });

    let relinquished_at = input.relinquished_at.as_ref().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
        }
    });

    if errors.has_errors() {
        let data = templates::boats::form::BoatFormData {
            name: input.name,
            weight_class,
            boat_type,
            acquired_at: input.acquired_at.unwrap_or_default(),
            manufactured_at: input.manufactured_at.unwrap_or_default(),
            relinquished_at: input.relinquished_at,
        };
        return Err(Html(templates::boats::form::boat_form_page(
            templates::boats::form::BoatFormMode::Edit(boat_id),
            data,
            errors,
        ).into_string()));
    }

    // Update boat - first get the existing boat, then update it
    let weight_class = weight_class.unwrap();
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            Html("Database connection error".to_string())
        })?;

    // Get existing boat
    let mut boat = conn
        .interact(move |conn| Boat::get_boat(conn, boat_id))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            Html("Database error".to_string())
        })?
        .map_err(|e| {
            tracing::warn!("Boat not found: {}", e);
            Html("Boat not found".to_string())
        })?;

    // Update fields
    boat.name = input.name;
    boat.weight_class = weight_class;

    if let Some(bt) = boat_type {
        let (has_cox, seat_count, oars_per_seat) = bt.into_values();
        boat.has_cox = has_cox;
        boat.seat_count = seat_count;
        boat.oars_per_seat = oars_per_seat;
    }

    boat.acquired_at = acquired_at;
    boat.manufactured_at = manufactured_at;
    boat.relinquished_at = relinquished_at;

    // Save updated boat
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            Html("Database connection error".to_string())
        })?;

    conn.interact(move |conn| {
        Boat::update_boat(conn, &boat)
    })
    .await
    .map_err(|e| {
        tracing::error!("Database interaction error: {}", e);
        Html("Database error".to_string())
    })?
    .map_err(|e| {
        tracing::error!("Failed to update boat: {}", e);
        Html("Failed to update boat".to_string())
    })?;

    tracing::info!("Successfully updated boat {}", boat_id.as_int());

    // Get boats list to return
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            Html("Database connection error".to_string())
        })?;

    let boats = conn
        .interact(|conn| BoatAndStats::get_boats(conn))
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            Html("Database error".to_string())
        })?
        .map_err(|e| {
            tracing::error!("Failed to get boats: {}", e);
            Html("Failed to get boats".to_string())
        })?;

    // Return content portion with toast as OOB swap
    let mut headers = HeaderMap::new();
    headers.insert("HX-Push-Url", "/boats".parse().unwrap());

    Ok((
        headers,
        Html(html! {
            (templates::boats::list::boat_list_content(&boats))
            (crate::templates::components::toast::success_toast("Boat updated successfully!"))
        }.into_string())
    ))
}

/// Handler for daily usage chart (30 days)
pub async fn daily_chart_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response, StatusCode> {
    let boat_id = BoatId::new(id);
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get last 30 days of data
    let start = chrono::Utc::now().naive_local() - chrono::TimeDelta::try_days(30).unwrap();
    let data = conn
        .interact(move |conn| {
            UseEvent::daily_timeseries_for_boat(conn, boat_id, start, None)
        })
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get daily timeseries: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Convert usize to i64 for charting
    let data: Vec<(chrono::NaiveDate, i64)> = data.into_iter()
        .map(|(date, count)| (date, count as i64))
        .collect();

    // Generate SVG
    let svg = templates::boats::charts::monthly_usage_chart(&data)
        .map_err(|e| {
            tracing::error!("Failed to generate chart: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Return SVG response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .body(svg.into())
        .unwrap())
}

/// Handler for monthly usage chart (12 months)
pub async fn monthly_chart_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response, StatusCode> {
    let boat_id = BoatId::new(id);
    let conn = state.pool().get().await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get last 12 months of data
    let start = chrono::Utc::now().naive_local() - chrono::TimeDelta::try_days(365).unwrap();
    let data = conn
        .interact(move |conn| {
            UseEvent::monthly_timeseries_for_boat(conn, boat_id, start, None)
        })
        .await
        .map_err(|e| {
            tracing::error!("Database interaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            tracing::error!("Failed to get monthly timeseries: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Convert usize to i64 for charting
    let data: Vec<(chrono::NaiveDate, i64)> = data.into_iter()
        .map(|(date, count)| (date, count as i64))
        .collect();

    // Generate SVG
    let svg = templates::boats::charts::yearly_usage_chart(&data)
        .map_err(|e| {
            tracing::error!("Failed to generate chart: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Return SVG response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .body(svg.into())
        .unwrap())
}
