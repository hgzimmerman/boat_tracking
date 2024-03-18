use std::collections::VecDeque;

use dioxus::prelude::*;


pub fn TastCenter(
    cx: Scope, 
) -> Element {
    let toasts = use_state(cx, VecDeque::new);

    cx.spawn({
        let _toasts = toasts.to_owned();
        async move {
            // remove toasts after expire? Probably want a coroutine for this.
        }
    });

    cx.render(rsx!(
        div {
            class: "",
            toasts.get()
                .iter()
                .map(|toast: &ToastData| rsx!{
                    Toast {
                        msg: &toast.msg,
                        ty: toast.ty
                    }
                })
        }
    ))
}

pub struct ToastData {
    msg: String,
    ty: MsgType,
    duration: std::time::Duration
}

/// https://preline.co/docs/toasts.html
#[component]
pub fn Toast<'a>(
    cx: Scope, 
    msg: &'a str,
    ty: MsgType
) -> Element<'a> {
    let svg = match ty {
        MsgType::Normal => rsx!{
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "16",
                height: "16",
                view_box: "0 0 16 16",
                fill: "currentColor",
                class: "flex-shrink-0 size-4 text-blue-500 mt-0.5",
                path { d: "M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16zm.93-9.412-1 4.705c-.07.34.029.533.304.533.194 0 .487-.07.686-.246l-.088.416c-.287.346-.92.598-1.465.598-.703 0-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381 2.29-.287zM8 5.5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z" }
            }
        },
        MsgType::Info => rsx!{
            
            svg {
                height: "16",
                width: "16",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "currentColor",
                view_box: "0 0 16 16",
                class: "flex-shrink-0 size-4 text-teal-500 mt-0.5",
                path { d: "M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z" }
            }
        },
        MsgType::Warn => rsx!{
            svg {
                fill: "currentColor",
                height: "16",
                xmlns: "http://www.w3.org/2000/svg",
                width: "16",
                view_box: "0 0 16 16",
                class: "flex-shrink-0 size-4 text-yellow-500 mt-0.5",
                path { d: "M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zM8 4a.905.905 0 0 0-.9.995l.35 3.507a.552.552 0 0 0 1.1 0l.35-3.507A.905.905 0 0 0 8 4zm.002 6a1 1 0 1 0 0 2 1 1 0 0 0 0-2z" }
            } 
        },
        MsgType::Error => rsx!{
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 16 16",
                height: "16",
                fill: "currentColor",
                width: "16",
                class: "flex-shrink-0 size-4 text-red-500 mt-0.5",
                path { d: "M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zM5.354 4.646a.5.5 0 1 0-.708.708L7.293 8l-2.647 2.646a.5.5 0 0 0 .708.708L8 8.707l2.646 2.647a.5.5 0 0 0 .708-.708L8.707 8l2.647-2.646a.5.5 0 0 0-.708-.708L8 7.293 5.354 4.646z" }
            }
        },
    };

    cx.render(rsx!{
        div {
            role: "alert",
            class: "max-w-xs bg-white border border-gray-200 rounded-xl shadow-lg dark:bg-gray-800 dark:border-gray-700",
            div { 
                class: "flex p-4",
                div { 
                    class: "flex-shrink-0",
                    svg 
                }
                div { class: "ms-3",
                    p { 
                        class: "text-sm text-gray-700 dark:text-gray-400",
                        msg
                    }
                }
            }
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MsgType {
    Normal,
    Info,
    Warn,
    Error
}