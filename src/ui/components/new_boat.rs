use std::ops::Deref;

use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use crate::{db::boat::{types::{BoatType, WeightClass}, Boat, NewBoat}, ui::components::toast::{MsgType, ToastData, ToastList}};





#[component]
pub fn NewBoatPage(cx: Scope) -> Element {

    let toasts = use_shared_state::<ToastList>(cx)?;

    let name = use_state(cx, String::new);
    let acquired_at = use_state(cx, String::new);
    let manufactured_at = use_state(cx, String::new);
    
    let show_boat_type_dropdown= use_state(cx, || false);
    let boat_type = use_state(cx, || Option::<BoatType>::None); 
    let show_weight_class_dropdown= use_state(cx, || false);
    let weight_class = use_state(cx, || Option::<WeightClass>::None); 


    let boat_svc = use_coroutine(cx, |rx| {
        to_owned![name, boat_type, weight_class, acquired_at, manufactured_at, toasts];
        create_boat_service(rx, name, weight_class, boat_type, acquired_at, manufactured_at, toasts)
    });


    cx.render(rsx!{
        div {
            class: "flex flex-col flex-grow bg-gray-50 dark:bg-gray-500 justify-center",
            div {
                class: "flex flex-row flex-grow justify-center",
                form {
                    class: "bg-gray-100 shadow-md rounded px-8 pt-6 pb-8 mb-4 dark:bg-gray-600 min-w-96 max-w-lg w-1/2",
                    onsubmit: move |event| {
                        event.stop_propagation();
                    },
                    h2 {
                        class: "mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 md:text-2xl lg:text-3xl dark:text-white",
                        "Add a new boat"
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
                            value: name.deref().deref(),
                            oninput: |event| {
                                event.stop_propagation();
                                name.set(event.value.clone())
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
                                show_weight_class_dropdown.set(!show_weight_class_dropdown.get());
                            },
                            onmouseover: move |e| {
                                e.stop_propagation();
                                show_weight_class_dropdown.set(true);
                            },
                            onmouseout: move |e| {
                                e.stop_propagation();
                                show_weight_class_dropdown.set(false);
                            },
                            format!("{weight_class:?}")
                            // the dropdown
                            div {
                                id: "weight-class-dropdown-positioner",
                                class: "relative h-0 w-0",
                                div {
                                    id: "weight-class-dropdown",
                                    class: if *show_weight_class_dropdown.get() {
                                        "absolute z-10 mt-2 w-20 top-0 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y p-2 text-slate-600 font-normal"
                                    } else {
                                        "hidden"
                                    },
                                    ul {
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                weight_class.set(None);
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "None"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                weight_class.set(Some(WeightClass::Light));
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "Light"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                weight_class.set(Some(WeightClass::Medium));
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "Medium"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                weight_class.set(Some(WeightClass::Heavy));
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "Heavy"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                weight_class.set(Some(WeightClass::Tubby));
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "Tubby"
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
                                show_boat_type_dropdown.set(!show_boat_type_dropdown.get());
                            },
                            onmouseover: move |e| {
                                e.stop_propagation();
                                show_boat_type_dropdown.set(true);
                            },
                            onmouseout: move |e| {
                                e.stop_propagation();
                                show_boat_type_dropdown.set(false);
                            },
                            format!("{boat_type:?}")
                            // the dropdown
                            div {
                                id: "boat-type-dropdown-positioner",
                                class: "relative h-0 w-0",
                                div {
                                    id: "boat-type-dropdown",
                                    class: if *show_boat_type_dropdown.get() {
                                        "absolute z-10 mt-2 w-20 top-0 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y p-2 text-slate-600 font-normal"
                                    } else {
                                        "hidden"
                                    },
                                    ul {
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                boat_type.set(None);
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "None"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Single));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Single"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Double));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Double"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Quad));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Quad"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::QuadPlus));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Quad+"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Four));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Four"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::FourPlus));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Four+"
                                        }
                                        li {
                                            onclick: |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Eight));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Eight"
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
                            value: acquired_at.deref().deref(),
                            oninput: |event| {
                                acquired_at.set(event.value.clone())
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
                            value: manufactured_at.deref().deref(),
                            oninput: |event| {
                                manufactured_at.set(event.value.clone())
                            }
                        }
                    }
                    button {
                        class: "btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                        disabled: boat_type.is_none() || weight_class.is_none(),
                        onclick: move |e| {
                            e.stop_propagation();
                            boat_svc.send(CreateBoatMsg::Submit);
                        },
                        "Save New Boat"
                    }
                }
            }
        }

    })
}


