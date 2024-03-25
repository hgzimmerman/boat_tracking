use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use crate::{db::boat::{types::{BoatType, WeightClass}, Boat, NewBoat}, ui::components::toast::{MsgType, ToastCenter, ToastData, ToastList, ToastMsgMsg}};


#[component]
pub fn NewBoatPage() -> Element {

    let mut name = use_signal(String::new);
    let mut acquired_at = use_signal(String::new);
    let mut manufactured_at = use_signal(String::new);
    
    let mut show_boat_type_dropdown= use_signal(|| false);
    let mut boat_type = use_signal(|| Option::<BoatType>::None); 
    let mut show_weight_class_dropdown= use_signal(|| false);
    let mut weight_class = use_signal(|| Option::<WeightClass>::None); 
    let toasts = use_signal(ToastList::default);

    let toast_svc = use_coroutine(|rx| {
        to_owned![toasts];
        crate::ui::components::toast::toast_service(rx, toasts)
    });

    let boat_svc = use_coroutine(|rx| {
        to_owned![name, boat_type, weight_class, acquired_at, manufactured_at, toast_svc];
        create_boat_service(rx, name, weight_class, boat_type, acquired_at, manufactured_at, toast_svc)
    });



    rsx!{
        ToastCenter {
            toasts: toasts,
            toast_svc: toast_svc
        }
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
                            {format!("{weight_class:?}")}
                            // the dropdown
                            div {
                                id: "weight-class-dropdown-positioner",
                                class: "relative h-0 w-0",
                                div {
                                    id: "weight-class-dropdown",
                                    class: if *show_weight_class_dropdown.read() {
                                        "absolute z-10 mt-2 w-20 top-0 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y p-2 text-slate-600 font-normal"
                                    } else {
                                        "hidden"
                                    },
                                    ul {
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                weight_class.set(None);
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "None"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                weight_class.set(Some(WeightClass::Light));
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "Light"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                weight_class.set(Some(WeightClass::Medium));
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "Medium"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                weight_class.set(Some(WeightClass::Heavy));
                                                show_weight_class_dropdown.set(false);
                                            },
                                            "Heavy"
                                        }
                                        li {
                                            onclick: move |e| {
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
                            {format!("{boat_type:?}")}
                            // the dropdown
                            div {
                                id: "boat-type-dropdown-positioner",
                                class: "relative h-0 w-0",
                                div {
                                    id: "boat-type-dropdown",
                                    class: if *show_boat_type_dropdown.read() {
                                        "absolute z-10 mt-2 w-20 top-0 left-4 origin-bottom-right rounded-md bg-white shadow-lg divide-y p-2 text-slate-600 font-normal"
                                    } else {
                                        "hidden"
                                    },
                                    ul {
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                boat_type.set(None);
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "None"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Single));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Single"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Double));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Double"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Quad));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Quad"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::QuadPlus));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Quad+"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::Four));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Four"
                                        }
                                        li {
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                boat_type.set(Some(BoatType::FourPlus));
                                                show_boat_type_dropdown.set(false);
                                            },
                                            "Four+"
                                        }
                                        li {
                                            onclick: move |e| {
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
                    button {
                        class: "btn btn-blue rounded-e disabled:opacity-45 disabled:bg-blue-500",
                        disabled: boat_type.read().is_none() || weight_class.read().is_none(),
                        onclick: move |e| {
                            e.stop_propagation();
                            boat_svc.send(CreateBoatMsg::Submit);
                        },
                        "Save New Boat"
                    }
                }
            }
        }

    }
}


#[server(GetBoats)]
pub(crate) async fn create_boat(
    name: String,
    weight: WeightClass,
    ty: BoatType,
    acquired_at: Option<NaiveDate>,
    manufactured_at: Option<NaiveDate>,
) -> Result<Boat, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    let boat = NewBoat::new(name, weight, ty, acquired_at, manufactured_at);

    conn 
        .interact(|conn| {
            Boat::new_boat(conn, boat).map_err(ServerFnError::from)
        })
        .await?
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
        name: Signal<String>,
        weight: Signal<Option<WeightClass>>,
        ty: Signal<Option<BoatType>>,
        acquired_at: Signal<String>,
        manufactured_at: Signal<String>
    ) -> Result<Self, BoatArgsError> {
        let name: String = if name.read().is_empty() {
            return Err(BoatArgsError::MissingName)
        } else {
            name.read().clone()
        };
        let weight: WeightClass= if let Some(weight) = *weight.read() {
            weight
        } else {
            return Err(BoatArgsError::MissingWeight)
        };
        let ty: BoatType = if let Some(ty) = *ty.read() {
            ty
        } else {
            return Err(BoatArgsError::MissingBoatType)
        };
        let acquired_at = if acquired_at.read().is_empty() {
            None
        } else {
            tracing::info!(acquired = ?acquired_at.read());
            chrono::NaiveDate::parse_from_str(&&acquired_at.read(), "%Y-%m-%d").map_err(BoatArgsError::InvalidAcquiredAt).map(Some)? 
        };
        let manufactured_at = if manufactured_at.read().is_empty(){
            None
        } else { 
            tracing::info!(manufactured = ?manufactured_at.read());
            chrono::NaiveDate::parse_from_str(&&manufactured_at.read(), "%Y-%m-%d").map_err(BoatArgsError::InvalidManufactureddAt).map(Some)?
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
    mut name: Signal<String>,
    mut weight: Signal<Option<WeightClass>>,
    mut ty: Signal<Option<BoatType>>,
    mut acquired_at: Signal<String>,
    mut manufactured_at: Signal<String>,
    toasts: Coroutine<ToastMsgMsg>
) {
    use futures::stream::StreamExt;


    while let Some(msg) = rx.next().await {
        match msg {
            CreateBoatMsg::Submit => {
                match BoatArgs::new(name, weight, ty, acquired_at, manufactured_at) {
                    Ok(args) => {
                        match create_boat(args.name, args.weight, args.ty, args.acquired_at, args.manufactured_at).await {
                            Ok(_boat) => {
                                name.set(String::new());
                                weight.set(None);
                                ty.set(None);
                                acquired_at.set(String::new());
                                manufactured_at.set(String::new());
                                
                                toasts.send(ToastMsgMsg::Add(
                                    ToastData {msg: "Created new boat".to_string(), ty: MsgType::Normal}, 
                                    std::time::Duration::from_secs(5)
                                ))
                            },
                            Err(error) => {
                                tracing::warn!(?error, "Could not send request");

                                toasts.send(ToastMsgMsg::Add(
                                    ToastData {msg: "Could not send requests".to_string(), ty: MsgType::Error}, 
                                    std::time::Duration::from_secs(2)
                                ))
                            },
                        }
                    }
                    Err(error) => {
                        tracing::warn!(?error, "failed validation");
                        toasts.send(ToastMsgMsg::Add(
                            ToastData {msg: error.to_string(), ty: MsgType::Warn}, 
                            std::time::Duration::from_secs(2)
                        ));
                    },
                }
            },
        }
    }
}