#![allow(non_snake_case)]
use dioxus::prelude::*;
#[cfg(feature = "ssr")]
pub mod state;
pub mod util;
mod components;
// use crate::{db::boat::types::BoatId, ui::{components::boat_list::BoatList, components::new_boat::NewBoatPage, util::loadable::Loadable}};
use dioxus_router::prelude::*;

pub fn app(cx: Scope) -> Element {
    // let boat_state = use_state(cx,  LoadableResult::<()>::default);
    // let boat_svc = use_coroutine(cx, |rx| {
    //     to_owned![pool, boat_state];
    //     boat_list_service(rx, pool, boat_state)
    // });
    // boat_svc.send(BoatListMsg::Fetch);

    cx.render(
        rsx!{
            Router::<components::Route>{}
        }
    )
}


// enum BoatListMsg {
//     Fetch
// }


// async fn boat_list_service(
//     mut rx: UnboundedReceiver<BoatListMsg>,
//     pool: Arc<Pool>,
//     boats: UseState<LoadableResult<Vec<BoatAndStats>>>
// ) {
//     while let Some(msg) = rx.next().await {
//         match msg {
//             BoatListMsg::Fetch => {
//                 boats.set(
//                 get_boats(pool.clone()).await.map_err(|x| x.to_string()).into()
//                 );
//             },
//         }
        
//     }
// }

use dioxus_fullstack::prelude::*;


