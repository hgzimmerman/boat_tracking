use crate::{
    db::boat::types::{BoatId, BoatType, WeightClass}, ui::components::{boat::creation_edit_form::{BoatForm, BoatFormMode}, toast::{ToastData, ToastMsgMsg}}
};
use dioxus::prelude::*;
use super::service::{create_boat_service, get_boat};

#[component]
pub fn EditBoatForm(id: BoatId) -> Element {
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
                    acquired_at.set(boat.acquired_at.as_ref().map(ToString::to_string).unwrap_or_default());                    
                    manufactured_at.set(boat.manufactured_at.as_ref().map(ToString::to_string).unwrap_or_default());                    
                    relinquished_at.set(boat.relinquished_at.as_ref().map(ToString::to_string).unwrap_or_default());                    
                    boat_type.set(boat.boat_type());
                    weight_class.set(Some(boat.weight_class));
                },
                Err(error) => toast_svc.send(ToastData::error(error).into()),
            }
    }});
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