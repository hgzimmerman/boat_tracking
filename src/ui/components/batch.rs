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

use super::toast::ToastMsgMsg;
use crate::{
    db::{
        boat::{
            types::{BoatId, HasCox, OarConfiguration, SeatCount},
            Boat, BoatFilter3,
        },
        use_event::{UseEvent, UseScenario},
        use_event_batch::BatchId,
    },
    ui::components::toast::ToastData,
};
use chrono::NaiveDateTime;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

mod list_pane;
mod search_pane;

/// A page that facilitates creating a batch of uses based on an existing batch.
#[component]
pub fn BatchTemplateCreationPage(id: BatchId) -> Element {
    rsx! {
        GeneralBatchCreationPage {
            mode: BatchPageMode::Template{id}
        }
    }
}

/// A page that facilitates creating a batch of uses.
#[component]
pub fn BatchCreationPage() -> Element {
    rsx! {
        GeneralBatchCreationPage {
            mode: BatchPageMode::Create
        }
    }
}

/// A page that allows viewing an existing batch of uses.
#[component]
pub fn BatchViewingPage(id: BatchId) -> Element {
    rsx! {
        GeneralBatchCreationPage {
            mode: BatchPageMode::View {id}
        }
    }
}

/// A page that allows editing an existing batch of uses.
#[component]
pub fn BatchEditPage(id: BatchId) -> Element {
    rsx! {
        GeneralBatchCreationPage {
            mode: BatchPageMode::Edit {id}
        }
    }
}

/// Because of limitations of the router, we need a distinct component for each use case.
/// In order to share code, we have one general implementation, that we pass the 'mode'
/// to to change aspects of the pages behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BatchPageMode {
    /// Create a completely new batch
    Create,
    /// View an existing batch.
    View { id: BatchId },
    /// Create a new batch, prepopulating the list with boats from an existing batch.
    /// The saved boats will create a new batch.
    Template { id: BatchId },
    /// Allow editing an existing batch.
    /// The saved boats will replace the entries in the old batch.
    Edit { id: BatchId },
}
impl BatchPageMode {
    pub fn as_option(self) -> Option<BatchId> {
        match self {
            BatchPageMode::Create => None,
            BatchPageMode::Template { id }
            | BatchPageMode::Edit { id }
            | BatchPageMode::View { id } => Some(id),
        }
    }
    pub fn is_view(self) -> bool {
        match self {
            BatchPageMode::View { .. } => true,
            BatchPageMode::Template { .. } | BatchPageMode::Edit { .. } | BatchPageMode::Create => {
                false
            }
        }
    }
}

