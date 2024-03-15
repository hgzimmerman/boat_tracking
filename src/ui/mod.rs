#![allow(non_snake_case)]
use std::ops::Deref ;

use dioxus::prelude::*;
#[cfg(feature = "ssr")]
pub mod state;
pub mod util;
mod components;


#[component]
fn Boat(cx: Scope<()>, id: i32) -> Element<'a> {
    cx.render(rsx!(
        div {
           id.to_string() 
        }
    ))
}


#[derive(Props)]
struct BoatRowProps<'a> {
    boat: &'a BoatAndStats
}

fn BoatRow<'a>(cx: Scope<'a, BoatRowProps<'a>>) -> Element<'a> {
    let boat = cx.props.boat;
    cx.render(rsx! {
        div {
            "style": "display:flex; flex-direction: horizontal; flex-grow: 1; gap: 10px; border: solid black 1px; padding: 6px",
            onclick: move |event| {
                // now, outer won't be triggered
                event.stop_propagation();
                
            },
            div {
                "style": "display:flex; flex-direction: column; flex-grow: 1; gap: 10px;",
                div {
                    "style": "min-width: 160px; font-size: x-large; font-weight: 500",
                    boat.boat.name.clone(),
                }
                div {
                    format!("{:?} {:?}",boat.boat.weight_class, boat.boat.boat_type().unwrap())
                }
            }
            
            boat.boat.acquired_at.map(|x| rsx! {
                div {
                    "Acquired at : ",
                    x.to_string()
                }
            })
            div {
                label {
                    "Uses: "
                }
                format!("{}",boat.total_uses.unwrap_or_default() )
            }
            div {
                label {
                    "Monthly Uses: "
                }
                format!("{}",boat.uses_last_thirty_days.unwrap_or_default())
            }
            div {
                label {
                    "Open Issues: "
                }
                format!("{}",boat.open_issues.unwrap_or_default())
            }
        }
    })
}

#[component]
pub fn BoatList<'a>(cx: Scope, boats: LoadableRefResult<'a, Vec<BoatAndStats>>) -> Element {
    cx.render(match boats {
        Loadable::Loaded(Ok(boats)) => {
            rsx! {
                div {
                    boats.iter().map(|boat| rsx! {
                        BoatRow {
                            boat: boat
                        } 
                    })
                }
            }
            
        },
        Loadable::Loaded(Err(error)) => {
            rsx!{
                div {
                    "error: ",
                    error.to_string()
                }
            }
        }
        Loadable::Loading => rsx! {
            div {
                "Loading"
            }
        },
    })
}

#[component]
pub fn BoatList4<'a>(cx: Scope, boats: std::cell::Ref<'a, Result<Vec<BoatAndStats>, ServerFnError>>) -> Element {
    cx.render(match boats.deref() {
        Ok(boats) => {
            rsx! {
                div {
                    boats.iter().map(|boat| rsx! {
                        BoatRow {
                            boat: boat
                        } 
                    })
                }
            }
            
        },
        Err(error) => {
            rsx!{
                div {
                    "error: ",
                    error.to_string()
                }
            }
        }
    })
}

#[component]
pub fn BoatList5<'a>(cx: Scope, boats: Loadable<std::cell::Ref<'a, Result<Vec<BoatAndStats>, ServerFnError>>>) -> Element {
    use Loadable::*;
    cx.render(match boats.as_deref() {
        Loaded(Ok(boats)) => {
            rsx! {
                div {
                    boats.iter().map(|boat| rsx! {
                        BoatRow {
                            boat: boat
                        } 
                    })
                }
            }
            
        },
        Loaded(Err(error)) => {
            rsx!{
                div {
                    "error: ",
                    error.to_string()
                }
            }
        }
        Loading => {
            rsx! {
                "loading... boats"
            }
        }
    })
}



pub fn app(cx: Scope) -> Element {
    // let boat_state = use_state(cx,  LoadableResult::<()>::default);
    // let boat_svc = use_coroutine(cx, |rx| {
    //     to_owned![pool, boat_state];
    //     boat_list_service(rx, pool, boat_state)
    // });
    // boat_svc.send(BoatListMsg::Fetch);

    // let boats_fut = use_server_future(cx, (), |_| async {
    //     get_boats().await
    // })?;
    let boats_fut= use_server_future(cx, (), |_| async {
        get_boats().await
    });

    // let page = use_state(cx, || AppPage::BoatList);


    // let x = match page.get() {
    //     AppPage::BoatList => rsx!{
    //         BoatList {
    //             boats: boat_state.get().as_ref()
    //         }

    //     },
    //     AppPage::Boat{id} => rsx!{
    //         div {
    //             "id"
    //         }
    //     },
    //     // AppPage::CreateNewBoat => todo!(),
    //     // AppPage::IssueList => todo!(),
    //     // AppPage::Issue => todo!(),
    //     // AppPage::CreateNewIssue => todo!(),
    //     // AppPage::UseList => todo!(),
    // };
    // let boats = boats_fut.value();
    // let boats = Loadable::from_option(boats_fut2.as_ref().map(|x|x.value()));
    cx.render(
        rsx!{
            h1 {
                "top of page"
            }
            div {
                BoatList5 {
                    boats: Loadable::from_option(boats_fut.as_ref().map(|x|x.value()))
                }
            }
            
        }
    )
}
use dioxus_router::prelude::*;

use crate::{db::boat::BoatAndStats, ui::util::loadable::Loadable};

#[derive(Clone)]
pub enum AppPage {
    // #[route("/boat")]
    BoatList,
    // #[route("/boat/:id")]
    Boat{id: i32,},
    // #[route("/boat/create")]
    // CreateNewBoat,
    // #[route("/issue_list")]
    // IssueList,
    // #[route("/issue")]
    // Issue,
    // #[route("/issue/create")]
    // CreateNewIssue,
    // #[route("/use_events")]
    // UseList,
}

enum BoatListMsg {
    Fetch
}


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

use self::util::loadable::LoadableRefResult;

// #[server(GetBoats)]
#[server(GetBoats)]
async fn get_boats() -> Result<Vec<BoatAndStats>, ServerFnError> {

    let state: state::AppState = extract().await.expect("to get state aoeu");
    
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;
    conn 
        .interact(|conn| {
            BoatAndStats::get_boats(conn).map_err(ServerFnError::from)
        })
        .await
        .map_err(ServerFnError::from)?
        // .map_err(|e| anyhow::anyhow!("{}",e.to_string()))?
}


// #[server(GetData)]
#[server]
async fn get_data() -> Result<usize, ServerFnError> {
    Ok(5)
}