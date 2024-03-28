use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::db::{
    boat::{types::BoatId, BoatAndStats},
    issue::Issue,
};
pub mod creation_edit_form;

/* #[derive(Debug, Clone, Copy, Default)]
enum BoatPageMode {
    #[default]
    View,
    Edit
} */

#[server(GetBoat)]
pub(crate) async fn get_boat(id: BoatId) -> Result<BoatAndStats, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    conn.interact(move |conn| BoatAndStats::get_boat(conn, id).map_err(ServerFnError::from))
        .await?
}

#[server(GetBoatOpenIssues)]
pub(crate) async fn get_open_issues_for_boat(id: BoatId) -> Result<Vec<Issue>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    conn.interact(move |conn| {
        Issue::get_open_issues_for_boat(conn, id).map_err(ServerFnError::from)
    })
    .await?
}

#[server(GetBoatEvents)]
pub(crate) async fn get_events_for_boat(id: BoatId) -> Result<Vec<(NaiveDate, f32)>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    conn.interact(move |conn| {
        let start = chrono::Utc::now().naive_local() - chrono::TimeDelta::try_days(30).unwrap();
        crate::db::use_event::UseEvent::timeseries_for_boat(conn, id, start, None)
        .map_err(ServerFnError::from)
        .map(|x| x.into_iter().map(|(date, count)|(date, count as f32)).collect())
    })
    .await?
}

#[server(GetBoatResolvedIssues)]
pub(crate) async fn get_resolved_issues_for_boat(id: BoatId) -> Result<Vec<Issue>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn = state.pool().get().await?;

    conn.interact(move |conn| {
        Issue::get_resolved_issues_for_boat(conn, id).map_err(ServerFnError::from)
    })
    .await?
}

#[component]
pub fn BoatNav() -> Element {
    use dioxus_router::prelude::*;
    use crate::ui::components::Route;
    let path: Route = use_route();
    let id = match path {
        Route::BoatSummary { id }
        | Route::BoatMonthlyUses{ id }
        | Route::BoatYearlyUses{ id }
        | Route::BoatEdit{ id } 
        | Route::BoatIssues{ id } => Some(id),
        _ => None
    }.expect("should be in path where id is known");
    
    let inactive_class = "inline-block p-4 rounded-t-lg hover:text-gray-600 hover:bg-gray-50 dark:hover:bg-gray-800 dark:hover:text-gray-300";
    let active_class = "inline-block p-4 rounded-t-lg hover:text-gray-600 hover:bg-gray-50 dark:hover:bg-gray-800 dark:hover:text-gray-300 active bg-gray-100 rounded-t-lg dark:bg-gray-800 dark:text-blue-500";

    rsx!{
        div { 
            ul { class: "flex flex-wrap text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:border-gray-700 dark:text-gray-400",
                li { class: "me-2",
                    Link {
                        class: if matches!(path, Route::BoatSummary{..}) { active_class } else {inactive_class}, 
                        to: Route::BoatSummary{id},
                        "Summary"
                    }
                }
                li { class: "me-2",
                    Link {
                        // "aria-current": "page",
                        class: if matches!(path, Route::BoatMonthlyUses{..}) { active_class } else {inactive_class}, 
                        to: Route::BoatMonthlyUses{id},
                        "Monthly Usage Chart"
                    }
                }
                li { class: "me-2",
                    Link {
                        class: if matches!(path, Route::BoatYearlyUses{..}) { active_class } else {inactive_class}, 
                        to: Route::BoatYearlyUses{id},
                        "Yearly Usage Chart"
                    }
                }
                li { class: "me-2",
                    Link {
                        class: if matches!(path, Route::BoatIssues{..}) { active_class } else {inactive_class}, 
                        to: Route::BoatIssues{id},
                        "Issues"
                    }
                }
                li { class: "me-2",
                    Link {
                        class: if matches!(path, Route::BoatEdit{..}) { active_class } else {inactive_class}, 
                        to: Route::BoatEdit{id},
                        "Edit"
                    }
                }
            } 
        }   
        dioxus_router::components::Outlet::<crate::ui::components::Route>  {}
    }
}

#[component]
pub fn BoatSummary(id: BoatId) -> Element {
    let boat_fut = use_server_future(move || async move { get_boat(id).await })?;

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            BoatTitle {
                boat: boat_fut.value().read().clone()?,
            }

        }
    }
}

#[component]
pub fn BoatMonthlyUses(id: BoatId) -> Element {
    let boat_fut = use_server_future(move || async move { get_boat(id).await })?;
    let uses_fut = use_server_future(move || async move { get_events_for_boat(id).await })?;

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            BoatTitle {
                boat: boat_fut.value().read().clone()?,
            }
            BoatUses {
                use_events: uses_fut.value().read().clone()?,
            }
        }
    }
}

