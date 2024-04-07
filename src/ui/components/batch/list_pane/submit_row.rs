use super::*;

#[component]
pub(super) fn SubmitRow(
    boats: Vec<Boat>,
    boat_svc: Coroutine<BoatListMsg>,
    mode: BatchPageMode,
    session_type: Signal<UseScenario>,
    created_at_time: Signal<String>,
) -> Element {
    rsx! {
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
                    SessionTypeDropup {
                        boat_svc,
                        mode,
                        session_type
                    }
                    TimeSelector {
                        boat_svc,
                        mode,
                        created_at_time
                    }

                    div {
                        class: "grow",
                    }
                    SubmitButton{
                        boat_svc,
                        mode,
                        boats
                    }
                }
            }
        }
    }
}

/// The form element that allows selecting the sort of practice or regatta
///
/// This could be select element, but this custom thing is fine for now.
#[component]
fn SessionTypeDropup(
    boat_svc: Coroutine<BoatListMsg>,
    mode: BatchPageMode,
    session_type: Signal<UseScenario>,
) -> Element {
    let mut show_session_type_dropdown = use_signal(|| false);
    rsx! {
        div {
            class: "my-0",
            button {
                id: "session-dropdown-btn",
                class: "btn min-w-44 rounded-lg bg-gray-700 text-white",
                onclick: move |e| {
                    e.stop_propagation();
                    if !mode.is_view() {
                        let inverted = !*show_session_type_dropdown.read();
                        show_session_type_dropdown.set(inverted);
                    }
                },
                onmouseover: move |e| {
                    e.stop_propagation();
                    if !mode.is_view() {
                        show_session_type_dropdown.set(true);
                    }
                },
                onmouseout: move |e| {
                    e.stop_propagation();
                    if !mode.is_view() {
                        show_session_type_dropdown.set(false);
                    }
                },
                {session_type.read().to_string()}
                // the dropdown
                div {
                    id: "session-dropdown-positioner",
                    class: "relative h-0 w-0 top-2",
                    div {
                        id: "session-dropdown",
                        class: if *show_session_type_dropdown.read() {
                            "absolute z-10 py-2 w-40 bottom-8 left-4 origin-bottom-right rounded-md bg-white dark:bg-slate-500 shadow-lg divide-y m-2 text-slate-600 dark:text-slate-50 font-normal"
                        } else {
                            "hidden"
                        },
                        ul {
                            {
                                [
                                    UseScenario::YouthGgrc,
                                    UseScenario::YouthSomerville,
                                    UseScenario::Adult,
                                    UseScenario::LearnToRow,
                                    UseScenario::ScullingSaturday,
                                    UseScenario::PrivateSession,
                                    UseScenario::Regatta,
                                    UseScenario::Other
                                ]
                                .into_iter()
                                .map(|use_scenario| {
                                    let active = session_type() == use_scenario;
                                    rsx!{
                                        li {
                                            class: "hover:bg-slate-300 dark:hover:bg-slate-700",
                                            class: if active {"bg-slate-200 dark:bg-slate-600"},
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                session_type.set(use_scenario);
                                                show_session_type_dropdown.set(false);
                                            },
                                            {use_scenario.to_string()}
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TimeSelector(
    boat_svc: Coroutine<BoatListMsg>,
    mode: BatchPageMode,
    created_at_time: Signal<String>,
) -> Element {
    rsx! {
        div {
            class: "my-0",
            input {
                r#type: "datetime-local",
                id: "manufactured-at",
                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                value: created_at_time.read().to_owned(),
                disabled: mode.is_view(),
                oninput: move |event| {
                    created_at_time.set(event.value())
                }
            }
        }
    }
}

#[component]
fn SubmitButton(
    boats: Vec<Boat>,
    boat_svc: Coroutine<BoatListMsg>,
    mode: BatchPageMode,
) -> Element {
    match mode {
        BatchPageMode::Create | BatchPageMode::Template { .. } => rsx! {
            button {
                class: "inline-flex items-center btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                disabled: boats.is_empty(),
                onclick: move |e| {
                    e.stop_propagation();
                    boat_svc.send(BoatListMsg::Submit);
                },
                MaskIcon {
                    class: "fill-current w-4 h-4 mr-1 bg-white",
                    url: "/save.svg"
                }
                span {
                    "Save New Boat Uses"
                }
            }
        },
        BatchPageMode::View { .. } => None,
        BatchPageMode::Edit { id } => rsx! {
            button {
                class: "inline-flex items-center btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                disabled: boats.is_empty(),
                onclick: move |e| {
                    e.stop_propagation();
                    boat_svc.send(BoatListMsg::SaveChanges { batch_id: id, boat_ids: boats.iter().map(|boat|boat.id).collect()});
                },
                MaskIcon {
                    class: "fill-current w-4 h-4 mr-1 bg-white",
                    url: "/save.svg"
                }
                span {
                    "Save Changes"
                }
            }
        },
    }
}
