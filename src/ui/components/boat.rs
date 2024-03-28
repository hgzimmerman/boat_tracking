use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::db::{
    boat::{types::BoatId, BoatAndStats},
    issue::Issue,
};

#[derive(Debug, Clone, Copy, Default)]
enum BoatPageMode {
    #[default]
    View,
    Edit
}

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
pub fn BoatPage(id: BoatId) -> Element {
    let boat_fut = use_server_future(move || async move { get_boat(id).await })?;
    let issues_fut = use_server_future(move || async move { get_open_issues_for_boat(id).await })?;
    let uses_fut = use_server_future(move || async move { get_events_for_boat(id).await })?;
    // TODO maybe do a modal, reusing the new boat logic?
    let mode = use_signal(|| BoatPageMode::View);

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)] dark:divide-white bg-slate-50 dark:bg-slate-500",
            BoatTitle {
                boat: boat_fut.value().read().clone()?,
                mode: mode
            }
            div {
                class: "flex flex-row flex-grow divide-x-4 dark:divide-white bg-slate-50 dark:bg-slate-500",
                BoatUses {
                    use_events: uses_fut.value().read().clone()?,
                }
                BoatIssueList {
                    issues: issues_fut.value().read().clone()?,
                    mode: mode
                }
            }
            
        }
    }
}


#[component]
fn BoatTitle(
    boat: Result<BoatAndStats, ServerFnError>,
    mut mode: Signal<BoatPageMode>
) -> Element {
    match boat {
        Ok(boat) => rsx! {
            div {
                class: "flex flex-row  bg-ggrc",
                div {
                    "style": "min-width: 160px; font-size: x-large; font-weight: 500 h-20",
                    
                    {boat.boat.name.clone()}
                }
                div {
                    {
                        format!("{:?} {:?}",boat.boat.weight_class, boat.boat.boat_type().unwrap())
                    }
                }
                button {
                    class: "btn btn-blue",
                    onclick: move | event| {
                        event.stop_propagation();
                        let current_mode = *mode.read();
                        match current_mode {
                            BoatPageMode::View => mode.set(BoatPageMode::Edit),
                            BoatPageMode::Edit => {
                                // TODO save changes; may want to rearchitect this into a service
                                mode.set(BoatPageMode::View);
                            } 
                        }
                    },
                    match *mode.read() {
                        BoatPageMode::View => "Edit",
                        BoatPageMode::Edit => "Save Changes"
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
                        "Uses"
                    }
                    if timed_counts.is_empty() {
                        div {
                            "Boat has not been used."
                        }
                    } else {
                        dioxus_charts::LineChart {
                            height: "100%",
                            width: "600px",
                            padding_top: 30,
                            padding_left: 50,
                            padding_bottom: 30,
                            padding_right: 50,
                            show_grid_ticks: true,
                            // bar_width: "10%",
                            // horizontal_bars: false,
                            label_interpolation: (|v| format!("{v}")) as fn(f32) -> String,
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
    mode: ReadOnlySignal<BoatPageMode>
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
