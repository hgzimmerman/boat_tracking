use super::service::create_boat_service;
use super::BoatForm;
use crate::{
    db::boat::types::{BoatType, WeightClass},
    ui::components::{boat::creation_edit_form::BoatFormMode, toast::ToastMsgMsg},
};
use dioxus::prelude::*;

#[component]
pub fn NewBoatPage() -> Element {
    let name = use_signal(String::new);
    let acquired_at = use_signal(String::new);
    let manufactured_at = use_signal(String::new);

    let boat_type = use_signal(|| Option::<BoatType>::None);
    let weight_class = use_signal(|| Option::<WeightClass>::None);

    let toast_svc = use_coroutine_handle::<ToastMsgMsg>();
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
            None,
            toast_svc,
        )
    });

    rsx! {
        div {
            class: "overflow-y-hidden justify-center flex flex-col grow",
            div {
                class: "flex flex-col overflow-y-auto",
                div {
                    class: "flex flex-row flex-grow justify-center",
                    BoatForm {
                        name: name,
                        acquired_at: acquired_at,
                        manufactured_at: manufactured_at,
                        boat_type: boat_type,
                        weight_class: weight_class,
                        mode: BoatFormMode::New
                    }
                }
            }
        }

    }
}
