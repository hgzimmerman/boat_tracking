use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::{db::boat::{types::BoatId, BoatAndStats}, ui::util::MaskIcon};
pub mod creation_edit_form;
pub use creation_edit_form::BoatEdit;
mod summary_tab;
pub use summary_tab::*;

mod use_count_chart_tabs;
pub use use_count_chart_tabs::*;

mod issues_tab;
pub use issues_tab::*;

#[server(GetBoat)]
pub(crate) async fn get_boat(id: BoatId) -> Result<BoatAndStats, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;

    conn.interact(move |conn| BoatAndStats::get_boat(conn, id).map_err(ServerFnError::from))
        .await?
}

#[component]
pub fn BoatNav() -> Element {
    use crate::ui::components::Route;
    use dioxus_router::prelude::*;
    let path: Route = use_route();
    let id = match path {
        Route::BoatSummary { id }
        | Route::BoatMonthlyUses { id }
        | Route::BoatYearlyUses { id }
        | Route::BoatEdit { id }
        | Route::BoatIssues { id } => Some(id),
        _ => None,
    }
    .expect("should be in path where id is known");

    let boat_fut = use_resource(use_reactive!(|id| async move { get_boat(id).await }));

    let inactive_class = "inline-flex items-center p-4 rounded-t-lg hover:text-gray-600 hover:bg-gray-50 dark:hover:bg-gray-800 dark:hover:text-gray-300";
    let active_class = "inline-flex items-center p-4 rounded-t-lg hover:text-gray-600 hover:bg-gray-50 dark:hover:bg-gray-800 dark:hover:text-gray-300 active bg-gray-100 rounded-t-lg dark:bg-gray-800 dark:text-blue-500";

    rsx! {
        BoatTitle {
            boat: boat_fut.value()()?,
        }
        nav {
            id: "boat-nav",
            class: "mx-4",
            ul {
                class: "flex flex-wrap text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:border-gray-700 dark:text-gray-400",
                li { class: "me-2",
                    Link {
                        class: if matches!(path, Route::BoatSummary{..}) { active_class } else {inactive_class},
                        to: Route::BoatSummary{id},
                        MaskIcon {
                            class: "fill-current w-4 h-4 mr-1 bg-gray-500 dark:bg-gray-400 hover:bg-gray-600 dark:hover:bg-gray-300",
                            url: "/pie_chart.svg"
                        }
                        span {
                            "Summary"
                        }
                    }
                }
                li { class: "me-2",
                    Link {
                        // "aria-current": "page",
                        class: if matches!(path, Route::BoatMonthlyUses{..}) { active_class } else {inactive_class},
                        to: Route::BoatMonthlyUses{id},
                        MaskIcon {
                            class: "fill-current w-4 h-4 mr-1 bg-gray-500 dark:bg-gray-400 hover:bg-gray-600 dark:hover:bg-gray-300",
                            url: "/chart.svg"
                        }
                        span {
                            "Monthly Usage Chart"
                        }
                    }
                }
                li { class: "me-2",
                    Link {
                        class: if matches!(path, Route::BoatYearlyUses{..}) { active_class } else {inactive_class},
                        to: Route::BoatYearlyUses{id},
                        MaskIcon {
                            class: "fill-current w-4 h-4 mr-1 bg-gray-500 dark:bg-gray-400 hover:bg-gray-600 dark:hover:bg-gray-300",
                            url: "/chart.svg"
                        }
                        span {
                            "Yearly Usage Chart"
                        }
                    }
                }
                li { class: "me-2",
                    Link {
                        class: if matches!(path, Route::BoatIssues{..}) { active_class } else {inactive_class},
                        to: Route::BoatIssues{id},
                        MaskIcon {
                            class: "fill-current w-4 h-4 mr-1 bg-gray-500 dark:bg-gray-400 hover:bg-gray-600 dark:hover:bg-gray-300",
                            url: "/warn.svg"
                        }
                        span {
                            "Issues"
                        }
                    }
                }
                li { class: "me-2",
                    Link {
                        class: if matches!(path, Route::BoatEdit{..}) { active_class } else {inactive_class},
                        to: Route::BoatEdit{id},
                        MaskIcon {
                            class: "fill-current w-4 h-4 mr-1 bg-gray-500 dark:bg-gray-400 hover:bg-gray-600 dark:hover:bg-gray-300",
                            url: "/pencil.svg"
                        }
                        span {
                            "Edit"
                        }
                    }
                }
                li {
                    class: "me-2",
                    a {
                        class: "inline-flex items-center p-4",
                        href: format!("/uses_export.csv?id={id}"),
                        target: "_blank",
                        MaskIcon {
                            class: "fill-current w-4 h-4 mr-1 bg-gray-500 dark:bg-gray-400 hover:bg-gray-600 dark:hover:bg-gray-300",
                            url: "/download.svg"
                        }
                        span {
                            "Export to CSV"
                        }
                    }
                }
            }
        }
        dioxus_router::components::Outlet::<crate::ui::components::Route>  {}
    }
}

#[component]
fn BoatTitle(boat: Result<BoatAndStats, ServerFnError>) -> Element {
    match boat {
        Ok(boat) => rsx! {
            div {
                class: "flex flex-row bg-ggrc items-center h-14 min-h-14",
                div {
                    "style": "min-width: 160px;",
                    class: "px-4 text-xl font-medium",
                    {boat.boat.name.clone()}
                }
                div {
                    class: "px-4",
                    {
                        format!("{} {}",boat.boat.weight_class, boat.boat.boat_type()?)
                    }
                }
            }
        },
        Err(error) => rsx! {
            div {
                {error.to_string()}
            }
        },
    }
}
