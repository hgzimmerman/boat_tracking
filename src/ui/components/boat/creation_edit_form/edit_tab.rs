use super::service::create_boat_service;
use crate::{
    db::boat::{
        types::{BoatId, BoatType, WeightClass},
        Boat,
    },
    ui::components::{
        boat::creation_edit_form::{BoatForm, BoatFormMode},
        toast::{ToastData, ToastMsgMsg},
    },
};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[component]
pub fn BoatEdit(id: BoatId) -> Element {
    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            EditBoatForm {
                id
            }
        }
    }
}

#[component]
fn EditBoatForm(id: BoatId) -> Element {
    let mut name = use_signal(String::new);
    let mut acquired_at = use_signal(String::new);
    let mut manufactured_at = use_signal(String::new);
    let mut relinquished_at = use_signal(String::new);

    let mut boat_type = use_signal(|| Option::<BoatType>::None);
    let mut weight_class = use_signal(|| Option::<WeightClass>::None);

    let toast_svc = use_coroutine_handle::<ToastMsgMsg>();
    use_future(move || {
        to_owned![id, toast_svc];
        async move {
            match get_boat(id).await {
                Ok(boat) => {
                    name.set(boat.name.clone());
                    acquired_at.set(
                        boat.acquired_at
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_default(),
                    );
                    manufactured_at.set(
                        boat.manufactured_at
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_default(),
                    );
                    relinquished_at.set(
                        boat.relinquished_at
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_default(),
                    );
                    boat_type.set(boat.boat_type());
                    weight_class.set(Some(boat.weight_class));
                }
                Err(error) => toast_svc.send(ToastData::error(error).into()),
            }
        }
    });
    let _boat_svc = use_coroutine(|rx| {
        to_owned![
            name,
            boat_type,
            weight_class,
            acquired_at,
            manufactured_at,
            toast_svc
        ];
        create_boat_service(
            rx,
            name,
            weight_class,
            boat_type,
            acquired_at,
            manufactured_at,
            Some(relinquished_at),
            toast_svc,
        )
    });
    rsx! {
        div {
            class: "flex flex-col flex-grow justify-center",
            div {
                class: "flex flex-row flex-grow justify-center",
                BoatForm {
                    name: name,
                    acquired_at: acquired_at,
                    manufactured_at: manufactured_at,
                    relinquished_at: relinquished_at,
                    boat_type: boat_type,
                    weight_class: weight_class,
                    mode: BoatFormMode::Edit(id)
                }
            }
        }

    }
}

#[server(GetBoat)]
/// Gets the boat at the id. Needed when populating the form for editing
async fn get_boat(boat_id: BoatId) -> Result<Boat, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;

    conn.interact(move |conn| Boat::get_boat(conn, boat_id).map_err(ServerFnError::from))
        .await?
}
