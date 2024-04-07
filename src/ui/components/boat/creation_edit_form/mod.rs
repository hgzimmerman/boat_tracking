use crate::{
    db::boat::types::{BoatId, BoatType, WeightClass},
    ui::components::boat::creation_edit_form::service::CreateBoatMsg,
};
use dioxus::prelude::*;

mod edit_tab;
pub use edit_tab::*;
mod new_boat_page;
pub use new_boat_page::*;
mod service;

/// A toggle used to control how the form behaves, as it can serve two purposes
/// - creation of a new boat or updating an existing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoatFormMode {
    /// A new boat should be created.
    New,
    /// An existing boat with the provided id should be updated.
    Edit(BoatId),
}

/// The Options for boat weights
const WEIGHTS: [Option<WeightClass>; 5] = [
    None, 
    Some(WeightClass::Light), 
    Some(WeightClass::Medium), 
    Some(WeightClass::Heavy), 
    Some(WeightClass::Tubby)
];
/// The boat type options.
/// Excludes some of the more exodic types because we don't ever plan on using them.
const BOAT_TYPES: [Option<BoatType>; 9] = [
    None, 
    Some(BoatType::Single), 
    Some(BoatType::Double), 
    Some(BoatType::Quad),
    Some(BoatType::QuadPlus),
    Some(BoatType::Pair), 
    Some(BoatType::Four), 
    Some(BoatType::FourPlus), 
    Some(BoatType::Eight), 
];

