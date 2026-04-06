use maud::{html, Markup};
use crate::db::boat::BoatAndStats;
use crate::templates::components::common::{card_mb, card, detail_field, section_title, boat_indicator, BTN_PRIMARY, BTN_SECONDARY};

/// Tab navigation for boat detail pages
pub fn boat_tabs(boat_id: i32, active: &str) -> Markup {
    let active_class = "inline-block py-2 px-4 border-b-4 border-blue-500 font-semibold text-blue-600 dark:text-blue-400";
    let inactive_class = "inline-block py-2 px-4 border-b-4 border-transparent hover:border-gray-300 font-semibold text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-white";

    let details_class = if active == "details" { active_class } else { inactive_class };
    let issues_class = if active == "issues" { active_class } else { inactive_class };
    let edit_class = if active == "edit" { active_class } else { inactive_class };

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
                a href=(format!("/boats/{boat_id}/edit")) class=(edit_class) {
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
        boat_detail_content(boat),
    )
}

/// Boat detail content (without page wrapper)
pub fn boat_detail_content(boat: &BoatAndStats) -> Markup {
    html! {
        div class="flex-grow flex flex-col bg-gray-50 dark:bg-slate-600" {
            (boat_tabs(boat.boat.id.as_int(), "details"))
            div class="p-8" {
                (boat_detail(boat))
            }
        }
    }
}

/// Boat detail component
pub fn boat_detail(boat: &BoatAndStats) -> Markup {
    html! {
        div class="max-w-6xl mx-auto" {
            // Header with boat name and actions
            div class="flex justify-between items-center mb-6" {
                div {
                    h1 class="text-2xl font-bold text-gray-900 dark:text-white" {
                        (boat.boat.name)
                    }
                    (boat_indicator(
                        boat.boat.weight_class,
                        boat.boat.seat_count.count(),
                        boat.boat.oars_per_seat.count() == 2,
                    ))
                }
                div class="flex gap-2" {
                    a href=(format!("/boats/{}/edit", boat.boat.id.as_int())) class=(BTN_PRIMARY) {
                        "Edit"
                    }
                    a href="/boats" class=(BTN_SECONDARY) {
                        "Back to List"
                    }
                }
            }

            // Boat information card
            (card_mb(html! {
                (section_title("Boat Information"))
                div class="grid grid-cols-2 gap-4" {
                    (detail_field("Weight Class", html! { (boat.boat.weight_class.to_string()) }))
                    @if let Some(boat_type) = boat.boat.boat_type() {
                        (detail_field("Boat Type", html! { (boat_type.to_string()) }))
                    }
                    @if let Some(acquired) = boat.boat.acquired_at {
                        (detail_field("Acquired At", html! { (acquired.to_string()) }))
                    }
                    @if let Some(manufactured) = boat.boat.manufactured_at {
                        (detail_field("Manufactured At", html! { (manufactured.to_string()) }))
                    }
                    (detail_field("Total Uses", html! { (boat.total_uses.unwrap_or(0)) }))
                    (detail_field("Uses (Last 30 Days)", html! { (boat.uses_last_thirty_days.unwrap_or(0)) }))
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
            }))

            // Usage charts
            div class="grid grid-cols-1 lg:grid-cols-2 gap-6" {
                (card(html! {
                    (section_title("Daily Usage (Last 30 Days)"))
                    img
                        src=(format!("/boats/{}/chart/daily", boat.boat.id.as_int()))
                        alt="Daily usage chart"
                        class="w-full";
                }))

                (card(html! {
                    (section_title("Monthly Usage (Last 12 Months)"))
                    img
                        src=(format!("/boats/{}/chart/monthly", boat.boat.id.as_int()))
                        alt="Monthly usage chart"
                        class="w-full";
                }))
            }
        }
    }
}
