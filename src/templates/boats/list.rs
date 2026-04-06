use maud::{html, Markup};
use crate::db::boat::BoatAndStats;

/// Boat list page with statistics
pub fn boat_list_page(boats: &[BoatAndStats]) -> Markup {
    crate::templates::layout::page("Boats", boat_list_content(boats))
}

/// Boat list content (without page wrapper)
pub fn boat_list_content(boats: &[BoatAndStats]) -> Markup {
    html! {
        div class="overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)]" {
            div class="flex-grow flex flex-col items-center bg-gray-50 dark:bg-gray-600" {
                (boat_list(boats))
            }
        }
    }
}

/// Boat list component
pub fn boat_list(boats: &[BoatAndStats]) -> Markup {
    html! {
        div class="flex flex-col flex-grow xl:px-12 w-full bg-gray-50 dark:bg-slate-600 md:min-w-96 max-w-xxl" {
            // Header with Add New button
            div class="flex justify-between items-center p-4 bg-white dark:bg-slate-700 shadow-md" {
                h2 class="text-2xl font-bold text-gray-900 dark:text-white" { "Boats" }
                a
                    href="/boats/new"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition"
                    {
                    "+ Add New Boat"
                }
            }
            // Boat list
            div class="flex-grow divide-y-2 dark:divide-white dark:text-white dark:bg-slate-700 shadow-md" {
                @if boats.is_empty() {
                    div .p-8 .text-center {
                        p class="text-gray-500 dark:text-gray-300" { "No boats found." }
                    }
                } @else {
                    @for boat in boats {
                        (boat_row(boat))
                    }
                }
            }
        }
    }
}

/// Individual boat row
fn boat_row(boat: &BoatAndStats) -> Markup {
    html! {
        div class="flex flex-row flex-grow gap-2.5 py-1.5 px-4" {
            div class="flex flex-col flex-grow gap-2.5" {
                div class="text-xl font-medium min-w-40" {
                    a class="inline-flex items-center hover:underline"
                      href=(format!("/boats/{}", boat.boat.id.as_int())) {
                        span { (boat.boat.name) }
                        // Link icon
                        svg class="w-4 h-4 ml-1 fill-current" viewBox="0 0 24 24" {
                            path d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" {}
                        }
                    }
                }
                div {
                    @if let Some(boat_type) = boat.boat.boat_type() {
                        (boat.boat.weight_class.to_string())
                        " "
                        (boat_type.to_string())
                    } @else {
                        (boat.boat.weight_class.to_string())
                    }
                }
            }

            @if let Some(acquired) = boat.boat.acquired_at {
                div {
                    "Acquired at: "
                    (acquired.to_string())
                }
            }

            div {
                label { "Uses: " }
                (boat.total_uses.unwrap_or_default())
            }

            div {
                label { "Monthly Uses: " }
                (boat.uses_last_thirty_days.unwrap_or_default())
            }

            div {
                label { "Open Issues: " }
                @if let Some(issues) = boat.open_issues {
                    @if issues > 0 {
                        span class="font-bold text-red-600" { (issues) }
                    } @else {
                        (issues)
                    }
                } @else {
                    "0"
                }
            }
        }
    }
}
