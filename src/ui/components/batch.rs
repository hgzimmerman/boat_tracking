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
use crate::{db::{boat::{types::{BoatId, HasCox, OarConfiguration, SeatCount}, Boat, BoatFilter3}, use_event::{UseEvent, UseScenario}, use_event_batch::{BatchId, NewBatch, NewBatchArgs, UseEventBatch}}, ui::components::toast::ToastData};
use super::toast::ToastMsgMsg;


mod list_pane;
mod search_pane;

#[component]
pub fn BatchTemplateCreationPage(id: BatchId) -> Element{

    rsx!{
        GeneralBatchCreationPage {
            mode: BatchPageMode::Template{id}
        }
    }
}

#[component]
pub fn BatchCreationPage() -> Element{
    rsx!{
        GeneralBatchCreationPage {
            mode: BatchPageMode::Create 
        }
    }
}

#[component]
pub fn BatchViewingPage(id: BatchId) -> Element {
    rsx!{
        GeneralBatchCreationPage {
            mode: BatchPageMode::View {id}
        }
    }
}

#[component]
pub fn BatchEditPage(id: BatchId) -> Element {
    rsx!{
        GeneralBatchCreationPage {
            mode: BatchPageMode::Edit {id}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BatchPageMode {
    Create,
    View{id: BatchId},
    Template{id: BatchId},
    Edit{id: BatchId},
}
impl BatchPageMode {
    pub fn as_option(self) -> Option<BatchId> {
        match self {
            BatchPageMode::Create  => None,
            BatchPageMode::Template { id } | BatchPageMode::Edit { id } | BatchPageMode::View{id} => Some(id),
        }
    }
    pub fn is_view(self) -> bool {
        match self {
            BatchPageMode::View{..}  => true,
            BatchPageMode::Template { .. } | BatchPageMode::Edit { .. } | BatchPageMode::Create => false,
        }
    }
}

/// `id` is an optional argument, that if specified, will initialize 
/// the page with boats that came from that particular batch.
#[component]
fn GeneralBatchCreationPage(mode: BatchPageMode) -> Element {
    let mut selected = use_signal(|| Vec::<Boat>::new());
    let filter = use_signal(BoatFilter3::default);
    let search_name = use_signal(|| Option::<String>::None);
    let search_boat_state = use_signal(|| Vec::<Boat>::new());

    // let toasts = use_signal(ToastList::default);
    // let toast_svc = use_coroutine(|rx| {
    //     to_owned![toasts];
    //     crate::ui::components::toast::toast_service(rx, toasts)
    // });
    let toast_svc = use_coroutine_handle::<ToastMsgMsg>();
    let boat_svc = use_coroutine(|rx| {
        to_owned![search_boat_state, filter, selected, search_name, toast_svc];
        boat_list_service(rx, search_boat_state, selected, filter, search_name, toast_svc)
    });

    // Initialize the bage by fetching the msgs.
    use_future(move || {
        async move {
            boat_svc.send(BoatListMsg::Fetch);
        }
    });

    // If the ID is populated, then use it to fetch the existing set of boats for a specific batch.
    if let Some(id) = mode.as_option() {
        use_future(move || {
            async move {
                match get_existing_batch(id).await {
                    Ok(batch) => {
                        let batch = batch.into_iter().map(|(_event, boat)| boat).collect::<Vec<_>>();
                        selected.set(batch)
                        // TODO also set the time element (when we add one) corresponding to the batch in question.
                    },
                    Err(error) => {
                        toast_svc.send(ToastMsgMsg::Add(ToastData::from(error), ToastData::DEFAULT_TIME))
                    },
                }
            }
        });
    }


    rsx!{
        div {
            // I don't love the magic number (42px corresponds to the nav height)
            class: "flex flex-row overflow-hide divide-x-4 grow max-h-[calc(100vh-42px)]", 
            list_pane::BatchListPane {
                boats: selected,
                boat_svc: boat_svc,
                mode: mode
            }
            {
                if mode.is_view() {
                    None
                } else {
                    rsx!{
                        search_pane::BoatSearchPane{
                            boats: search_boat_state,
                            filter: filter,
                            search_name: search_name,
                            boat_svc: boat_svc
                        }
                    }
                }
            }
        }
    }
}


#[server(GetBoats2)]
pub(crate) async fn search_boats(
    filter: BoatFilter3,
    search_name: Option<String>, 
) -> Result<Vec<Boat>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu"); 
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;
    tracing::info!(?search_name, ?filter);
    conn 
        .interact(|conn| {
            Boat::get_boats3(conn, filter, search_name)
            .map_err(ServerFnError::from)
        })
        .await?
}

#[server(SubmitBoats)]
pub(crate) async fn submit_boats(
    boat_ids: Vec<BoatId>,
    session_type: UseScenario
) -> Result<BatchId, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu"); 
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;
    let new_batch = NewBatchArgs { boat_ids, batch: NewBatch { use_scenario: session_type, recorded_at: chrono::Utc::now().naive_utc() }};
    conn 
        .interact(|conn| {
            UseEventBatch::create_batch(conn, new_batch)
            .map_err(ServerFnError::from)
        })
        .await?
}
#[server(GetExistingBatch)]
pub(crate) async fn get_existing_batch(
    batch_id: BatchId,
) -> Result<Vec<(UseEvent, Boat)>, ServerFnError> {
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;
    conn 
        .interact(move |conn| {
            UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)
            .map_err(ServerFnError::from)
        })
        .await?
}



