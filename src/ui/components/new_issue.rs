use dioxus::prelude::*;
// use dioxus_fullstack::prelude::*;

#[component]
pub fn NewIssuePage() -> Element {
    rsx!{
        div {
            "style": "display:flex; flex-direction: vertical; flex-grow: 1;",
            "We should have a form here for a new issue" 
        }
    }
}