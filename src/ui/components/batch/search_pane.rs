use super::BoatListMsg;
use crate::db::boat::{
    types::{HasCox, OarConfiguration, SeatCount},
    Boat, BoatFilter3,
};
use dioxus::prelude::*;

#[component]
pub(super) fn BoatSearchPane(
    boats: Signal<Vec<Boat>>,
    filter: Signal<BoatFilter3>,
    search_name: Signal<Option<String>>,
    boat_svc: Coroutine<BoatListMsg>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-1/2 overflow-auto divide-y-2",
            // The submission box
            FilterPane {
                filter: filter,
                search_name: search_name,
                boat_svc: boat_svc
            }
            // The search results
            SearchResults {
                boats: boats.read().clone(),
                boat_svc: boat_svc
            }
        }
    }
}

#[component]
fn SearchResults(boats: Vec<Boat>, boat_svc: Coroutine<BoatListMsg>) -> Element {
    rsx! {
        div {
            class: "flex flex-col grow divide-y",
            {boats.into_iter().map(|b| rsx! {
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
                    button {
                        class: "m-2 btn btn-blue",
                        onclick: move |_| {
                            boat_svc.send(BoatListMsg::AddToBatch(b.id.clone()));
                        },
                        "Add to batch"
                    }
                }
            })}
        }
    }
}

#[component]
fn FilterPane(
    filter: Signal<BoatFilter3>,
    search_name: Signal<Option<String>>,
    boat_svc: Coroutine<BoatListMsg>,
) -> Element {
    rsx! {
        form {
            class: "flex flex-col h-30 p-4 bg-white dark:bg-gray-500",
            onsubmit: move |e| {
                e.stop_propagation();
                boat_svc.send(BoatListMsg::Fetch);
            },
            div {
                class: "flex flex-row gap-x-4 flex-wrap justify-items-center",
                div {
                    class: "grow",
                    label {
                        r#for: "select-oar-config",
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Oar Configuration"
                    }
                    select {
                        id: "select-oar-config",
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        option {
                            selected: filter.read().oars_config.is_none(),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterOarConfig(None));
                            },
                            value: "None",
                            "None"
                        }
                        option {
                            selected: filter.read().oars_config == Some(OarConfiguration::Sweep),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterOarConfig(Some(OarConfiguration::Sweep)));
                            },
                            value: "Sweep",
                            "Sweep"
                        }
                        option {
                            selected: filter.read().oars_config == Some(OarConfiguration::Scull),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterOarConfig(Some(OarConfiguration::Scull)));
                            },
                            value: "Scull",
                            "Scull"
                        }
                    }
                }
                div {
                    class: "grow",
                    label {
                        r#for: "select-coxed",
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Filter by Coxed"
                    }
                    select {
                        id: "select-coxed",
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        option {
                            selected: filter.read().coxed.is_none(),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterCoxed(None));
                            },
                            value: "None",
                            "None"
                        }
                        option {
                            selected: filter.read().coxed == Some(HasCox::new(true)),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterCoxed(Some(HasCox::new(true))));
                            },
                            value: "Coxed",
                            "Coxed"
                        }
                        option {
                            selected: filter.read().coxed == Some(HasCox::new(false)),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterCoxed(Some(HasCox::new(false))));
                            },
                            value: "Coxless",
                            "Coxless"
                        }
                    }
                }

                div {
                    class: "grow",
                    label {
                        r#for: "select-num-seats",
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Filter by Coxed"
                    }
                    select {
                        id: "select-num-seats",
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        option {
                            selected: filter.read().num_seats.is_none(),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterNumSeats(None));
                            },
                            value: "None",
                            "None"
                        }
                        option {
                            selected: filter.read().num_seats.as_ref().map(SeatCount::count).unwrap_or_default() == 1,
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterNumSeats(SeatCount::new(1)));
                            },
                            value: "1",
                            "1"
                        }
                        option {
                            selected: filter.read().num_seats.as_ref().map(SeatCount::count).unwrap_or_default() == 2,
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterNumSeats(SeatCount::new(2)));
                            },
                            value: "2",
                            "2"
                        }
                        option {
                            selected: filter.read().num_seats.as_ref().map(SeatCount::count).unwrap_or_default() == 4,
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterNumSeats(SeatCount::new(4)));
                            },
                            value: "4",
                            "4"
                        }
                        option {
                            selected: filter.read().num_seats.as_ref().map(SeatCount::count).unwrap_or_default() == 8,
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(BoatListMsg::SetFilterNumSeats(SeatCount::new(8)));
                            },
                            value: "8",
                            "8"
                        }
                    }
                }
            }
            input {
                r#type:"text",
                id: "boat_search",
                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                placeholder: "Boat Name",
                autocomplete: "off",
                value: search_name.read().as_deref(),
                oninput: move |event| {
                    boat_svc.send(BoatListMsg::SetSearch(event.value().clone()));
                }
            }
        }
    }
}
