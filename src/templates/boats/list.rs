use maud::{html, Markup};
use crate::db::boat::BoatAndStats;
use crate::templates::components::common::{page_content, page_header, empty_state, csv_export_link, boat_indicator, BTN_PRIMARY};

/// Boat list page with statistics
pub fn boat_list_page(boats: &[BoatAndStats]) -> Markup {
    crate::templates::layout::page("Boats", boat_list_content(boats))
}

/// Boat list content (without page wrapper)
pub fn boat_list_content(boats: &[BoatAndStats]) -> Markup {
    page_content(boat_list(boats))
}

/// Boat list component
pub fn boat_list(boats: &[BoatAndStats]) -> Markup {
    html! {
        div class="flex flex-col flex-grow xl:px-12 w-full bg-gray-50 dark:bg-slate-600 md:min-w-96 max-w-xxl" {
            (page_header("Boats", html! {
                (csv_export_link("/boats_export.csv"))
                a href="/boats/new" class=(BTN_PRIMARY) {
                    "+ Add New Boat"
                }
            }))

            @if boats.is_empty() {
                div class="p-4" {
                    (empty_state("No boats found."))
                }
            } @else {
                div class="bg-white dark:bg-slate-700 shadow-md overflow-x-auto" {
                    table class="w-full" {
                        thead class="dark:text-white" {
                            tr {
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Name" }
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Type" }
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Acquired" }
                                th class="px-4 py-3 text-right font-bold uppercase text-xs tracking-wider" { "Uses" }
                                th class="px-4 py-3 text-right font-bold uppercase text-xs tracking-wider" { "Monthly" }
                                th class="px-4 py-3 text-right font-bold uppercase text-xs tracking-wider" { "Open Issues" }
                            }
                        }
                        tbody class="divide-y dark:divide-gray-600" {
                            @for boat in boats {
                                (boat_row(boat))
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Individual boat row
fn boat_row(boat: &BoatAndStats) -> Markup {
    let boat_type_str = if let Some(boat_type) = boat.boat.boat_type() {
        format!("{} {}", boat.boat.weight_class, boat_type)
    } else {
        boat.boat.weight_class.to_string()
    };

    html! {
        tr class="hover:bg-gray-50 dark:hover:bg-gray-600 dark:text-white" {
            td class="px-4 py-3 text-sm" {
                a class="text-blue-600 hover:underline dark:text-blue-400 font-medium"
                  href=(format!("/boats/{}", boat.boat.id.as_int())) {
                    (boat.boat.name)
                }
                (boat_indicator(
                    boat.boat.weight_class,
                    boat.boat.seat_count.count(),
                    boat.boat.oars_per_seat.count() == 2,
                ))
            }
            td class="px-4 py-3 text-sm" {
                (boat_type_str)
            }
            td class="px-4 py-3 text-sm" {
                @if let Some(acquired) = boat.boat.acquired_at {
                    (acquired.to_string())
                } @else {
                    span class="text-gray-400" { "-" }
                }
            }
            td class="px-4 py-3 text-sm text-right" {
                (boat.total_uses.unwrap_or_default())
            }
            td class="px-4 py-3 text-sm text-right" {
                (boat.uses_last_thirty_days.unwrap_or_default())
            }
            td class="px-4 py-3 text-sm text-right" {
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
