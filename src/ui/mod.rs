#![allow(non_snake_case)]
use dioxus::prelude::*;
#[cfg(feature = "ssr")]
pub mod state;
pub mod util;
mod components;
// use crate::{db::boat::types::BoatId, ui::{components::boat_list::BoatList, components::new_boat::NewBoatPage, util::loadable::Loadable}};
use dioxus_router::prelude::*;

use self::components::toast::{ToastCenter, ToastList};

pub fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, ToastList::default);
    // use_shared_state_provider(cx, ToastList2::default);

    cx.render(
        rsx!{
            ToastCenter {}
            Router::<components::Route>{}
        }
    )
}