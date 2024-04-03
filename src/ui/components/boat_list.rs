use crate::db::boat::BoatAndStats;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[server(GetBoats)]
pub(crate) async fn get_boats() -> Result<Vec<BoatAndStats>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    conn.interact(|conn| BoatAndStats::get_boats(conn).map_err(ServerFnError::from))
        .await?
}
pub fn BoatListPage() -> Element {
    let boats_fut = use_server_future(|| async { get_boats().await })?;
    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)]",
            div {
                class: "flex-grow flex flex-col items-center bg-gray-50 divide-x-2 dark:divide-white",
                BoatList {
                    boats: boats_fut.value().cloned()?
                }
            }

        }
    }
}

pub fn BoatListNav() -> Element {
    use crate::ui::components::Route;
    use dioxus_router::prelude::*;
    rsx! {
        nav {
            class: "bg-ggrc sticky top-0 px-4",
            ul {
                class: "flex items-center",

                li {
                    class: "mr-3",
                    Link {
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::NewBoatPage{},
                        "New Boat"
                    }
                }
                li {
                    class: "mr-3",
                    a {
                        class: "inline-block p-4",
                        href: format!("/boats_export.csv"),
                        target: "_blank",
                        "Export to CSV"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn BoatRow(boat: BoatAndStats) -> Element {
    rsx! {
        div {
            class: "flex flex-row flex-grow gap-2.5 p-1.5",
            onclick: move |event| {
                // now, outer won't be triggered
                event.stop_propagation();

            },
            div {
                class: "flex flex-col flex-grow gap-2.5",
                div {
                    class: "text-lg font-medium min-w-40",
                    dioxus_router::components::Link {
                        to: crate::ui::components::Route::BoatSummary{id: boat.boat.id},
                        {boat.boat.name.clone()}
                    }
                }
                div {
                    {
                        format!("{} {}",boat.boat.weight_class, boat.boat.boat_type()?)
                    }
                }
            }

            {
                boat.boat.acquired_at.map(|x| rsx! {
                    div {
                        "Acquired at: ",
                        {x.to_string()}
                    }
                })
            }
            div {
                label {
                    "Uses: "
                }
                {format!("{}", boat.total_uses.unwrap_or_default() )}
            }
            div {
                label {
                    "Monthly Uses: "
                }
                {format!("{}", boat.uses_last_thirty_days.unwrap_or_default())}
            }
            div {
                label {
                    "Open Issues: "
                }
                {format!("{}", boat.open_issues.unwrap_or_default())}
            }
        }
    }
}

#[component]
pub fn BoatList(boats: ReadOnlySignal<Result<Vec<BoatAndStats>, ServerFnError>>) -> Element {
    match boats() {
        Ok(boats) => {
            rsx! {
                div {
                    class: "flex flex-row flex-grow xl:px-12 w-full bg-gray-50 dark:bg-gray-500 md:min-w-96 max-w-xxl shadow-md",
                    div {
                        class: "flex-grow divide-y-2 dark:divide-white dark:text-white dark:bg-gray-600 lg:px-4",
                        {
                            boats.into_iter().map(|boat| rsx! {
                                BoatRow {
                                    boat: boat.clone() // maybe avoid cloning this in the future?
                                }
                            })
                        }
                    }
                }
            }
        }
        Err(error) => {
            rsx! {
                div {
                    "error: ",
                    {error.to_string()}
                }
            }
        }
    }
}
