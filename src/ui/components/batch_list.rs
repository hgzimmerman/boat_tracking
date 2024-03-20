use crate::db::{boat::{Boat, BoatAndStats}, use_event::{UseEvent, UseScenario}, use_event_batch::UseEventBatch};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use futures::TryFutureExt;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BatchAndCount {
    batch: UseEventBatch,
    use_counts: i64
}

#[server(GetBatches)]
async fn get_batches(
    scenario: Option<UseScenario>,
    offset: usize,
    limit: usize
) -> Result<Vec<BatchAndCount>, ServerFnError> {
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
        .map(|list| list.into_iter().map(|(batch, use_counts)| BatchAndCount{batch, use_counts}).collect::<Vec<_>>())
        .map_err(ServerFnError::from)
}

#[component]
pub fn BatchList(cx: Scope) -> Element {
    let offset_state: &UseState<usize> = use_state(cx, || 0);
    let limit_state: &UseState<usize> = use_state(cx, || 0);
    let scenario_state: &UseState<Option<UseScenario>> = use_state(cx, || None);
    let batch_data_state: &UseState<Vec<BatchAndCount>> = use_state(cx, || vec![]);
    use_future(cx, (offset_state, limit_state, scenario_state), move |(offset, limit, scenario)| {
        to_owned![batch_data_state];
        async move {
            let _ = get_batches(
                scenario.current().as_ref().clone(), 
                offset.current().as_ref().clone(),
                limit.current().as_ref().clone(),
            )
            .await
            .map(|batch_data| batch_data_state.set(batch_data))
            .map_err(|error| {
                tracing::warn!(?error, "Colud not fetch");
            });
        }
    });
    cx.render(rsx! {
        div {
            batch_data_state.get().iter().map(|BatchAndCount { batch, use_counts }| rsx!{
                div {
                    class: "flex flex-row h-16 items-center",
                    div {
                        class: "m-2 w-8",
                        batch.use_scenario.to_string()
                    }
                    div {
                        class: "m-2 w-5",
                        batch.recorded_at.to_string()
                    }
                    div {
                        class: "m-2 w-5",
                        format!("{use_counts} uses")
                    }
                    // add button  for opening this batch. Try to reuse the batch pane, creating an edit mode for it.
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