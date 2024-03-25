#![allow(non_snake_case)]
use dioxus::prelude::*;
mod components;
#[cfg(feature = "ssr")]
pub mod state;
pub mod util;
use dioxus_router::prelude::*;

pub fn app() -> Element {
    // fermi::use_init_atom_root(cx);
    let toasts = use_signal(components::toast::ToastList::default);
    let toast_svc = use_coroutine(|rx| {
        to_owned![toasts];
        crate::ui::components::toast::toast_service(rx, toasts)
    });
    rsx! {
        components::toast::ToastCenter {
            toasts: toasts,
            toast_svc: toast_svc
        }
        Router::<components::Route>{}
    }
}


/// Use this on the ssr side to basically turn of ssr. (observing crashes in hydration routine when this was added)
pub fn empty_app() -> Element {
    None
}