use maud::{html, Markup};
use crate::db::use_event_batch::BatchAndCounts;

/// Batch list page
pub fn batch_list_page(batches: &[BatchAndCounts]) -> Markup {
    crate::templates::layout::page("Batches", batch_list_content(batches))
}

/// Batch list content (without page wrapper)
pub fn batch_list_content(batches: &[BatchAndCounts]) -> Markup {
    html! {
        div class="overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)]" {
            div class="flex-grow flex flex-col items-center bg-gray-50 dark:bg-gray-600" {
                (batch_list(batches))
            }
        }
    }
}

/// Batch list component
pub fn batch_list(batches: &[BatchAndCounts]) -> Markup {
    html! {
        div class="flex flex-col flex-grow xl:px-12 w-full bg-gray-50 dark:bg-slate-600 md:min-w-96 max-w-xxl" {
            // Header with Add New button and Export
            div class="flex justify-between items-center p-4 bg-white dark:bg-slate-700 shadow-md" {
                h2 class="text-2xl font-bold text-gray-900 dark:text-white" { "Batches" }
                div class="flex gap-2" {
                    a
                        href="/uses_export.csv"
                        target="_blank"
                        class="inline-flex items-center bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded transition"
                    {
                        img src="/download.svg" alt="Download" class="w-4 h-4 mr-2 invert";
                        "Export CSV"
                    }
                    a
                        href="/batches/new"
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition"
                    {
                        "+ Add New Batch"
                    }
                }
            }
            // Batch list
            div class="flex-grow divide-y-2 dark:divide-white dark:text-white dark:bg-slate-700 shadow-md" {
                @if batches.is_empty() {
                    div .p-8 .text-center {
                        p class="text-gray-500 dark:text-gray-300" { "No batches found." }
                    }
                } @else {
                    @for batch in batches {
                        (batch_row(batch))
                    }
                }
            }
        }
    }
}

/// Individual batch row
fn batch_row(batch: &BatchAndCounts) -> Markup {
    let batch_id = batch.batch.id.as_int();
    html! {
        div class="flex flex-row flex-grow gap-2.5 py-1.5 px-4" {
            div class="flex flex-col flex-grow gap-2.5" {
                div class="text-xl font-medium min-w-40" {
                    a class="inline-flex items-center hover:underline"
                      href=(format!("/batches/{}", batch_id)) {
                        span { (batch.batch.recorded_at.format("%Y-%m-%d %H:%M")) }
                        // Link icon
                        svg class="w-4 h-4 ml-1 fill-current" viewBox="0 0 24 24" {
                            path d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" {}
                        }
                    }
                }
            }

            div {
                label { "Scenario: " }
                (batch.batch.use_scenario.to_string())
            }

            div class="relative"
                onmouseleave=(format!("setTimeout(() => document.getElementById('boats-preview-{}').innerHTML = '', 100)", batch_id))
            {
                label { "Boats: " }
                span class="cursor-pointer"
                    onmouseenter="document.querySelectorAll('[id^=\"boats-preview-\"]').forEach(el => el.innerHTML = '')"
                    hx-get=(format!("/api/batches/{}/boats", batch_id))
                    hx-trigger="mouseenter delay:500ms"
                    hx-target=(format!("#boats-preview-{}", batch_id))
                    hx-swap="innerHTML"
                {
                    (batch.use_counts)
                }
                // Boat list popup - positioned absolutely by the popup content itself
                div id=(format!("boats-preview-{}", batch_id)) {}
            }
        }
    }
}

/// Boat preview popup (rendered on hover)
pub fn boats_preview_popup(boat_names: &[String]) -> Markup {
    html! {
        div class="absolute top-1 left-0 bg-slate-100 dark:bg-slate-600 rounded border-2 border-slate-200 dark:border-white z-50 p-2 min-w-48" {
            ul {
                @for boat_name in boat_names {
                    li { (boat_name) }
                }
            }
        }
    }
}
