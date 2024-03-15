#![allow(non_snake_case)]
use dioxus::prelude::*;
#[cfg(feature = "ssr")]
pub mod state;
pub mod util;
mod components;
use crate::{db::boat::types::BoatId, ui::{components::boat_list::BoatList, util::loadable::Loadable}};

pub fn app(cx: Scope) -> Element {
    // let boat_state = use_state(cx,  LoadableResult::<()>::default);
    // let boat_svc = use_coroutine(cx, |rx| {
    //     to_owned![pool, boat_state];
    //     boat_list_service(rx, pool, boat_state)
    // });
    // boat_svc.send(BoatListMsg::Fetch);

    let boats_fut= use_server_future(cx, (), |_| async {
        components::boat_list::get_boats().await
    });

    // let page = use_state(cx, || AppPage::BoatList);


    // let boats = boats_fut.value();
    // let boats = Loadable::from_option(boats_fut2.as_ref().map(|x|x.value()));
    cx.render(
        rsx!{
            Router::<Route>{}
        }
    )
}
use dioxus_router::prelude::*;





#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home, 
    #[route("/dummy")]
    Dummy,
    #[nest("/boats")]
        #[route("/")]
        BoatListPage,
        #[route("/:id")]
        BoatPage{id: BoatId}
    // #[end_nest]

}
use crate::ui::components::boat::BoatPage;
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Home Page"
        }
    })
}

fn Dummy(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "dummy"
        }
    })
}

fn BoatListPage(cx: Scope) -> Element {
    let boats_fut= use_server_future(cx, (), |_| async {
        components::boat_list::get_boats().await
    });
    cx.render(rsx! {
        div {
            BoatList {
                boats: Loadable::from_option(boats_fut.map(|x|x.value()))
            }
        }
    })
}

#[component]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::Dummy {}, "Dummy" } }
                li { Link { to: Route::BoatListPage {}, "Boats" } }
            }
        }
        Outlet::<Route> {}
    }
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


