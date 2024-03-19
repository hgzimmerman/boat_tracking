//! The idea with the batch is that it should be a fast way to enter in boats that were used at the same time.
//! 
//! It should be a growable list of boats, and an add button. 
//! Clicking the add button should open up a righthand pane, splitting the page in two (if mobile ideally show it as a modal?).
//! In the new, righthand pane, should be a search function for boats (search by name, by boat type).
//! Clicking a boat there should add it to the list on the left, removing it from the search.
//! Once all boats are added, there should be an option to save the whole batch, creating use_events that are tied to that batch.
//! 
//! Previous batches should be able to be looked through, and should have an option to "copy batch as template", 
//! bringing you to the aforementioned view, pre-populated with the same boats from that batch that you can then edit and save.
//! 
use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use crate::{db::{boat::{types::{BoatId, HasCox, OarConfiguration, SeatCount}, Boat, BoatFilter3}, use_event::UseScenario, use_event_batch::{BatchId, NewBatch, NewBatchArgs, UseEventBatch}}, ui::components::toast::{MsgType, ToastCenter, ToastData}};
use super::toast::{ToastList, ToastMsgMsg};


mod list_pane;
mod search_pane;

#[component]
pub fn BatchCreationPage(cx: Scope) -> Element {
    let selected = use_state(cx, || Vec::<Boat>::new());
    let filter = use_state(cx, BoatFilter3::default);
    let search_name = use_state(cx, || Option::<String>::None);
    let search_boat_state = use_state(cx,  || Vec::<Boat>::new());

    let toasts = use_state(cx, ToastList::default);
    let toast_svc = use_coroutine(cx, |rx| {
        to_owned![toasts];
        crate::ui::components::toast::toast_service(rx, toasts)
    });
    let boat_svc = use_coroutine(cx, |rx| {
        to_owned![search_boat_state, filter, selected, search_name, toast_svc];
        boat_list_service(rx, search_boat_state, selected, filter, search_name, toast_svc)
    });


    cx.render(rsx!{
        ToastCenter {
            toasts: &*toasts,
            toast_svc: toast_svc
        }
        div {
            // I don't love the magic number (42px corresponds to the nav height)
            class: "flex flex-row overflow-hide divide-x-4 grow max-h-[calc(100vh-42px)]", 
            list_pane::BatchListPane {
                boats: selected,
                boat_svc: boat_svc
            }
            search_pane::BoatSearchPane{
                boats: search_boat_state,
                filter: filter,
                search_name: search_name,
                boat_svc: boat_svc
            }
        }
    })
}

#[server(GetBoats2)]
pub(crate) async fn search_boats(
    filter: BoatFilter3,
    search_name: Option<String>, 
) -> Result<Vec<Boat>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu"); 
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;
    tracing::info!(?search_name, ?filter);
    conn 
        .interact(|conn| {
            Boat::get_boats3(conn, filter, search_name)
            .map_err(ServerFnError::from)
        })
        .await
        .map_err(ServerFnError::from)?
}

#[server(SubmitBoats)]
pub(crate) async fn submit_boats(
    boat_ids: Vec<BoatId>,
    session_type: UseScenario
) -> Result<BatchId, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu"); 
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;
    let new_batch = NewBatchArgs { boat_ids, batch: NewBatch { use_scenario: session_type, recorded_at: chrono::Utc::now().naive_utc() }};
    conn 
        .interact(|conn| {
            UseEventBatch::create_batch(conn, new_batch)
            .map_err(ServerFnError::from)
        })
        .await
        .map_err(ServerFnError::from)?
}




enum BoatListMsg {
    /// Run the fetch
    Fetch,
    SetFilterOarConfig(Option<OarConfiguration>),
    SetFilterCoxed(Option<HasCox>),
    SetFilterNumSeats(Option<SeatCount>),
    SetSearch(String),
    AddToBatch(BoatId),
    RemoveFromBatch(BoatId),
    /// Submits the selected boats, saving use events for each one.
    Submit
}


