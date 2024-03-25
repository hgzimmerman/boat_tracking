use dioxus::prelude::*;
use crate::db::{boat::Boat, use_event::UseScenario};
use super::{BatchPageMode, BoatListMsg};


/// The primary pane for seeing which boats will be saved, as well as controls for saving, adding, etc... 
#[component]
pub(super) fn BatchListPane(
    boats: Signal<Vec<Boat>>,
    boat_svc: Coroutine<BoatListMsg>,
    mode: BatchPageMode
) -> Element {
    rsx!{
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
                boat_svc: boat_svc,
                mode: mode
            }
        }
    }
}

#[component]
fn List(
    boats: Vec<Boat>,
    boat_svc: Coroutine<BoatListMsg>,
    mode: BatchPageMode
) -> Element {
    rsx!{
        div {
            class: "flex flex-col grow overflow-auto divide-y",
            {boats.into_iter().map(|b| rsx!{
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
                        match mode {
                            BatchPageMode::View { .. } => {
                                None
                            },
                            _ => rsx!{
                                button {
                                    class: "m-2 btn btn-red",
                                    onclick: move |_| {
                                        boat_svc.send(BoatListMsg::RemoveFromBatch(b.id.clone()));
                                    },
                                    "Remove"
                                }
                            } 
                        }
                    }
                    
                }
            }) }
        }
    }
}


#[component]
fn SubmitRow(
    boats: Vec<Boat>,
    boat_svc: Coroutine<BoatListMsg>,
    mode: BatchPageMode
) -> Element {

    // TODO make this use the current time of day to initialize it.
    let mut session_type = use_signal(|| UseScenario::AM);
    let mut show_session_type_dropdown = use_signal(|| false); 


    rsx!{
        div {
            form {
                onclick: move |e| {
                    e.stop_propagation();
                },
                class: "flex flex-col h-30",
                div {
                    id: "button-group",
                    class: "inline-flex rounded-md shadow-sm m-4",
                    role: "group",
                    button {
                        id: "session-dropdown-btn",
                        class: "btn btn-blue min-w-28 rounded-s ",
                        onclick: move |e| {
                            e.stop_propagation();
                            let inverted = !*show_session_type_dropdown.read();
                            show_session_type_dropdown.set(inverted);
                        },
                        onmouseover: move |e| {
                            e.stop_propagation();
                            show_session_type_dropdown.set(true);
                        },
                        onmouseout: move |e| {
                            e.stop_propagation();
                            show_session_type_dropdown.set(false);
                        },
                        {session_type.read().to_string()}
                        // the dropdown
                        div {
                            id: "session-dropdown-positioner",
                            class: "relative h-0 w-0",
                            div {
                                id: "session-dropdown",
                                class: if *show_session_type_dropdown.read() {
                                    "absolute z-10 mt-2 w-20 bottom-8 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y p-2 text-slate-600 font-normal"
                                } else {
                                    "hidden"
                                },
                                ul {
                                    li {
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            *session_type.write() = UseScenario::AM;
                                            *show_session_type_dropdown.write() = false;
                                        },
                                        "AM"
                                    }
                                    li {
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            session_type.set(UseScenario::PM);
                                            show_session_type_dropdown.set(false);
                                        },
                                        "PM"
                                    }
                                    li {
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            session_type.set(UseScenario::Regatta);
                                            show_session_type_dropdown.set(false);
                                        },
                                        "Regatta"
                                    }
                                    li {
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            session_type.set(UseScenario::Other);
                                            show_session_type_dropdown.set(false);
                                        },
                                        "Other"
                                    }
                                }
                            }
                        }
                    }
                    {
                        match mode {
                            BatchPageMode::Create | BatchPageMode::Template { .. } => rsx!{
                                button {
                                    class: "btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                                    disabled: boats.is_empty(),
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        boat_svc.send(BoatListMsg::Submit);
                                    },
                                    "Save New Boat Uses"
                                }
                            },
                            BatchPageMode::View { .. } => None,
                            BatchPageMode::Edit { id } => rsx!{
                                button {
                                    class: "btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                                    disabled: boats.is_empty(),
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        boat_svc.send(BoatListMsg::SaveChanges { batch_id: id, boat_ids: boats.iter().map(|boat|boat.id).collect()});
                                    },
                                    "Save Changes"
                                }
                            },
                        }
                    }
                    
                } 
            }
        }
    }
}