use maud::{html, Markup};
use crate::db::boat::BoatAndStats;

/// Tab navigation for boat detail pages
fn boat_tabs(boat_id: i32, active: &str) -> Markup {
    let details_class = if active == "details" {
        "inline-block py-2 px-4 border-b-4 border-blue-500 font-semibold text-blue-600 dark:text-blue-400"
    } else {
        "inline-block py-2 px-4 border-b-4 border-transparent hover:border-gray-300 font-semibold text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-white"
    };

    let issues_class = if active == "issues" {
        "inline-block py-2 px-4 border-b-4 border-blue-500 font-semibold text-blue-600 dark:text-blue-400"
    } else {
        "inline-block py-2 px-4 border-b-4 border-transparent hover:border-gray-300 font-semibold text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-white"
    };

    html! {
        nav class="bg-white dark:bg-slate-700 border-b border-gray-200 dark:border-gray-600"
            hx-boost="true" hx-target="#content" hx-swap="innerHTML" {
            div class="flex space-x-4 px-8" {
                a href=(format!("/boats/{boat_id}")) class=(details_class) {
                    "Details & Charts"
                }
                a href=(format!("/boats/{boat_id}/issues")) class=(issues_class) {
                    "Issues"
                }
                a href=(format!("/boats/{boat_id}/edit"))
                  class="inline-block py-2 px-4 border-b-4 border-transparent hover:border-gray-300 font-semibold text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-white" {
                    "Edit Boat"
                }
                a href=(format!("/uses_export.csv?id={boat_id}"))
                  target="_blank"
                  hx-boost="false"
                  class="inline-flex items-center py-2 px-4 border-b-4 border-transparent hover:border-gray-300 font-semibold text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-white" {
                    img src="/download.svg" alt="Download" class="w-4 h-4 mr-1 opacity-60";
                    "Export CSV"
                }
            }
        }
    }
}

/// Boat detail page with charts
pub fn boat_detail_page(boat: &BoatAndStats) -> Markup {
    crate::templates::layout::page(
        &format!("{} - Boat Details", boat.boat.name),
        html! {
            div class="flex-grow flex flex-col bg-gray-50 dark:bg-gray-600" {
                (boat_tabs(boat.boat.id.as_int(), "details"))
                div class="p-8" {
                    (boat_detail(boat))
                }
            }
        },
    )
}

/// Boat detail component
pub fn boat_detail(boat: &BoatAndStats) -> Markup {
    html! {
        div class="max-w-6xl mx-auto" {
            // Header with boat name and actions
            div class="flex justify-between items-center mb-6" {
                h1 class="text-3xl font-bold text-gray-900 dark:text-white" {
                    (boat.boat.name)
                }
                div class="flex gap-2" {
                    a
                        href=(format!("/boats/{}/edit", boat.boat.id.as_int()))
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition"
                        {
                        "Edit"
                    }
                    a
                        href="/boats"
                        class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded transition"
                        {
                        "Back to List"
                    }
                }
            }

            // Boat information card
            div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-6 mb-6" {
                h2 class="text-2xl font-bold mb-4 text-gray-900 dark:text-white" { "Boat Information" }
                div class="grid grid-cols-2 gap-4" {
                    div {
                        p class="text-sm text-gray-600 dark:text-gray-300" { "Weight Class" }
                        p class="font-semibold dark:text-white" { (boat.boat.weight_class.to_string()) }
                    }
                    @if let Some(boat_type) = boat.boat.boat_type() {
                        div {
                            p class="text-sm text-gray-600 dark:text-gray-300" { "Boat Type" }
                            p class="font-semibold dark:text-white" { (boat_type.to_string()) }
                        }
                    }
                    @if let Some(acquired) = boat.boat.acquired_at {
                        div {
                            p class="text-sm text-gray-600 dark:text-gray-300" { "Acquired At" }
                            p class="font-semibold dark:text-white" { (acquired.to_string()) }
                        }
                    }
                    @if let Some(manufactured) = boat.boat.manufactured_at {
                        div {
                            p class="text-sm text-gray-600 dark:text-gray-300" { "Manufactured At" }
                            p class="font-semibold dark:text-white" { (manufactured.to_string()) }
                        }
                    }
                    div {
                        p class="text-sm text-gray-600 dark:text-gray-300" { "Total Uses" }
                        p class="font-semibold dark:text-white" { (boat.total_uses.unwrap_or(0)) }
                    }
                    div {
                        p class="text-sm text-gray-600 dark:text-gray-300" { "Uses (Last 30 Days)" }
                        p class="font-semibold dark:text-white" { (boat.uses_last_thirty_days.unwrap_or(0)) }
                    }
                    div {
                        p class="text-sm text-gray-600 dark:text-gray-300" { "Open Issues" }
                        @if let Some(issues) = boat.open_issues {
                            @if issues > 0 {
                                p class="font-semibold text-red-600" { (issues) }
                            } @else {
                                p class="font-semibold dark:text-white" { (issues) }
                            }
                        } @else {
                            p class="font-semibold dark:text-white" { "0" }
                        }
                    }
                }
            }

            // Usage charts
            div class="grid grid-cols-1 lg:grid-cols-2 gap-6" {
                // Daily usage chart (30 days)
                div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-6" {
                    h3 class="text-xl font-bold mb-4 text-gray-900 dark:text-white" { "Daily Usage (Last 30 Days)" }
                    img
                        src=(format!("/boats/{}/chart/daily", boat.boat.id.as_int()))
                        alt="Daily usage chart"
                        class="w-full";
                }

                // Monthly usage chart (12 months)
                div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-6" {
                    h3 class="text-xl font-bold mb-4 text-gray-900 dark:text-white" { "Monthly Usage (Last 12 Months)" }
                    img
                        src=(format!("/boats/{}/chart/monthly", boat.boat.id.as_int()))
                        alt="Monthly usage chart"
                        class="w-full";
                }
            }
        }
    }
}
