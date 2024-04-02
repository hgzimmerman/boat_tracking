use super::{BatchPageMode, BoatListMsg};
use crate::db::{boat::Boat, use_event::UseScenario};
use dioxus::prelude::*;

mod submit_row;
use self::submit_row::SubmitRow;

/// The primary pane for seeing which boats will be saved, as well as controls for saving, adding, etc...
#[component]
pub(super) fn BatchListPane(
    boats: Signal<Vec<Boat>>,
    boat_svc: Coroutine<BoatListMsg>,
    mode: BatchPageMode,
    session_type: Signal<UseScenario>,
    created_at_time: Signal<String>,
) -> Element {
    rsx! {
        // The pane
        div {
            class: "flex flex-col grow divide-y-2 min-w-1/2",
            // The list of boats
            List {
                boats: boats.read().clone(),
                boat_svc: boat_svc,
                mode: mode
            }
            // Submission form
            SubmitRow {
                boats: boats.read().clone(),
                boat_svc,
                mode,
                session_type,
                created_at_time
            }
        }
    }
}

#[component]
fn List(boats: Vec<Boat>, boat_svc: Coroutine<BoatListMsg>, mode: BatchPageMode) -> Element {
    rsx! {
        div {
            class: "flex flex-col grow overflow-auto divide-y",
            {
                boats.into_iter().map(|b| rsx!{
                    div {
                        class: "flex flex-row h-16 items-center",
                        div {
                            class: "m-2 grow",
                            {b.name.clone()}
                        }
                        div {
                            class: "m-2",
                            {b.boat_type().as_ref().map(ToString::to_string)}
                        }
                        {
                            if mode.is_view() {
                                // We don't want a remove button when just viewing the page.
                                None
                            } else {
                                rsx!{
                                    button {
                                        class: "m-2 btn btn-red",
                                        onclick: move |_| {
                                            boat_svc.send(BoatListMsg::RemoveFromBatch(b.id));
                                        },
                                        "Remove"
                                    }
                                }
                            }
                        }

                    }
                })
            }
        }
    }
}
