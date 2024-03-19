use dioxus::prelude::*;
use crate::db::{boat::Boat, use_event::UseScenario};
use super::BoatListMsg;


/// The primary pane for seeing which boats will be saved, as well as controls for saving, adding, etc... 
#[component]
pub(super) fn BatchListPane<'a>(
    cx: Scope, 
    boats: &'a [Boat],
    boat_svc: &'a Coroutine<BoatListMsg>
) -> Element {

    let show_session_type_dropdown = use_state(cx, || false); 
    // TODO make this use the current time of day to initialize it.
    let session_type = use_state(cx, || UseScenario::AM);  
    cx.render(rsx!{
        // The pane
        div {
            class: "flex flex-col grow divide-y-2 min-w-1/2",
            // The list of boats
            div {
                class: "flex flex-col grow overflow-auto divide-y",
                boats.iter().map(|b| rsx!{
                    div {
                        class: "flex flex-row h-16 items-center",
                        div {
                            class: "m-2",
                            b.name.clone()
                        }
                        div {
                            class: "m-2",
                            b.boat_type().as_ref().map(ToString::to_string)
                        }
                        button {
                            class: "m-2 btn btn-red",
                            onclick: move |_| {
                                boat_svc.send(BoatListMsg::RemoveFromBatch(b.id.clone()));
                            },
                            "Remove"
                        }
                    }
                }) 
            }
            // Submission form
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
                                show_session_type_dropdown.set(!show_session_type_dropdown.get());
                            },
                            onmouseover: move |e| {
                                e.stop_propagation();
                                show_session_type_dropdown.set(true);
                            },
                            onmouseout: move |e| {
                                e.stop_propagation();
                                show_session_type_dropdown.set(false);
                            },
                            session_type.get().to_string()
                            // the dropdown
                            div {
                                id: "session-dropdown-positioner",
                                class: "relative h-0 w-0",
                                div {
                                    id: "session-dropdown",
                                    class: if *show_session_type_dropdown.get() {
                                        "absolute z-10 mt-2 w-20 bottom-8 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y p-2 text-slate-600 font-normal"
                                    } else {
                                        "hidden"
                                    },
                                    ul {
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                session_type.set(UseScenario::AM);
                                                show_session_type_dropdown.set(false);
                                            },
                                            "AM"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                session_type.set(UseScenario::PM);
                                                show_session_type_dropdown.set(false);
                                            },
                                            "PM"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                session_type.set(UseScenario::Regatta);
                                                show_session_type_dropdown.set(false);
                                            },
                                            "Regatta"
                                        }
                                        li {
                                            onclick: |e| {
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
                        
                        button {
                            class: "btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                            disabled: boats.is_empty(),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::Submit);
                            },
                            "Save Boat Uses"
                        }
                    }
                    
                }
            }

        }
    })
}