use dioxus::prelude::*;
// use dioxus_fullstack::prelude::*;

#[component]
pub fn IssueListPage() -> Element {
    rsx! {
        div {
            "style": "display:flex; flex-direction: vertical; flex-grow: 1;",
            "We should have a bunch of issues here"
        }

    }
}
