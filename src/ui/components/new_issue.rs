use dioxus::prelude::*;
// use dioxus_fullstack::prelude::*;

#[component]
pub fn NewIssuePage() -> Element {
    rsx! {
        div {
            class: "flex flex-column flex-grow",
            "We should have a form here for a new issue"
        }
    }
}
