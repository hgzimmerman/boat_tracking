use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::db::{boat::types::BoatId, issue::Issue};




#[component]
pub fn BoatIssues(id: BoatId) -> Element {
    let issues_fut = use_resource(use_reactive!(|id| async move {
        get_open_issues_for_boat(id).await
    }));

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            BoatIssueList {
               issues: issues_fut.value().read().clone()?
            }
        }
    }
}


#[component]
fn BoatIssueList(issues: Result<Vec<Issue>, ServerFnError>) -> Element {
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
                    "style": "min-width: 160px;",
                    class: "text-xl font-medium",
                    {issue.note.clone()}
                }

            }
        }
    }
}


#[server(GetBoatOpenIssues)]
pub(crate) async fn get_open_issues_for_boat(id: BoatId) -> Result<Vec<Issue>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
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