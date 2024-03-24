
use dioxus_fullstack::prelude::{server_fn::ServerFn, *};
use dioxus::prelude::*;

use crate::db::{boat::{types::BoatId, BoatAndStats}, issue::Issue};

#[server(GetBoat)]
pub(crate) async fn get_boat(id: BoatId) -> Result<BoatAndStats, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    conn 
        .interact(move |conn| {
            BoatAndStats::get_boat(conn, id).map_err(ServerFnError::from)
        })
        .await?
}

#[server(GetBoatOpenIssues)]
pub(crate) async fn get_open_issues_for_boat(id: BoatId) -> Result<Vec<Issue>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    conn 
        .interact(move |conn| {
            Issue::get_open_issues_for_boat(conn, id).map_err(ServerFnError::from)
        })
        .await?
}

#[server(GetBoatResolvedIssues)]
pub(crate) async fn get_resolved_issues_for_boat(id: BoatId) -> Result<Vec<Issue>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn = state.pool().get().await?;

    conn 
        .interact(move |conn| {
            Issue::get_resolved_issues_for_boat(conn, id).map_err(ServerFnError::from)
        })
        .await?
}
#[component]
pub fn BoatPage(id: BoatId) -> Element {
    let boat_fut= use_server_future(move || async move {
        get_boat(id).await
    })?;
    let issues_fut= use_server_future(move || async move {
        get_open_issues_for_boat(id).await
    })?;

    rsx!{
        div {
            "style": "display:flex; flex-direction: vertical; flex-grow: 1;",
            BoatTitle {
                boat: boat_fut.value().read().clone()?
            }
            BoatIssueList {
                issues: issues_fut.value().read().clone()?
            }
        }

    }
}

// pub type RefResult<'a, T> = std::cell::Ref<'a, Result<T, ServerFnError>>;

#[component]
fn BoatTitle(boat: Result<BoatAndStats, ServerFnError>) -> Element {
    match boat {
        Ok(boat) => rsx!{
            div {
                "style": "display:flex; flex-direction: horizontal; flex-grow: 1; padding: 6px",
                onclick: move |event| {
                    // now, outer won't be triggered
                    event.stop_propagation();
                    
                },
                div {
                    "style": "display:flex; flex-direction: column; flex-grow: 1; gap: 10px;",
                    div {
                        "style": "min-width: 160px; font-size: x-large; font-weight: 500",
                        {boat.boat.name.clone()}
                    }
                    div {
                        {
                            format!("{:?} {:?}",boat.boat.weight_class, boat.boat.boat_type().unwrap())
                        }
                    }
                } 
            }
        },
        Err(error) => rsx!{
            div {
                {error.to_string()}
            }
        }
    }
}

#[component]
fn BoatIssueList(issues: Result<Vec<Issue>, ServerFnError>) -> Element {
    // use crate::ui::util::loadable::Loadable::*;
    match issues {
        Ok(issues) => {
            rsx! {
                div {
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
            
        },
        Err(error) => {
            rsx!{
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
            "style": "display:flex; flex-direction: column; flex-grow: 1; padding: 6px",
            onclick: move |event| {
                // now, outer won't be triggered
                event.stop_propagation();
            },
            div {
                "style": "display:flex; flex-direction: column; flex-grow: 1; gap: 10px;",
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