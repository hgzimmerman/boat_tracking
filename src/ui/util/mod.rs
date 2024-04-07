pub mod loadable;
pub mod sleep;
pub mod time;


use dioxus::prelude::*;

/// Can't set img svg stroke/fill color using css, so we use a mask on a div instead.
/// 
/// This component does that for us.
/// It expects that you pass a color in the class argument like `bg-black` or `bg-white`.
/// 
/// 
/// Note - its a litte goofy, and the passed in class may not be registered by tailwind, 
/// so they must be added to the safelist manually in the tailwind.config.js
#[component]
pub fn MaskIcon(url: &'static str, class: Option<String>) -> Element {
    let class = class.unwrap_or_default() + " mask-icon";
    rsx!{
        div {
            class,
            "style": format!("mask: url({url}); webkit-mask: url({url});") 
        }       
    }
}