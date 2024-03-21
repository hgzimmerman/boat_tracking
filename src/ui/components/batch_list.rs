use std::{fmt::Write, str::FromStr};

use crate::db::{use_event::UseScenario, use_event_batch::{BatchAndCounts, UseEventBatch}};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::routable::FromQuery;


#[server(GetBatches)]
async fn get_batches(
    scenario: Option<UseScenario>,
    offset: usize,
    limit: usize
) -> Result<Vec<BatchAndCounts>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu"); 
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;
    conn 
        .interact(move |conn| {
            UseEventBatch::get_most_recent_batches_and_their_use_count(
                conn, 
                scenario, 
                offset,
                limit 
            )
            .map_err(ServerFnError::from)
        })
        .await?
        .map_err(ServerFnError::from)
}

#[component]
pub fn BatchList (
    cx: Scope,
    offset_state: UseState<usize>,
    limit_state: UseState<usize>
) -> Element {

    let scenario_state: &UseState<Option<UseScenario>> = use_state(cx, || None);
    // let batch_data_state: &UseState<Vec<BatchAndCounts>> = use_state(cx, || vec![]);
    let batches_fut = use_server_future(cx, (offset_state, limit_state, scenario_state), move |(offset, limit, scenario)| {
        // to_owned![batch_data_state];
        let scenario = scenario.current().as_ref().clone();
        let offset = offset.current().as_ref().clone();
        let limit = limit.current().as_ref().clone();
        tracing::debug!(?scenario, ?offset, ?limit, "fetching batches");
        async move {
            get_batches(scenario, offset, limit)
            .await
            .map(|batch_data| {
                tracing::debug!(?batch_data);
                // batch_data_state.set(batch_data.clone());
                batch_data
            })
            .map_err(|error| {
                tracing::warn!(?error, "Colud not fetch");
                error
            })
        }
    })?;
    cx.render(rsx! {
        div {
            batches_fut.value().as_ref().unwrap().iter().map(|BatchAndCounts { batch, use_counts }| {
                rsx!{
                    div {
                        class: "flex flex-row h-16 items-center",
                        div {
                            class: "m-2 w-20",
                            batch.use_scenario.to_string()
                        }
                        div {
                            class: "m-2 w-40",
                            batch.recorded_at.to_string()
                        }
                        div {
                            class: "m-2 w-10",
                            format!("{use_counts} boats used")
                        }
                        // -> batch/view/:batch_id or batch/:batch_id depending on if I can make this page use ?page= parameters.
                        button {
                            class: "btn btn-blue",
                            "View"
                        }
                        // -> batch/edit/:batch_id
                        button {
                            class: "btn btn-blue",
                            "Edit"
                        }
                        // -> batch/new/:batch_id
                        button {
                            class: "btn btn-blue",
                            "Use as Template"
                        }
                    }
                }
            })
        }
    })
}

#[component]
pub fn BatchListPage(
    cx: Scope,
    page: PageQueryParams 
) -> Element {
    let offset_state: &UseState<usize> = use_state(cx, || 0);
    let limit_state: &UseState<usize> = use_state(cx, || 20);
    cx.render(rsx! {
        div {
            class: "flex flex-col overflow-hide grow max-h-[calc(100vh-42px)]",
            // page header/nav
            div {
                class: "h-8",
                "Add controls here for pagination, "
                if *offset_state.get() != 0 {
                    rsx!{
                        button {
                            class: "btn btn-blue",
                            "Newer" 
                        }
                    }
                }
                button {
                    class: "btn btn-blue",
                    "Older"
                }
            }
            // the controls
            BatchList {
                offset_state: offset_state.clone(),
                limit_state: limit_state.clone()
            }
        }
        
    })

}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Page(usize);

impl FromStr for Page {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let search = "page=";
        if s.len() < search.len() {
            return Err("Input to small".to_string())
        }
        if &s[0..search.len()] == search {
            Ok(Page(usize::from_str(&s[search.len()..]).map_err(|e| e.to_string())?))
        } else {
            Err("Missing 'page='".to_string())
        }
    }
}
impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let page = self.0;
        f.write_fmt(format_args!("page={page}"))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct PageQueryParams {
    page: Option<usize>
}
impl FromQuery for PageQueryParams {
    fn from_query(query: &str) -> Self {
        Self {
            page: Page::from_str(query).ok().map(|x| x.0)
        }
    }
}
impl std::fmt::Display for PageQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let page = self.page.map(Page).unwrap_or_default();
        page.fmt(f)
    }
}