#[component]
pub fn BoatYearlyUses(id: BoatId) -> Element {
    let boat_fut = use_server_future(move || async move { get_boat(id).await })?;
    let uses_fut = use_server_future(move || async move { get_events_for_boat(id).await })?;

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            BoatTitle {
                boat: boat_fut.value().read().clone()?,
            }
            BoatUses {
                use_events: uses_fut.value().read().clone()?,
            }
        }
    }
}

#[component]
pub fn BoatIssues(id: BoatId) -> Element {
    let boat_fut = use_server_future(move || async move { get_boat(id).await })?;
    let issues_fut = use_server_future(move || async move { get_open_issues_for_boat(id).await })?;

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            BoatTitle {
                boat: boat_fut.value().read().clone()?, 
            }
            BoatIssueList {
               issues: issues_fut.value().read().clone()? 
            } 
        }
    }
}


#[component]
pub fn BoatEdit(id: BoatId) -> Element {
    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            self::creation_edit_form::EditBoatForm {
                id
            }
        }
    }
}


#[component]
fn BoatTitle(
    boat: Result<BoatAndStats, ServerFnError>,
) -> Element {
    match boat {
        Ok(boat) => rsx! {
            div {
                class: "flex flex-row  bg-ggrc items-center",
                div {
                    "style": "min-width: 160px; font-size: x-large; font-weight: 500 ",
                    class: "px-4",
                    {boat.boat.name.clone()}
                }
                div {
                    class: "px-4",
                    {
                        format!("{:?} {}",boat.boat.weight_class, boat.boat.boat_type().unwrap())
                    }
                }
            }
        },
        Err(error) => rsx! {
            div {
                {error.to_string()}
            }
        },
    }
}

#[component]
fn BoatUses(
    use_events: Result<Vec<(NaiveDate, f32)>, ServerFnError>,
) -> Element {
    match use_events {
        Ok(timed_counts) => {
            rsx! {
                div {
                    class: "px-4",
                    h3 {
                        class: "font-large",
                        "Uses"
                    }
                    if timed_counts.is_empty() {
                        div {
                            "Boat has not been used."
                        }
                    } else {
                        dioxus_charts::BarChart {
                            height: "100%",
                            width: "1000px",
                            padding_top: 10,
                            padding_left: 40,
                            padding_bottom: 20,
                            padding_right: 40,
                            show_grid_ticks: true,
                            bar_width: "2%",
                            horizontal_bars: true,
                            label_interpolation: (|v| {
                                if v == 0.0 {
                                    "".to_string()
                                } else {
                                    format!("{v}")
                                }
                            }) as fn(f32) -> String,
                            series: vec![timed_counts.iter().map(|( _time, count,)| *count).collect::<Vec<_>>()],
                            labels: timed_counts.into_iter().map(|(time, _count)| time.format("%m-%d").to_string()).collect::<Vec<_>>(),
                        }
                    }
                }
            }
        }
        Err(error) => {
            rsx! {
                div {
                    "error: ",
                    {error.to_string()}
                }
            }
        }
    }
}

#[component]
fn BoatIssueList(
    issues: Result<Vec<Issue>, ServerFnError>,
) -> Element {
    match issues {
        Ok(issues) => {
            rsx! {
                div {
                    class: "px-4",
                    h3 {
                        "Issues"
                    }
                    if issues.is_empty() {
                        div {
                            "No issues"
                        }
                    } else {
                        {issues.into_iter().map(|issue| rsx! {
                            BoatIssue {
                                issue: issue
                            }
                        })}
                    }
                }
            }
        }
        Err(error) => {
            rsx! {
                div {
                    "error: ",
                    {error.to_string()}
                }
            }
        }
    }
}

#[component]
fn BoatIssue(issue: Issue) -> Element {
    rsx! {
        div {
            class: "flex flex-col flex-grow p-3",
            onclick: move |event| {
                // now, outer won't be triggered
                event.stop_propagation();
            },
            div {
                class: "flex flex-col flex grow gap-10",
                div {
                    "Created at ",
                    {issue.recorded_at.to_string()},
                }
                {issue.resolved_at.map(|time| rsx!{
                    div {
                        "Resolved at ",
                        {time.to_string()}
                    }
                })}
                div {
                    "style": "min-width: 160px; font-size: x-large; font-weight: 500",
                    {issue.note.clone()}
                }

            }
        }
    }
}
