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
    let mode = use_signal(|| BoatPageMode::View);

    rsx! {
        div {
            class: "overflow-y-auto flex flex-row flex-grow max-h-[calc(100vh-42px)] divide-x-4 dark:divide-white bg-slate-50 dark:bg-slate-500",
            BoatTitle {
                boat: boat_fut.value().read().clone()?,
                mode: mode
            }
            BoatIssueList {
                issues: issues_fut.value().read().clone()?,
                mode: mode
            }
        }

    }
}

// pub type RefResult<'a, T> = std::cell::Ref<'a, Result<T, ServerFnError>>;

#[component]
fn BoatTitle(
    boat: Result<BoatAndStats, ServerFnError>,
    mut mode: Signal<BoatPageMode>
) -> Element {
    match boat {
        Ok(boat) => rsx! {
            div {
                class: "flex flex-row flex-grow p-3",
                onclick: move |event| {
                    // now, outer won't be triggered
                    event.stop_propagation();

                },
                div {
                    class: "flex flex-row flex grow gap-10 bg-ggrc",
                    div {
                        "style": "min-width: 160px; font-size: x-large; font-weight: 500 ",
                        
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
