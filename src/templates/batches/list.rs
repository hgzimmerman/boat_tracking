use maud::{html, Markup};
use crate::db::use_event_batch::BatchAndCounts;
use crate::db::use_scenario::{UseScenario, UseScenarioId};
use crate::templates::components::common::{page_content, page_header, empty_state, csv_export_link, BTN_PRIMARY};
use std::collections::HashMap;

/// Batch list page
pub fn batch_list_page(batches: &[BatchAndCounts], scenarios: &[UseScenario]) -> Markup {
    crate::templates::layout::page("Boat Uses", batch_list_content(batches, scenarios))
}

/// Batch list content (without page wrapper)
pub fn batch_list_content(batches: &[BatchAndCounts], scenarios: &[UseScenario]) -> Markup {
    page_content(batch_list(batches, scenarios))
}

/// Batch list component
pub fn batch_list(batches: &[BatchAndCounts], scenarios: &[UseScenario]) -> Markup {
    let scenario_names: HashMap<UseScenarioId, &str> = scenarios
        .iter()
        .map(|s| (s.id, s.name.as_str()))
        .collect();

    html! {
        div class="flex flex-col flex-grow xl:px-12 w-full bg-gray-50 dark:bg-slate-600 md:min-w-96 max-w-xxl" {
            (page_header("Boat Uses", html! {
                (csv_export_link("/uses_export.csv"))
                a href="/batches/new" class=(BTN_PRIMARY) {
                    "+ Record Boat Uses"
                }
            }))

            @if batches.is_empty() {
                div class="p-4" {
                    (empty_state("No batches found."))
                }
            } @else {
                div class="bg-white dark:bg-slate-700 shadow-md overflow-x-auto" {
                    table class="w-full" {
                        thead class="dark:text-white" {
                            tr {
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Date & Time" }
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Scenario" }
                                th class="px-4 py-3 text-right font-bold uppercase text-xs tracking-wider" { "Boats" }
                            }
                        }
                        tbody class="divide-y dark:divide-gray-600" {
                            @for batch in batches {
                                (batch_row(batch, &scenario_names))
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Individual batch row
fn batch_row(batch: &BatchAndCounts, scenario_names: &HashMap<UseScenarioId, &str>) -> Markup {
    let batch_id = batch.batch.id.as_int();
    let scenario_name = scenario_names
        .get(&batch.batch.use_scenario_id)
        .unwrap_or(&"Unknown");

    html! {
        tr class="hover:bg-gray-50 dark:hover:bg-gray-600 dark:text-white" {
            td class="px-4 py-3 text-sm" {
                a class="text-blue-600 hover:underline dark:text-blue-400 font-medium"
                  href=(format!("/batches/{}", batch_id)) {
                    (batch.batch.recorded_at.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M"))
                }
            }
            td class="px-4 py-3 text-sm" {
                (scenario_name)
            }
            td class="px-4 py-3 text-sm text-right relative cursor-pointer"
                onmouseenter="document.querySelectorAll('[id^=\"boats-preview-\"]').forEach(el => el.innerHTML = '')"
                onmouseleave=(format!("setTimeout(() => document.getElementById('boats-preview-{}').innerHTML = '', 100)", batch_id))
                hx-get=(format!("/api/batches/{}/boats", batch_id))
                hx-trigger="mouseenter delay:500ms"
                hx-target=(format!("#boats-preview-{}", batch_id))
                hx-swap="innerHTML"
            {
                (batch.use_counts)
                div id=(format!("boats-preview-{}", batch_id)) {}
            }
        }
    }
}

/// Boat preview popup (rendered on hover)
pub fn boats_preview_popup(boat_names: &[String]) -> Markup {
    html! {
        div class="absolute top-full right-0 mt-1 bg-slate-100 dark:bg-slate-600 rounded border-2 border-slate-200 dark:border-white z-50 p-2 min-w-48" {
            ul {
                @for boat_name in boat_names {
                    li { (boat_name) }
                }
            }
        }
    }
}