async fn boat_list_service(
    mut rx: UnboundedReceiver<BoatListMsg>,
    searched_boats: UseState<Vec<Boat>>,
    selected_boats: UseState<Vec<Boat>>,
    filter: UseState<BoatFilter3>,
    search_name: UseState<Option<String>>,
    toasts: Coroutine<ToastMsgMsg>
) {
    use futures::stream::StreamExt;

    let search = || async {
        let _ = search_boats(
                filter.current().as_ref().to_owned(), 
                search_name.current().as_ref().to_owned(),
            )
            .await
            .map(|boats| {
                let exclude: HashSet<BoatId> = selected_boats.current().iter().map(|x| x.id).collect();
                boats.into_iter().filter(|x| !exclude.contains(&x.id)).collect()
            })
            .map(|x| searched_boats.set(x));
    };

    while let Some(msg) = rx.next().await {
        match msg {
            BoatListMsg::Fetch => {
                tracing::info!("fetching");
                search().await
            },
            BoatListMsg::SetSearch(search_str) => {
                tracing::info!(%search_str, "setting search");
                if search_str.is_empty() {
                    search_name.set(None);
                } else {
                    search_name.set(Some(search_str));
                    // TODO add some debouncing if already searching
                    search().await
                }
            }
            BoatListMsg::SetFilterOarConfig(oars_config) => {
                filter.set({
                    let current = filter.current();
                    BoatFilter3 { _x: current._x, num_seats: current.num_seats, coxed: current.coxed, oars_config }
                });
                search().await;
            }
            BoatListMsg::SetFilterCoxed(coxed) => {
                filter.set({
                    let current = filter.current();
                    BoatFilter3 { _x: current._x, num_seats: current.num_seats, coxed, oars_config: current.oars_config }
                });
                search().await;
            }
            BoatListMsg::SetFilterNumSeats(num_seats) => {
                filter.set({
                    let current = filter.current();
                    BoatFilter3 { _x: current._x, num_seats, coxed: current.coxed, oars_config: current.oars_config }
                });
                search().await;
            }
            BoatListMsg::AddToBatch(id) => {
                tracing::info!(?id,"adding to batch");
                let mut boat_to_add = None;
                searched_boats.modify(|boats| {
                    boats.iter().cloned().filter_map(|boat| {
                        if boat.id == id {
                            boat_to_add = Some(boat);
                            None
                        } else {
                            Some(boat)
                        }
                    })
                    .collect()
                });
                if let Some(boat) = boat_to_add{
                    selected_boats.make_mut().push(boat);
                    searched_boats.needs_update();
                }
            },
            BoatListMsg::RemoveFromBatch(id) => {
                let mut boat_to_remove = None;
                selected_boats.modify(|boats| {
                    boats.iter().cloned().filter_map(|boat| {
                        if boat.id == id {
                            boat_to_remove = Some(boat);
                            None
                        } else {
                            Some(boat)
                        }
                    })
                    .collect()
                });
                if let Some(boat) = boat_to_remove {
                    searched_boats.make_mut().push(boat);
                    selected_boats.needs_update();
                }
            },
            BoatListMsg::Submit => {
                let ids: Vec<BoatId> = selected_boats.current().iter().map(|b| b.id).collect();
                let session_type = UseScenario::AM; // TODO Get this from the actual setting
                if !ids.is_empty() {
                    match submit_boats(ids, session_type).await {
                        Ok(id) => {
                            tracing::info!(%id, "Created batch");
                            // In the future, toast a success message
                            searched_boats.set(Vec::new());
                            selected_boats.set(Vec::new());
                            search_name.set(None);
                            toasts.send(ToastMsgMsg::Add(
                                ToastData { msg: "Submitted boats".to_string(), ty: MsgType::Normal }, 
                                std::time::Duration::from_secs(2)
                            ));
                        }
                        Err(error) => {
                            tracing::error!(?error, "Could not submit batch");
                            toasts.send(ToastMsgMsg::Add(
                                ToastData { msg: format!("Could not submit batch {error}"), ty: MsgType::Normal }, 
                                std::time::Duration::from_secs(2)
                            ));
                        }
                    }
                }
            }
        } 
    }
}

