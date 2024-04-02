use dioxus::prelude::*;
// use dioxus_fullstack::prelude::*;

#[component]
pub fn IssueListPage() -> Element {
    rsx! {
        div {
            class: "flex flex-column flex-grow",
            "We should have a bunch of issues here"
        }

    }
}