/// The common form component that handles creation and upating of boats.
#[component]
fn BoatForm(
    name: Signal<String>,
    acquired_at: Signal<String>,
    manufactured_at: Signal<String>,
    relinquished_at: Option<Signal<String>>,
    boat_type: Signal<Option<BoatType>>,
    weight_class: Signal<Option<WeightClass>>,
    mode: BoatFormMode,
) -> Element {
    let mut show_weight_class_dropdown = use_signal(|| false);
    let mut show_boat_type_dropdown = use_signal(|| false);

    let boat_svc = use_coroutine_handle::<CreateBoatMsg>();

    let title = use_memo(move || match mode {
        BoatFormMode::New => "Add a new boat".to_string(),
        BoatFormMode::Edit(_) => format!("Edit {name}"),
    });

    rsx! {
        form {
            class: "bg-gray-100 shadow-md rounded px-8 pt-6 pb-8 mb-4 dark:bg-gray-600 min-w-96 max-w-lg w-1/2",
            onsubmit: move |event| {
                event.stop_propagation();
            },
            h2 {
                class: "mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 md:text-2xl lg:text-3xl dark:text-white",
                {title}
            }
            div {
                class: "mb-4",
                label {
                    r#for: "boat_name",
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Boat Name"
                }
                input {
                    r#type: "text",
                    id: "boat_name",
                    class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    placeholder: "Boat Name",
                    autocomplete: "off",
                    value: name.read().to_owned(),
                    oninput: move |event| {
                        event.stop_propagation();
                        name.set(event.value())
                    }
                }
            }
            div {
                class: "mb-4",
                label {
                    r#for: "weight-class-dropdown-btn",
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Weight Class"
                }
                button {
                    id: "weight-class-dropdown-btn",
                    class: "btn btn-blue min-w-28 rounded-s",
                    onclick: move |e| {
                        e.stop_propagation();
                        let inverted = !*show_weight_class_dropdown.read();
                        show_weight_class_dropdown.set(inverted);
                    },
                    onmouseover: move |e| {
                        e.stop_propagation();
                        show_weight_class_dropdown.set(true);
                    },
                    onmouseout: move |e| {
                        e.stop_propagation();
                        show_weight_class_dropdown.set(false);
                    },
                    {
                        weight_class().as_ref().map(ToString::to_string).unwrap_or_else(|| "None".to_string())
                    }
                    // the dropdown
                    div {
                        id: "weight-class-dropdown-positioner",
                        class: "relative h-0 w-0",
                        div {
                            id: "weight-class-dropdown",
                            class: if *show_weight_class_dropdown.read() {
                                "absolute z-10 py-2 w-20 top-0 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y m-2 text-slate-600 font-normal"
                            } else {
                                "hidden"
                            },
                            ul {
                                {
                                    WEIGHTS
                                    .iter()
                                    .map(|weight| {
                                        let active = weight_class() == *weight;
                                        rsx! {
                                            li {
                                                class: "hover:bg-slate-300",
                                                class: if active {"bg-slate-200"},
                                                onclick: move |e| {
                                                    e.stop_propagation();
                                                    weight_class.set(*weight);
                                                    show_weight_class_dropdown.set(false);
                                                },
                                                {weight.as_ref().map(ToString::to_string).unwrap_or_else(|| "None".to_string())} 
                                            }
                                        }
                                    })
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "mb-4",
                label {
                    r#for: "boat-type-dropdown-btn",
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Boat Type"
                }
                button {
                    id: "boat-type-dropdown-btn",
                    class: "btn btn-blue min-w-28 rounded-s",
                    onclick: move |e| {
                        e.stop_propagation();
                        let inverted = !*show_boat_type_dropdown.read();
                        show_boat_type_dropdown.set(inverted);
                    },
                    onmouseover: move |e| {
                        e.stop_propagation();
                        show_boat_type_dropdown.set(true);
                    },
                    onmouseout: move |e| {
                        e.stop_propagation();
                        show_boat_type_dropdown.set(false);
                    },
                    {
                        boat_type().as_ref().map(ToString::to_string).unwrap_or_else(|| "None".to_string())
                    }
                    // the dropdown
                    div {
                        id: "boat-type-dropdown-positioner",
                        class: "relative h-0 w-0",
                        div {
                            id: "boat-type-dropdown",
                            class: if *show_boat_type_dropdown.read() {
                                "absolute z-10 py-2 w-20 top-0 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y m-2 text-slate-600 font-normal"
                            } else {
                                "hidden"
                            },
                            ul {
                                {
                                   BOAT_TYPES 
                                    .iter()
                                    .map(|bt| {
                                        let active = boat_type() == *bt;
                                        rsx! {
                                            li {
                                                class: "hover:bg-slate-300",
                                                class: if active {"bg-slate-200"},
                                                onclick: move |e| {
                                                    e.stop_propagation();
                                                    boat_type.set(*bt);
                                                    show_boat_type_dropdown.set(false);
                                                },
                                                {bt.as_ref().map(ToString::to_string).unwrap_or_else(|| "None".to_string())} 
                                            }
                                        }
                                    })
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "mb-4",
                label {
                    r#for: "acquired-at",
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Acquired date"
                }
                input {
                    r#type: "date",
                    id: "acquired-at",
                    class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    value: acquired_at.read().to_string(),
                    oninput: move |event| {
                        acquired_at.set(event.value())
                    }
                }
            }
            div {
                class: "mb-4",
                label {
                    r#for: "manufactured-at",
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Manufactured date"
                }
                input {
                    r#type: "date",
                    id: "manufactured-at",
                    class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    value: manufactured_at.read().to_owned(),
                    oninput: move |event| {
                        manufactured_at.set(event.value())
                    }
                }
            }
            // Show the sold at date
            {
               if let Some(mut relinquished_at) = relinquished_at {
                    rsx! {
                        div {
                            class: "mb-4",
                            label {
                                r#for: "relinquished-at",
                                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                "Sold-at date (setting this will prevent the boat from appearing in practice/regatta search queries)"
                            }
                            input {
                                r#type: "date",
                                id: "relinquished-at",
                                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                value: relinquished_at.read().to_owned(),
                                oninput: move |event| {
                                    relinquished_at.set(event.value())
                                }
                            }
                        }
                   }
               } else {
                    rsx!()
               }
            }

            {

                match mode {
                    BoatFormMode::New => rsx!{
                        button {
                            class: "btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                            disabled: boat_type.read().is_none() || weight_class.read().is_none(),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(CreateBoatMsg::Create);
                            },
                            "Save New Boat"
                        }
                    },
                    BoatFormMode::Edit(id) => rsx!{
                        button {
                            class: "btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                            disabled: boat_type.read().is_none() || weight_class.read().is_none(),
                            onclick: move |e| {
                                e.stop_propagation();
                                boat_svc.send(CreateBoatMsg::Update(id));
                            },
                            "Save Changes to Boat"
                        }
                    },
                }
            }
        }
    }
}
