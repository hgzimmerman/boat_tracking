#![allow(non_snake_case)]
use dioxus::prelude::*;
#[cfg(feature = "ssr")]
pub mod state;
pub mod util;
mod components;
// use crate::{db::boat::types::BoatId, ui::{components::boat_list::BoatList, components::new_boat::NewBoatPage, util::loadable::Loadable}};
use dioxus_router::prelude::*;

pub fn app(cx: Scope) -> Element {

    cx.render(
        rsx!{
            Router::<components::Route>{}
        }
    )
}