use crate::db::{boat::Boat, use_event::{UseEvent, UseScenario}, use_event_batch::UseEventBatch};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OldBatchAndData {
    batch: UseEventBatch,
    uses_and_boats: Vec<(UseEvent, Boat)>
}

#[server(GetOldBatch)]
async fn get_old_batch(
    scenario: Option<UseScenario>,
    offset: usize 
) -> Result<Option<OldBatchAndData>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu"); 
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;
    conn 
        .interact(move |conn| {
            if let Some(batch) = UseEventBatch::get_most_recent_batch(
                conn, 
                scenario, 
                offset
            )
            .map_err(ServerFnError::from)? {
                let uses_and_boats = UseEventBatch::get_events_and_boats_for_batch(
                    conn, 
                    batch.id
                ).map_err(ServerFnError::from)?;
                Ok(Some(OldBatchAndData {
                    batch,
                    uses_and_boats,
                }))
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(ServerFnError::from)?
}

#[component]
pub fn OldBatches(cx: Scope) -> Element {
    let offset_state: &UseState<usize> = use_state(cx, || 0);
    let scenario_state: &UseState<Option<UseScenario>> = use_state(cx, || None);
    let batch_data_state: &UseState<Option<OldBatchAndData>> = use_state(cx, || None);
    use_future(cx, (offset_state, scenario_state), move |(offset, scenario)| {
        to_owned![batch_data_state];
        async move {
            let _ = get_old_batch(scenario.current().as_ref().clone(), offset.current().as_ref().clone())
            .await
            .map(|batch_data| batch_data_state.set(batch_data))
            .map_err(|error| {
                tracing::warn!(?error, "Colud not fetch");
            });
        }
    });
    cx.render(rsx! {
        div {
            if let Some(batch) = batch_data_state.get() {
                cx.render(rsx!{
                    List {
                        boats: &batch.uses_and_boats
                    }
                })
                
            } else {
                None
            }
        }
    })
}

#[component]
fn List<'a>(
    cx: Scope, 
    boats: &'a [(UseEvent, Boat)] 
    // boat_svc: &'a Coroutine<BoatListMsg>
) -> Element {
    cx.render(rsx!{
        div {
            class: "flex flex-col grow overflow-auto divide-y",
            boats.iter().map(|(_u, b)| rsx!{
                div {
                    class: "flex flex-row h-16 items-center",
                    div {
                        class: "m-2 grow",
                        b.name.clone()
                    }
                    div {
                        class: "m-2",
                        b.boat_type().as_ref().map(ToString::to_string)
                    }
                    // button {
                    //     class: "m-2 btn btn-red",
                    //     onclick: move |_| {
                    //         boat_svc.send(BoatListMsg::RemoveFromBatch(b.id.clone()));
                    //     },
                    //     "Remove"
                    // }
                }
            }) 
        }
    })
}