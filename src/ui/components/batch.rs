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

use crate::db::{boat::{types::BoatId, Boat, BoatFilter3}, use_event::UseScenario, use_event_batch::{BatchId, NewBatch, NewBatchArgs, UseEventBatch}};

#[component]
pub fn BatchCreationPage(cx: Scope) -> Element {
    let selected = use_state(cx, || Vec::<Boat>::new());
    let filter = use_state(cx, BoatFilter3::default);
    let search_name = use_state(cx, || Option::<String>::None);

    let search_boat_state = use_state(cx,  || Vec::<Boat>::new());
    let boat_svc = use_coroutine(cx, |rx| {
        to_owned![search_boat_state, filter, selected, search_name];
        boat_list_service(rx, search_boat_state, selected, filter, search_name)
    });

    cx.render(rsx!{
        div {
            // I don't love the magic number (42px corresponds to the nav height)
            class: "flex flex-row overflow-hide divide-x-4 grow max-h-[calc(100vh-42px)]", 
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

#[server(GetBoats2)]
pub(crate) async fn search_boats(
    filter: BoatFilter3,
    search_name: Option<String>, 
) -> Result<Vec<Boat>, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu"); 
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;
    tracing::info!(?search_name);
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
    SetFilter(BoatFilter3),
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
    search_name: UseState<Option<String>>
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
                        }
                        Err(error) => {
                            tracing::error!(?error, "could not submit batch")
                        }
                    }
                }
            }
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

#[component]
fn BoatSearchPane<'a>(
    cx: Scope, 
    boats: &'a [Boat], 
    filter: &'a BoatFilter3,
    search_name: &'a Option<String>,
    boat_svc: &'a Coroutine<BoatListMsg>
) -> Element {
    let show_filter_dropdown = use_state(cx, || false); 
    cx.render(rsx!{
        div {
            class: "flex flex-col w-1/2 overflow-auto divide-y-2",
            // The submission box
            form {
                class: "flex flex-col h-30 m-4",
                onsubmit: move |e| {
                    e.stop_propagation();
                    boat_svc.send(BoatListMsg::Fetch);
                },
                /* 
                button {
                    id: "filter-dropdown-btn",
                    class: "btn btn-blue min-w-28 rounded-s",
                    onclick: move |e| {
                        e.stop_propagation();
                        show_filter_dropdown.set(!show_filter_dropdown.get());
                    },
                    onmouseover: move |e| {
                        e.stop_propagation();
                        show_filter_dropdown.set(true);
                    },
                    onmouseout: move |e| {
                        e.stop_propagation();
                        show_filter_dropdown.set(false);
                    },
                    format!("{filter:?}")
                    // the dropdown
                    div {
                        id: "filter-dropdown-positioner",
                        class: "relative h-0 w-0",
                        div {
                            id: "filter-dropdown",
                            class: if *show_filter_dropdown.get() {
                                "absolute z-10 mt-2 w-20 top-2 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y p-2 text-slate-600 font-normal"
                            } else {
                                "hidden"
                            },
                            ul {
                                li {
                                    onclick: |e| {
                                        e.stop_propagation();
                                        boat_svc.send(BoatListMsg::SetFilter(BoatFilter2::None));
                                        show_filter_dropdown.set(false);
                                    },
                                    "None"
                                }
                                li {
                                    onclick: |e| {
                                        e.stop_propagation();
                                        boat_svc.send(BoatListMsg::SetFilter(BoatFilter2::ByType(BoatType::Single)));
                                        show_filter_dropdown.set(false);
                                    },
                                    "Single"
                                }
                                li {
                                    onclick: |e| {
                                        e.stop_propagation();
                                        boat_svc.send(BoatListMsg::SetFilter(BoatFilter2::ByType(BoatType::Double)));
                                        show_filter_dropdown.set(false);
                                    },
                                    "Double"
                                }
                                li {
                                    onclick: |e| {
                                        e.stop_propagation();
                                        boat_svc.send(BoatListMsg::SetFilter(BoatFilter2::ByType(BoatType::Quad)));
                                        show_filter_dropdown.set(false);
                                    },
                                    "Quad"
                                }
                                li {
                                    onclick: |e| {
                                        e.stop_propagation();
                                        boat_svc.send(BoatListMsg::SetFilter(BoatFilter2::ByType(BoatType::Four)));
                                        show_filter_dropdown.set(false);
                                    },
                                    "Four"
                                }
                                li {
                                    onclick: |e| {
                                        e.stop_propagation();
                                        boat_svc.send(BoatListMsg::SetFilter(BoatFilter2::ByType(BoatType::Eight)));
                                        show_filter_dropdown.set(false);
                                    },
                                    "Eight"
                                }
                            }
                        }
                    }
                }
                **/
                input {
                    r#type:"text",
                    id: "boat_search",
                    class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    placeholder: "Boat Name",
                    autocomplete: "off",
                    value: search_name.as_deref(),
                    oninput: |event| {
                        boat_svc.send(BoatListMsg::SetSearch(event.value.clone()));
                    }
                } 


            }
            // The search results 
            div {
                class: "flex flex-col grow divide-y",
                boats.iter().map(|b| rsx! {
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