/// `id` is an optional argument, that if specified, will initialize
/// the page with boats that came from that particular batch.
#[component]
fn GeneralBatchCreationPage(mode: BatchPageMode) -> Element {
    let mut selected = use_signal(Vec::<Boat>::new);
    let filter = use_signal(BoatFilter3::default);
    let search_name = use_signal(|| Option::<String>::None);
    let search_boat_state = use_signal(Vec::<Boat>::new);

    let mut session_type = use_signal(|| UseScenario::Adult);
    let mut created_at_time =
        use_signal(|| crate::ui::util::time::render_local(chrono::Utc::now().naive_utc()));

    let toast_svc = use_coroutine_handle::<ToastMsgMsg>();
    let boat_svc = use_coroutine(|rx| {
        to_owned![search_boat_state, filter, selected, search_name, toast_svc];
        boat_list_service(
            rx,
            search_boat_state,
            selected,
            filter,
            search_name,
            session_type,
            created_at_time,
            toast_svc,
        )
    });

    // Initialize the bage by fetching the msgs.
    use_future(move || async move {
        boat_svc.send(BoatListMsg::Fetch);
    });

    // If the ID is populated, then use it to fetch the existing set of boats for a specific batch.
    use_future(move || {
        async move {
            if let Some(id) = mode.as_option() {
                match get_existing_batch(id).await {
                    Ok(batch) => {
                        // This could be done better by using a different future that gets the date from the batch itself,
                        // but because the datetime _should_be_ the same between the batch and its constituent events, and batches should have at least one item,
                        // this will be fine.
                        if let Some(time) = batch.iter().next().map(|x| x.0.recorded_at) {
                            created_at_time.set(crate::ui::util::time::render_local(time))
                        }
                        if let Some(use_scenario) = batch.iter().next().map(|x| x.0.use_scenario) {
                            session_type.set(use_scenario)
                        }

                        // TODO  make the above use a distinct db query.
                        let batch = batch
                            .into_iter()
                            .map(|(_event, boat)| boat)
                            .collect::<Vec<_>>();
                        selected.set(batch);

                        // TODO also set the time element (when we add one) corresponding to the batch in question.
                    }
                    Err(error) => toast_svc.send(ToastMsgMsg::Add(
                        ToastData::from(error),
                        ToastData::DEFAULT_TIME,
                    )),
                }
            }
        }
    });

    rsx! {
        div {
            // I don't love the magic number (42px corresponds to the nav height)
            class: "flex flex-row overflow-hidden divide-x-4 grow max-h-[calc(100vh-42px)]",
            list_pane::BatchListPane {
                boats: selected,
                boat_svc,
                mode,
                session_type,
                created_at_time,
            }
            {
                if mode.is_view() {
                    None
                } else {
                    rsx!{
                        search_pane::BoatSearchPane{
                            boats: search_boat_state,
                            filter,
                            search_name,
                            boat_svc,
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
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    tracing::info!(?search_name, ?filter);
    conn.interact(|conn| Boat::get_boats3(conn, filter, search_name).map_err(ServerFnError::from))
        .await?
}

#[server(SubmitBoats)]
pub(crate) async fn submit_boats(
    boat_ids: Vec<BoatId>,
    session_type: UseScenario,
    recorded_at: NaiveDateTime,
) -> Result<BatchId, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    let new_batch = crate::db::use_event_batch::NewBatchArgs {
        boat_ids,
        batch: crate::db::use_event_batch::NewBatch {
            use_scenario: session_type,
            recorded_at,
        },
    };
    conn.interact(|conn| {
        crate::db::use_event_batch::UseEventBatch::create_batch(conn, new_batch)
            .map_err(ServerFnError::from)
    })
    .await?
}
#[server(GetExistingBatch)]
pub(crate) async fn get_existing_batch(
    batch_id: BatchId,
) -> Result<Vec<(UseEvent, Boat)>, ServerFnError> {
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    conn.interact(move |conn| {
        crate::db::use_event_batch::UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)
            .map_err(ServerFnError::from)
    })
    .await?
}

#[server(ReplaceBatch)]
pub(crate) async fn replace_batch(
    batch_id: BatchId,
    boat_ids: Vec<BoatId>,
    use_type: Option<UseScenario>,
) -> Result<(), ServerFnError> {
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    conn.interact(move |conn| {
        // currently don't overwrite the recorded at field, because we don't support customizing it in the first place
        crate::db::use_event_batch::UseEventBatch::replace_batch_uses(
            conn, batch_id, boat_ids, use_type, None,
        )
        .map_err(ServerFnError::from)
        .map(|_| ())
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
    SaveChanges {
        batch_id: BatchId,
        boat_ids: Vec<BoatId>,
    },
    /// Submits the selected boats, saving use events for each one.
    Submit,
}

async fn boat_list_service(
    mut rx: UnboundedReceiver<BoatListMsg>,
    mut searched_boats: Signal<Vec<Boat>>,
    mut selected_boats: Signal<Vec<Boat>>,
    mut filter: Signal<BoatFilter3>,
    mut search_name: Signal<Option<String>>,
    session_type: Signal<UseScenario>,
    mut created_at_time: Signal<String>,
    toasts: Coroutine<ToastMsgMsg>,
) {
    use futures::stream::StreamExt;

    let search = {
        to_owned![filter, search_name];
        move || async move {
            let _ = search_boats(filter.read().to_owned(), search_name.read().to_owned())
                .await
                .map(|boats| {
                    let exclude: HashSet<BoatId> =
                        selected_boats.read().iter().map(|x| x.id).collect();
                    boats
                        .into_iter()
                        .filter(|x| !exclude.contains(&x.id))
                        .collect()
                })
                .map(|x| searched_boats.set(x));
        }
    };

    while let Some(msg) = rx.next().await {
        match msg {
            BoatListMsg::Fetch => {
                tracing::info!("fetching");
                search().await
            }
            BoatListMsg::SaveChanges { batch_id, boat_ids } => {
                let session_type = *session_type.read();
                tracing::info!(?batch_id, ?boat_ids, "Overwriting old batch with new data");
                match replace_batch(batch_id, boat_ids, Some(session_type)).await {
                    Ok(_) => {
                        toasts.send(ToastData::info(format!("Edited batch {batch_id}")).into())
                    }
                    Err(error) => toasts.send(ToastData::error(error).into()),
                }
            }
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
                    BoatFilter3 {
                        _x: current._x,
                        num_seats: current.num_seats,
                        coxed: current.coxed,
                        oars_config,
                    }
                });
                search().await;
            }
            BoatListMsg::SetFilterCoxed(coxed) => {
                filter.set({
                    let current = filter.read();
                    BoatFilter3 {
                        _x: current._x,
                        num_seats: current.num_seats,
                        coxed,
                        oars_config: current.oars_config,
                    }
                });
                search().await;
            }
            BoatListMsg::SetFilterNumSeats(num_seats) => {
                filter.set({
                    let current = filter.read();
                    BoatFilter3 {
                        _x: current._x,
                        num_seats,
                        coxed: current.coxed,
                        oars_config: current.oars_config,
                    }
                });
                search().await;
            }
            BoatListMsg::AddToBatch(id) => {
                tracing::info!(?id, "adding to batch");
                let mut boat_to_add = None;
                let new_searched_boats = {
                    searched_boats
                        .read()
                        .iter()
                        .cloned()
                        .filter_map(|boat| {
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
                if let Some(boat) = boat_to_add {
                    selected_boats.write().push(boat);
                }
            }
            BoatListMsg::RemoveFromBatch(id) => {
                let mut boat_to_remove = None;
                // O(n), retain the items that aren't the one at the id
                let new_selected_boats = {
                    selected_boats
                        .read()
                        .iter()
                        .cloned()
                        .filter_map(|boat| {
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
                search().await;
            }
            BoatListMsg::Submit => {
                let ids: Vec<BoatId> = selected_boats.read().iter().map(|b| b.id).collect();
                let session_type = *session_type.read();
                let recorded_at = created_at_time();
                tracing::trace!(recorded= ?recorded_at.trim());

                // let date_result = DateTime::<FixedOffset>::parse_from_str(recorded_at.trim(), crate::ui::util::time::MINUTE_RESOLUTION_FMT);
                let date_result =
                    crate::ui::util::time::parse_str_as_naive_to_utc(recorded_at.trim());
                match date_result {
                    Ok(recorded_at) => {
                        // This recorded at should be input + 4h
                        tracing::info!(?recorded_at, input = created_at_time());
                        if !ids.is_empty() {
                            match submit_boats(ids, session_type, recorded_at).await {
                                Ok(id) => {
                                    tracing::info!(%id, "Created batch");
                                    searched_boats.set(Vec::new());
                                    selected_boats.set(Vec::new());
                                    search_name.set(None);
                                    created_at_time.set(crate::ui::util::time::render_local(
                                        chrono::Utc::now().naive_utc(),
                                    ));
                                    toasts.send(ToastData::success("Submitted boats").into());
                                    // refresh the search page
                                    search().await
                                }
                                Err(error) => {
                                    tracing::error!(?error, "Could not submit batch");
                                    toasts.send(
                                        ToastData::warn("Could not submit batch".to_string())
                                            .into(),
                                    );
                                }
                            }
                        }
                    }
                    Err(error) => {
                        toasts.send(ToastData::error(error).into());
                    }
                }
            }
        }
    }
}
