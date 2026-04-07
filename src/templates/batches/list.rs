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
                div id="boats-tooltip"
                    class="fixed z-50 bg-slate-100 dark:bg-slate-600 dark:text-white rounded border-2 border-slate-200 dark:border-white p-2 min-w-48 pointer-events-none empty:hidden"
                {}
                style {
                    (maud::PreEscaped(r#"
                        #boats-tooltip::after {
                            content: '';
                            position: absolute;
                            top: 50%;
                            right: -12px;
                            transform: translateY(-50%);
                            border: 6px solid transparent;
                            border-left-color: rgb(203 213 225);
                        }
                        @media (prefers-color-scheme: dark) {
                            #boats-tooltip::after {
                                border-left-color: white;
                            }
                        }
                    "#))
                }
                script {
                    (maud::PreEscaped(r#"
                        var expectedPath = null;
                        function positionTooltip(cell) {
                            expectedPath = cell.getAttribute('hx-get');
                            var tip = document.getElementById('boats-tooltip');
                            var rect = cell.getBoundingClientRect();
                            tip.style.top = (rect.top + rect.height / 2) + 'px';
                            tip.style.transform = 'translateY(-50%)';
                            tip.style.right = (window.innerWidth - rect.left + 12) + 'px';
                            tip.style.left = '';
                            tip.innerHTML = '';
                        }
                        function hideTooltip() {
                            expectedPath = null;
                            var tip = document.getElementById('boats-tooltip');
                            tip.innerHTML = '';
                        }
                        document.addEventListener('scroll', hideTooltip, true);
                        document.body.addEventListener('htmx:afterSwap', function(evt) {
                            if (evt.detail.target.id === 'boats-tooltip' && evt.detail.requestConfig.path !== expectedPath) {
                                evt.detail.target.innerHTML = '';
                            }
                        });
                    "#))
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
                  href=(format!("/batches/{batch_id}")) {
                    (batch.batch.recorded_at.with_timezone(&chrono::Local).format("%Y-%m-%d %-I:%M %p"))
                }
            }
            td class="px-4 py-3 text-sm" {
                (scenario_name)
            }
            td class="px-4 py-3 text-sm text-right"
                onmouseleave="hideTooltip()"
            {
                span class="pl-6 py-3 -my-3 cursor-pointer"
                    hx-get=(format!("/api/batches/{batch_id}/boats"))
                    hx-trigger="mouseenter delay:300ms"
                    hx-target="#boats-tooltip"
                    hx-swap="innerHTML"
                    onmouseenter="positionTooltip(this)"
                {
                    (batch.use_counts)
                }
            }
        }
    }
}

/// Boat preview popup content (rendered on hover, inserted into shared tooltip)
pub fn boats_preview_popup(boat_names: &[String]) -> Markup {
    html! {
        ul {
            @for boat_name in boat_names {
                li { (boat_name) }
            }
        }
    }
}
