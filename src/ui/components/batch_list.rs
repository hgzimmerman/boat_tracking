use crate::db::{use_event::UseScenario, use_event_batch::{BatchAndCounts, UseEventBatch}};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;


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
pub fn BatchList(cx: Scope) -> Element {
    let offset_state: &UseState<usize> = use_state(cx, || 0);
    let limit_state: &UseState<usize> = use_state(cx, || 20);
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
                        // add button  for opening this batch. Try to reuse the batch pane, creating an edit mode for it.
                    }
                }
            })
        }
    })
}

#[component]
pub fn BatchListPage(cx: Scope) -> Element {

    cx.render(rsx! {
        div {
            class: "flex flex-col overflow-hide grow max-h-[calc(100vh-42px)]",
            // page header/nav
            div {
                class: "h-8",
                "Add controls here for pagination, "
            }
            // the controls
            BatchList {

            }
        }
        
    })

}