#[derive(Debug, Clone, PartialEq)]
enum BoatListMsg {
    /// Fetch the boats according to the search criteria. 
    Fetch,
    /// Filter the search to only boats with sweep or sculling oar configurations. 
    SetFilterOarConfig(Option<OarConfiguration>),
    /// Filter the search to only boats with or without a coxswain. 
    SetFilterCoxed(Option<HasCox>),
    /// Filter the search to only boats with this number of seats
    SetFilterNumSeats(Option<SeatCount>),
    /// Set the search string used to filter boats in the right-hand-pane.
    SetSearch(String),
    /// Remove an item from the right-hand-pane, adding it to the left-hand-pane.
    /// This item will be saved as part of the batch.
    AddToBatch(BoatId),
    /// Remove an item from the left-hand-pane.
    /// This should cause it to show back up in the other pane as it is no longer filtered.
    RemoveFromBatch(BoatId),
    /// Only used when in edit mode.
    /// This should, in a transaction, delete the use events tied to the batch, 
    /// and re-create new ones that reflect the boat ids passed in.
    /// The created at time should also be set to the value provided. 
    SaveChanges{batch_id: BatchId, boat_ids: Vec<BoatId>},
    /// Submits the selected boats, saving use events for each one.
    Submit
}


async fn boat_list_service(
    mut rx: UnboundedReceiver<BoatListMsg>,
    mut searched_boats: Signal<Vec<Boat>>,
    mut selected_boats: Signal<Vec<Boat>>,
    mut filter: Signal<BoatFilter3>,
    mut search_name: Signal<Option<String>>,
    toasts: Coroutine<ToastMsgMsg>
) {
    use futures::stream::StreamExt;

    let search = {
        to_owned![filter, search_name];
        move || {
            async move {
            let _ = search_boats(
                    filter.read().to_owned(), 
                    search_name.read().to_owned(),
                )
                .await
                .map(|boats| {
                    let exclude: HashSet<BoatId> = selected_boats.read().iter().map(|x| x.id).collect();
                    boats.into_iter().filter(|x| !exclude.contains(&x.id)).collect()
                })
                .map(|x| searched_boats.set(x));
        }}
    };

    while let Some(msg) = rx.next().await {
        match msg {
            BoatListMsg::Fetch => {
                tracing::info!("fetching");
                search().await
            },
            BoatListMsg::SaveChanges { batch_id, boat_ids }=> {
                tracing::info!(?batch_id, ?boat_ids, "Overwriting old batch with new data");
                // search().await
            },
            BoatListMsg::SetSearch(search_str) => {
                tracing::info!(%search_str, "setting search");
                if search_str.is_empty() {
                    search_name.set(None);
                } else {
                    search_name.set(Some(search_str));
                }
                search().await
            }
            BoatListMsg::SetFilterOarConfig(oars_config) => {
                filter.set({
                    let current = filter.read();
                    BoatFilter3 { _x: current._x, num_seats: current.num_seats, coxed: current.coxed, oars_config }
                });
                search().await;
            }
            BoatListMsg::SetFilterCoxed(coxed) => {
                filter.set({
                    let current = filter.read();
                    BoatFilter3 { _x: current._x, num_seats: current.num_seats, coxed, oars_config: current.oars_config }
                });
                search().await;
            }
            BoatListMsg::SetFilterNumSeats(num_seats) => {
                filter.set({
                    let current = filter.read();
                    BoatFilter3 { _x: current._x, num_seats, coxed: current.coxed, oars_config: current.oars_config }
                });
                search().await;
            }
            BoatListMsg::AddToBatch(id) => {
                tracing::info!(?id,"adding to batch");
                let mut boat_to_add = None;
                let new_searched_boats = {
                    searched_boats.read().iter().cloned().filter_map(|boat| {
                        if boat.id == id {
                            boat_to_add = Some(boat);
                            None
                        } else {
                            Some(boat)
                        }
                    })
                    .collect()
                };
                searched_boats.set(new_searched_boats);
                if let Some(boat) = boat_to_add{
                    selected_boats.write().push(boat);
                }
            },
            BoatListMsg::RemoveFromBatch(id) => {
                let mut boat_to_remove = None;
                let new_selected_boats = {
                    selected_boats.read().iter().cloned().filter_map(|boat| {
                        if boat.id == id {
                            boat_to_remove = Some(boat);
                            None
                        } else {
                            Some(boat)
                        }
                    })
                    .collect()
                };
                selected_boats.set(new_selected_boats);
                if let Some(boat) = boat_to_remove {
                    searched_boats.write().push(boat);
                }
            },
            BoatListMsg::Submit => {
                let ids: Vec<BoatId> = selected_boats.read().iter().map(|b| b.id).collect();
                let session_type = UseScenario::AM; // TODO Get this from the actual setting
                if !ids.is_empty() {
                    match submit_boats(ids, session_type).await {
                        Ok(id) => {
                            tracing::info!(%id, "Created batch");
                            searched_boats.set(Vec::new());
                            selected_boats.set(Vec::new());
                            search_name.set(None);
                            toasts.send(
                                ToastData::success("Submitted boats").into()
                            );
                        }
                        Err(error) => {
                            tracing::error!(?error, "Could not submit batch");
                            toasts.send(
                                ToastData::warn(format!("Could not submit batch")).into()
                            );
                        }
                    }
                }
            }
        } 
    }
}

