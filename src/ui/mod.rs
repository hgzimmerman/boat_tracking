#![allow(non_snake_case)]
use dioxus::prelude::*;
#[cfg(feature = "ssr")]
pub mod state;
pub mod util;
mod components;
use dioxus_router::prelude::*;


pub fn app(cx: Scope) -> Element {
    // fermi::use_init_atom_root(cx);
    cx.render(
        rsx!{
            Router::<components::Route>{}
        }
    )
}