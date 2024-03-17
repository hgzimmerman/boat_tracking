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

use crate::db::{boat::{types::BoatId, Boat, BoatFilter2}, use_event::UseScenario};

#[component]
pub fn BatchCreationPage(cx: Scope) -> Element {
    let selected = use_state(cx, || Vec::<Boat>::new());
    let filter = use_state(cx, || BoatFilter2::None);
    let search_name = use_state(cx, || Option::<String>::None);
    // let boat_search = use_server_future(cx, (filter.get(),), |(filter,)| async move {
    //     search_boats(filter.to_owned()).await
    // })?;

    let search_boat_state = use_state(cx,  || Vec::<Boat>::new());
    let boat_svc = use_coroutine(cx, |rx| {
        to_owned![search_boat_state, filter, selected, search_name];
        boat_list_service(rx, search_boat_state, selected, filter, search_name)
    });
    // boat_svc.send(BoatListMsg::Fetch);

    cx.render(rsx!{
        div {
            class: "flex flex-row max-h-full overflow-hide divide-x-2 grow",
            BatchListPane {
                boats: selected,
                boat_svc: boat_svc
            }
            BoatSearchPane{
                boats: search_boat_state,
                filter: filter,
                search_name: search_name,
                boat_svc: boat_svc
            }
        }
    })
}
#[server(GetBoats)]
pub(crate) async fn search_boats(
    filter: BoatFilter2,
    search_name: Option<String>, 
) -> Result<Vec<Boat>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu"); 
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;
    tracing::info!(?search_name);
    conn 
        .interact(|conn| {
            Boat::get_boats(conn, filter, search_name)
            .map_err(ServerFnError::from)
            
        })
        .await
        .map_err(ServerFnError::from)?
}
enum BoatListMsg {
    /// Run the fetch
    Fetch,
    SetFilter(BoatFilter2),
    SetSearch(String),
    AddToBatch(BoatId),
    RemoveFromBatch(BoatId),
}


async fn boat_list_service(
    mut rx: UnboundedReceiver<BoatListMsg>,
    searched_boats: UseState<Vec<Boat>>,
    selected_boats: UseState<Vec<Boat>>,
    filter: UseState<BoatFilter2>,
    search_name: UseState<Option<String>>
) {
    use futures::stream::StreamExt;
    while let Some(msg) = rx.next().await {
        match msg {
            BoatListMsg::Fetch => {
                tracing::info!("fetching");
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
            },
            BoatListMsg::SetSearch(search) => {
                tracing::info!(%search, "setting search");
                if search.is_empty() {
                    search_name.set(None);
                } else {
                    search_name.set(Some(search));
                }
            }
            BoatListMsg::SetFilter(new_filter) => {
                tracing::info!(?new_filter, "set filter");
                filter.set(new_filter);
            },
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
        }
        
    }
}

/// The primary pane for seeing which boats will be saved, as well as controls for saving, adding, etc... 
#[component]
fn BatchListPane<'a>(
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
            class: "flex flex-col grow overflow-auto divide-y",
            // The list of boats
            div {
                class: "flex flex-col grow overflow-auto",
                boats.iter().map(|b| rsx!{
                    div {
                        class: "flex flex-row h-4",
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
                        // boat_svc.send(BoatListMsg::Fetch);
                    },
                    class: "flex flex-col h-30",
                    div {
                        class: "inline-flex rounded-md shadow-sm m-4",
                        role: "group",
                        button {
                            class:"btn btn-blue",
                            "data-dropdown-toggle": "dropdown",
                            onclick: move |e| {
                                e.stop_propagation();
                                show_session_type_dropdown.set(!show_session_type_dropdown.get())
                            },
                            session_type.get().to_string()
                        }
                        div {
                            id: "dropdown",
                            class: if *show_session_type_dropdown.get() {
                                "relative bg-white divide-y rounded-lg -top-45 shadow w-44 dark:bg-gray-700"
                            } else {
                                "hidden"
                            },
                            ul {
                                li {
                                    onclick: |_| session_type.set(UseScenario::AM),
                                    "AM"
                                }
                                li {
                                    onclick: |_| session_type.set(UseScenario::PM),
                                    "PM"
                                }
                                li {
                                    onclick: |_| session_type.set(UseScenario::Regatta),
                                    "Regatta"
                                }
                                li {
                                    onclick: |_| session_type.set(UseScenario::Other),
                                    "Other"
                                }
                            }
                        }
                        button {
                            class: "btn btn-blue",
                            onclick: move |e| {
                                e.stop_propagation();
                                // boat_svc.send(BoatListMsg::Fetch);
                            },
                            "Save Boats"
                        }
                    }
                    
                }
            }

        }
    })
}

#[component]
fn BoatSearchPane<'a>(
    cx: Scope, 
    boats: &'a [Boat], 
    filter: &'a BoatFilter2,
    search_name: &'a Option<String>,
    boat_svc: &'a Coroutine<BoatListMsg>
) -> Element {
    cx.render(rsx!{
        div {
            class: "flex flex-col w-1/2 overflow-auto divide-y",
            form {
                class: "flex flex-col h-30 m-4",
                onsubmit: move |e| {
                    e.stop_propagation();
                    boat_svc.send(BoatListMsg::Fetch);
                },
                input {
                    r#type:"text",
                    id: "boat_search",
                    class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    placeholder:  "Boat Name",
                    value: search_name.as_deref(),
                    oninput: |event| {
                        boat_svc.send(BoatListMsg::SetSearch(event.value.clone()));
                    }
                }
                button {
                    class: "m-4 btn btn-blue",
                    onclick: move |e| {
                        e.stop_propagation();
                        boat_svc.send(BoatListMsg::Fetch);
                    },
                    "search"
                }
            }
            div {
                class: "flex flex-col grow",
                boats.iter().map(|b| rsx!{
                    div {
                        class: "flex flex-row h-16",
                        div {
                            class: "m-2",
                            b.name.clone()
                        }
                        div {
                            class: "m-2",
                            b.boat_type().as_ref().map(ToString::to_string)
                        }
                        button {
                            class: "m-2 btn btn-blue",
                            onclick: move |_| {
                                boat_svc.send(BoatListMsg::AddToBatch(b.id.clone()));
                            },
                            "Add to batch"
                        }
                    }
                }) 
            }
        } 
    })
}