#[server(GetBoats)]
pub(crate) async fn create_boat(
    name: String,
    weight: WeightClass,
    ty: BoatType,
    acquired_at: Option<NaiveDate>,
    manufactured_at: Option<NaiveDate>,
) -> Result<Boat, ServerFnError> {
    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;

    let boat = NewBoat::new(name, weight, ty, acquired_at, manufactured_at);

    conn 
        .interact(|conn| {
            Boat::new_boat(conn, boat).map_err(ServerFnError::from)
        })
        .await
        .map_err(ServerFnError::from)?
}

enum CreateBoatMsg {
    Submit
}


struct BoatArgs {
    name: String,
    weight: WeightClass,
    ty: BoatType,
    acquired_at: Option<NaiveDate>,
    manufactured_at: Option<NaiveDate>,
}
#[derive(Debug,Clone, thiserror::Error)]
enum BoatArgsError {
    #[error("Missing Name")]
    MissingName,
    #[error("Missing Weight")]
    MissingWeight,
    #[error("Missing Boat Type")]
    MissingBoatType,
    #[error("Could not parse Acquired date.")]
    InvalidAcquiredAt(chrono::ParseError),
    #[error("Could not parse Manufacturing date.")]
    InvalidManufactureddAt(chrono::ParseError)
}
impl BoatArgs {
    fn new(    
        name: &UseState<String>,
        weight: &UseState<Option<WeightClass>>,
        ty: &UseState<Option<BoatType>>,
        acquired_at: &UseState<String>,
        manufactured_at: &UseState<String>
    ) -> Result<Self, BoatArgsError> {
        let name: String = if name.current().as_ref().is_empty() {
            return Err(BoatArgsError::MissingName)
        } else {
            name.current().as_ref().clone()
        };
        let weight: WeightClass= if let Some(weight) = *weight.current() {
            weight
        } else {
            return Err(BoatArgsError::MissingWeight)
        };
        let ty: BoatType = if let Some(ty) = *ty.current() {
            ty
        } else {
            return Err(BoatArgsError::MissingBoatType)
        };
        let acquired_at = if acquired_at.current().is_empty() {
            None
        } else {
            tracing::info!(acquired = ?acquired_at.current().as_ref());
            chrono::NaiveDate::parse_from_str(&&acquired_at.current(), "%Y-%m-%d").map_err(BoatArgsError::InvalidAcquiredAt).map(Some)? 
        };
        let manufactured_at = if manufactured_at.current().is_empty(){
            None
        } else { 
            tracing::info!(manufactured = ?manufactured_at.current().as_ref());
            chrono::NaiveDate::parse_from_str(&&manufactured_at.current(), "%Y-%m-%d").map_err(BoatArgsError::InvalidManufactureddAt).map(Some)?
        };
        Ok(BoatArgs {
            name,
            weight,
            ty,
            acquired_at,
            manufactured_at
        })
    }
}


async fn create_boat_service(
    mut rx: UnboundedReceiver<CreateBoatMsg>,
    name: UseState<String>,
    weight: UseState<Option<WeightClass>>,
    ty: UseState<Option<BoatType>>,
    acquired_at: UseState<String>,
    manufactured_at: UseState<String>,
    toasts: UseSharedState<ToastList>
) {
    use futures::stream::StreamExt;


    while let Some(msg) = rx.next().await {
        match msg {
            CreateBoatMsg::Submit => {
                match BoatArgs::new(&name, &weight, &ty, &acquired_at, &manufactured_at) {
                    Ok(args) => {
                        match create_boat(args.name, args.weight, args.ty, args.acquired_at, args.manufactured_at).await {
                            Ok(_boat) => {
                                name.set(String::new());
                                weight.set(None);
                                ty.set(None);
                                acquired_at.set(String::new());
                                manufactured_at.set(String::new());
                                toasts.read().add(ToastData {msg: "Created new boat".to_string(), ty: MsgType::Normal}, std::time::Duration::from_secs(2));
                            },
                            Err(error) => {
                                tracing::warn!(?error, "Could not send request");
                                toasts.read().add(ToastData {msg: "Could not send requests".to_string(), ty: MsgType::Error}, std::time::Duration::from_secs(2));
                            },
                        }
                    }
                    Err(error) => {
                        tracing::warn!(?error, "failed validation");
                        toasts.read().add(ToastData {msg: error.to_string(), ty: MsgType::Warn}, std::time::Duration::from_secs(2));
                    },
                }
            },
        }
    }
}