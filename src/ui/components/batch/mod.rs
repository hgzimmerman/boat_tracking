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
            Boat, BoatFilter,
        },
        use_event::{UseEvent, UseScenario},
        use_event_batch::{BatchId, UseEventBatch},
    },
    ui::components::{
        batch::service::{boat_list_service, get_existing_batch, BoatListMsg, ExistingBatch},
        toast::ToastData,
    },
};
use chrono::NaiveDateTime;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

mod list_pane;
mod search_pane;
mod service;

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
    /// Create a completely new batch.
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
    /// Gets the id for the batch if available.
    ///
    /// It isn't available if the batch is being created.
    pub fn as_option(self) -> Option<BatchId> {
        match self {
            BatchPageMode::Create => None,
            BatchPageMode::Template { id }
            | BatchPageMode::Edit { id }
            | BatchPageMode::View { id } => Some(id),
        }
    }
    /// Is the page intended for viewing only?
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
    let filter = use_signal(BoatFilter::default);
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

    // Initialize the page by fetching the boats to possibly select.
    use_future(move || async move {
        boat_svc.send(BoatListMsg::Fetch);
    });

    // If the ID is populated, then use it to fetch the existing set of boats for a specific batch.
    //
    // Use this to set the selected list, the time and the type of session.
    use_future(move || {
        async move {
            if let Some(id) = mode.as_option() {
                match get_existing_batch(id).await {
                    Ok(ExistingBatch {
                        batch,
                        batch_entries,
                    }) => {
                        match mode {
                            BatchPageMode::Create | BatchPageMode::Template { .. } => {}
                            BatchPageMode::View { .. } | BatchPageMode::Edit { .. } => {
                                // Only set the created at time if we are editing or viewing, otherwise,
                                // we want the template to use the current time.
                                if let Some(time) = batch.as_ref().map(|x| x.recorded_at) {
                                    created_at_time.set(crate::ui::util::time::render_local(time))
                                }
                            }
                        }

                        if let Some(use_scenario) = batch.map(|x| x.use_scenario) {
                            session_type.set(use_scenario)
                        }

                        let batch = batch_entries
                            .into_iter()
                            .map(|(_event, boat)| boat)
                            .collect::<Vec<_>>();
                        selected.set(batch);
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
