use std::ops::Deref;

use dioxus_fullstack::prelude::*;
use dioxus::prelude::*;

use crate::db::{boat::{types::BoatId, BoatAndStats}, issue::Issue};

#[server(GetBoat)]
pub(crate) async fn get_boat(id: BoatId) -> Result<BoatAndStats, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;

    conn 
        .interact(move |conn| {
            BoatAndStats::get_boat(conn, id).map_err(ServerFnError::from)
        })
        .await
        .map_err(ServerFnError::from)?
}

#[server(GetBoatOpenIssues)]
pub(crate) async fn get_open_issues_for_boat(id: BoatId) -> Result<Vec<Issue>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;

    conn 
        .interact(move |conn| {
            Issue::get_open_issues_for_boat(conn, id).map_err(ServerFnError::from)
        })
        .await
        .map_err(ServerFnError::from)?
}

#[server(GetBoatResolvedIssues)]
pub(crate) async fn get_resolved_issues_for_boat(id: BoatId) -> Result<Vec<Issue>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;

    conn 
        .interact(move |conn| {
            Issue::get_resolved_issues_for_boat(conn, id).map_err(ServerFnError::from)
        })
        .await
        .map_err(ServerFnError::from)?
}
#[component]
pub fn BoatPage(cx: Scope, id: BoatId) -> Element {
    let boat_fut= use_server_future(cx, id, |id| async move {
        get_boat(id).await
    })?;
    let issues_fut= use_server_future(cx, id, |id| async move {
        get_open_issues_for_boat(id).await
    })?;
    cx.render(rsx!{
        div {
            "style": "display:flex; flex-direction: vertical; flex-grow: 1;",
            BoatTitle {
                boat: boat_fut.value()
            }
            BoatIssueList {
                issues: issues_fut.value()
            }
        }

    })
}

pub type RefResult<'a, T> = std::cell::Ref<'a, Result<T, ServerFnError>>;

#[component]
fn BoatTitle<'a>(cx: Scope, boat: RefResult<'a, BoatAndStats>) -> Element<'a> {
    cx.render(rsx! {
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
                    boat.as_ref().unwrap().boat.name.clone(),
                }
                div {
                    format!("{:?} {:?}",boat.as_ref().unwrap().boat.weight_class, boat.as_ref().unwrap().boat.boat_type().unwrap())
                }
            } 
        }
    })
}

#[component]
fn BoatIssueList<'a>(cx: Scope, issues: RefResult<'a, Vec<Issue>>) -> Element<'a> {
    use crate::ui::util::loadable::Loadable::*;
    cx.render(match issues.as_deref() {
        Ok(issues) => {
            rsx! {
                div {
                    h3 {
                        "Issues"
                    }
                    
                    if issues.is_empty() {
                        cx.render(rsx!{
                            div {
                                "No issues"
                            }
                        })
                    } else {
                        cx.render(rsx!{issues.iter().map(|issue| rsx! {
                            BoatIssue {
                                issue: issue 
                            } 
                        })})
                    }
                }
            }
            
        },
        Err(error) => {
            rsx!{
                div {
                    "error: ",
                    error.to_string()
                }
            }
        }
    })
}

#[component]
fn BoatIssue<'a>(cx: Scope, issue: &'a Issue) -> Element<'a> {
    cx.render(rsx! {
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
                    issue.recorded_at.to_string(),
                } 
                issue.resolved_at.map(|time| rsx!{
                    div {
                        "Resolved at ",
                        time.to_string(),
                    }
                })
                div {
                    "style": "min-width: 160px; font-size: x-large; font-weight: 500",
                    issue.note.clone(),
                }
                 
            } 
        }
    })
}