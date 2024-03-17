use crate::{db::boat::BoatAndStats, ui::util::loadable::{Loadable, LoadableRefResult}};

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[server(GetBoats)]
pub(crate) async fn get_boats() -> Result<Vec<BoatAndStats>, ServerFnError> {

    let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    
    let conn = state.pool().get().await.map_err(ServerFnError::from)?;
    conn 
        .interact(|conn| {
            BoatAndStats::get_boats(conn).map_err(ServerFnError::from)
        })
        .await
        .map_err(ServerFnError::from)?
}
pub fn BoatListPage(cx: Scope) -> Element {
    let boats_fut= use_server_future(cx, (), |_| async {
        get_boats().await
    });
    cx.render(rsx! {
        div {
            class: "overflow-y-auto flex-grow max-h-[calc(100vh-42px)]",
            BoatList {
                boats: Loadable::from_option(boats_fut.map(|x|x.value()))
            }
        }
    })
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
                    // boat.boat.name.clone(),
                    dioxus_router::components::Link { 
                        to: crate::ui::components::Route::BoatPage{id: boat.boat.id}, 
                        boat.boat.name.clone(),
                    }
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
pub fn BoatList<'a>(
    cx: Scope, 
    boats: LoadableRefResult<'a, Vec<BoatAndStats>>
) -> Element {
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

