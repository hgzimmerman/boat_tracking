use std::sync::Arc;

use dioxus::prelude::*;

pub enum ToastMsgMsg {
    Add(ToastData, std::time::Duration),
    Remove(usize)
}


/// A hacked-together sleep function that works in both server and web contexts.
async fn sleep(duration: std::time::Duration) {
    #[cfg(feature = "ssr")]
    {
        tokio::time::sleep(duration).await
    }
    #[cfg(feature = "web")]
    {
        use wasm_bindgen::JsCast;
        let (send, recv) = futures::channel::oneshot::channel();
        let closure = wasm_bindgen::closure::Closure::once(|| {
            let _ = send.send(());
        });
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0( 
            closure.as_ref().unchecked_ref(), 
            duration.as_millis() as i32
        );

        closure.forget();
        recv.await;
    }
}

pub async fn toast_service(
    mut rx: UnboundedReceiver<ToastMsgMsg>,
    toasts: UseState<ToastList>
) {
    use futures::stream::StreamExt;
    while let Some(msg) = rx.next().await {
        match msg {
            ToastMsgMsg::Add(toast, duration) => {
                let counter = toasts.add(toast);
                toasts.needs_update();
                let toasts = toasts.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    sleep(duration).await;
                    toasts.toasts.lock().unwrap().shift_remove(&counter);
                    tracing::debug!("removed toast");
                    toasts.needs_update();
                });
            },
            ToastMsgMsg::Remove(id) => {
                tracing::debug!(?id, "removed toast externally");
                toasts.toasts.lock().unwrap().shift_remove(&id);
                toasts.needs_update();
            },
        }
    }
}



#[component]
pub fn ToastCenter<'a>(
    cx: Scope, 
    toasts: &'a UseState<ToastList>,
    toast_svc: &'a Coroutine<ToastMsgMsg>
) -> Element<'a> {
    let t = {
        toasts.toasts.as_ref().lock().unwrap().clone() 
    };


    cx.render(rsx!(
        div {
            class: "absolute top-8 right-8 z-20",
            t
                .into_iter()
                .map(|(counter, toast)| rsx!{
                    Toast {
                        msg: toast.msg,
                        ty: toast.ty,
                        toast_svc: toast_svc,
                        counter: counter
                    }
                })
        }
    ))
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToastData {
    pub msg: String,
    pub ty: MsgType,
}


#[derive(Default, Clone)]
pub struct ToastList {
    counter: Arc<std::sync::Mutex<usize>>,
    toasts: Arc<std::sync::Mutex<indexmap::IndexMap<usize, ToastData>>>
}

impl PartialEq for ToastList {
    fn eq(&self, other: &Self) -> bool {
        let self_counter = {
            *self.counter.lock().unwrap()
        };
        let other_counter = {
            *other.counter.lock().unwrap()
        };
        let self_toasts = {
            self.toasts.lock().unwrap().clone()
        };
        let other_toasts = {
            other.toasts.lock().unwrap().clone()
        };
        self_counter == other_counter && self_toasts == other_toasts
    }
}

impl ToastList {

    pub fn add(&self, toast: ToastData) -> usize {
        let counter = {
            let mut counter = self.counter.lock().unwrap();
            let c = *counter;
            *counter += 1;
            c
        };
        
        {
            self.toasts.lock().unwrap().insert(counter, toast);
        }
        counter
    }
}

/// https://preline.co/docs/toasts.html
#[component]
pub fn Toast<'a>(
    cx: Scope, 
    msg: String,
    ty: MsgType,
    toast_svc: &'a Coroutine<ToastMsgMsg>,
    counter: usize
) -> Element<'a> {
    let svg = match ty {
        MsgType::Info => rsx!{
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
        MsgType::Normal => rsx!{ 
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
            class: "max-w-xs bg-white border border-gray-200 rounded-xl shadow-xl dark:bg-gray-800 dark:border-gray-700",
            onclick: |_| toast_svc.send(ToastMsgMsg::Remove(*counter)),
            div { 
                class: "flex p-4",
                div { 
                    class: "flex-shrink-0",
                    svg 
                }
                div { class: "ms-3",
                    p { 
                        class: "text-sm text-gray-700 dark:text-gray-400",
                        msg.as_str()
                    }
                }
            }
        }
    })
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MsgType {
    #[default]
    Normal,
    Info,
    Warn,
    Error
}