use dioxus::prelude::*;


#[component]
pub fn Modal(hidden: bool, children: Element ) -> Element{
    rsx!{
        div {
            class: "",
            class: if hidden { "hidden" },
            {children}
        } 
    }
}