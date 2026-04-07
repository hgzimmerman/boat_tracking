use maud::{html, Markup};
use crate::db::{boat::{Boat, BoatAndStats}, use_scenario::UseScenario};
use crate::templates::components::common::boat_indicator;

/// Batch creation page with two-pane interface
pub fn batch_creation_page(scenarios: &[UseScenario], selected_boats: &[Boat]) -> Markup {
    crate::templates::layout::page("Record Boat Uses", batch_creation_content(scenarios, selected_boats))
}

/// Batch creation content
pub fn batch_creation_content(scenarios: &[UseScenario], selected_boats: &[Boat]) -> Markup {
    html! {
        div class="overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)]" {
            div class="flex-grow flex flex-col bg-gray-50 dark:bg-gray-600 p-4" {
                // Header
                div class="mb-4" {
                    h2 class="text-2xl font-bold text-gray-900 dark:text-white" { "Record Boat Uses" }
                    p class="text-sm text-gray-600 dark:text-gray-300 mt-1" {
                        "Select boats for this practice or regatta session"
                    }
                }

                div class="flex flex-col md:flex-row gap-4 flex-grow" {
                    // Left pane: Search and boat list
                    div class="flex-1 bg-white dark:bg-slate-700 rounded shadow-md p-4 flex flex-col" {
                        h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-3" {
                            "Available Boats"
                        }

                        // Search and filters
                        div class="mb-4 space-y-2" {
                            input
                                type="text"
                                placeholder="Search boats..."
                                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white"
                                hx-post="/api/batches/search"
                                hx-trigger="keyup changed delay:300ms"
                                hx-target="#boat-search-results"
                                hx-include="[name^='filter_']"
                                name="search";

                            // Filter row 1: Weight Class and Oars Config
                            div class="flex gap-2" {
                                select
                                    name="filter_weight"
                                    class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 text-gray-400 text-sm"
                                    hx-post="/api/batches/search"
                                    hx-trigger="change"
                                    hx-target="#boat-search-results"
                                    hx-include="[name='search'], [name^='filter_']"
                                    hx-on:change="this.classList.toggle('text-gray-400', this.value === ''); this.classList.toggle('text-gray-900', this.value !== ''); this.classList.toggle('dark:text-white', this.value !== '')"
                                {
                                    option value="" { "All Weight Classes" }
                                    option value="Light" { "Light" }
                                    option value="Medium" { "Medium" }
                                    option value="Heavy" { "Heavy" }
                                    option value="Tubby" { "Tubby" }
                                }

                                select
                                    name="filter_oars"
                                    class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 text-gray-400 text-sm"
                                    hx-post="/api/batches/search"
                                    hx-trigger="change"
                                    hx-target="#boat-search-results"
                                    hx-include="[name='search'], [name^='filter_']"
                                    hx-on:change="this.classList.toggle('text-gray-400', this.value === ''); this.classList.toggle('text-gray-900', this.value !== ''); this.classList.toggle('dark:text-white', this.value !== '')"
                                {
                                    option value="" { "All Oar Configs" }
                                    option value="Scull" { "Sculling (2 oars)" }
                                    option value="Sweep" { "Sweep (1 oar)" }
                                }
                            }

                            // Filter row 2: Cox and Boat Type
                            div class="flex gap-2" {
                                select
                                    name="filter_cox"
                                    class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 text-gray-400 text-sm"
                                    hx-post="/api/batches/search"
                                    hx-trigger="change"
                                    hx-target="#boat-search-results"
                                    hx-include="[name='search'], [name^='filter_']"
                                    hx-on:change="this.classList.toggle('text-gray-400', this.value === ''); this.classList.toggle('text-gray-900', this.value !== ''); this.classList.toggle('dark:text-white', this.value !== '')"
                                {
                                    option value="" { "Coxed/Coxless" }
                                    option value="coxed" { "Coxed" }
                                    option value="coxless" { "Coxless" }
                                }

                                select
                                    name="filter_boat_type"
                                    class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 text-gray-400 text-sm"
                                    hx-post="/api/batches/search"
                                    hx-trigger="change"
                                    hx-target="#boat-search-results"
                                    hx-include="[name='search'], [name^='filter_']"
                                    hx-on:change="this.classList.toggle('text-gray-400', this.value === ''); this.classList.toggle('text-gray-900', this.value !== ''); this.classList.toggle('dark:text-white', this.value !== '')"
                                {
                                    option value="" { "All Boat Types" }
                                    option value="Single" { "1x (Single)" }
                                    option value="Double" { "2x (Double)" }
                                    option value="DoublePlus" { "2x+ (Double+)" }
                                    option value="Pair" { "2- (Pair)" }
                                    option value="PairPlus" { "2+ (Pair+)" }
                                    option value="Quad" { "4x (Quad)" }
                                    option value="QuadPlus" { "4x+ (Quad+)" }
                                    option value="Four" { "4- (Four)" }
                                    option value="FourPlus" { "4+ (Four+)" }
                                    option value="Octo" { "8x (Octo)" }
                                    option value="OctoPlus" { "8x+ (Octo+)" }
                                    option value="Eight" { "8+ (Eight)" }
                                }
                            }
                        }

                        // Boat list (will be populated by HTMX)
                        div
                            id="boat-search-results"
                            class="flex-grow overflow-y-auto"
                            hx-get="/api/batches/boats"
                            hx-trigger="load"
                        {
                            p class="text-gray-500 dark:text-gray-400 text-center py-8" {
                                "Loading boats..."
                            }
                        }
                    }

                    // Right pane: Selected boats and form
                    div class="flex-1 bg-white dark:bg-slate-700 rounded shadow-md p-4 flex flex-col" {
                        h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-3" {
                            "Selected Boats "
                            span id="selected-count" class="text-sm font-normal text-gray-600 dark:text-gray-400" {
                                "(" (selected_boats.len()) ")"
                            }
                        }

                        // Form for batch metadata + selected boats
                        form
                            hx-post="/batches"
                            hx-target="body"
                            class="flex flex-col flex-grow"
                        {
                            // Alpine.js only for scenario selector and datetime picker
                            div
                                x-data=(format!(
                                    "{{ useScenarioId: '{}', recordedAt: '', scenarioDefaults: {{{}}} }}",
                                    scenarios.first().map(|s| s.id.as_int()).unwrap_or(0),
                                    scenarios.iter()
                                        .filter_map(|s| s.default_time.map(|t| format!("'{}': '{}'", s.id.as_int(), t.format("%H:%M"))))
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                ))
                                class="mb-4 space-y-3"
                            {
                                // Use scenario selector
                                div {
                                    label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1" {
                                        "Use Scenario"
                                    }
                                    select
                                        name="use_scenario_id"
                                        x-model="useScenarioId"
                                        x-on:change="if (recordedAt === '' && scenarioDefaults[useScenarioId]) { const now = new Date(); recordedAt = now.toISOString().slice(0,11) + scenarioDefaults[useScenarioId] }"
                                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white"
                                        required
                                    {
                                        @for scenario in scenarios {
                                            option value=(scenario.id.as_int()) { (scenario.name) }
                                        }
                                    }
                                }

                                // Date/time picker
                                div {
                                    label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1" {
                                        "Date & Time (optional)"
                                    }
                                    input
                                        type="datetime-local"
                                        name="recorded_at"
                                        x-model="recordedAt"
                                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white";
                                }
                            }

                            // Server-rendered selected boats container
                            div id="selected-boats-container" class="flex flex-col flex-grow" {
                                (selected_boats_fragment(selected_boats))
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders the selected boats panel interior as an HTMX fragment.
/// Returned by add/remove session handlers and rendered inline on page load.
pub fn selected_boats_fragment(boats: &[Boat]) -> Markup {
    html! {
        // OOB swap to update the count in the header
        span id="selected-count" hx-swap-oob="true" class="text-sm font-normal text-gray-600 dark:text-gray-400" {
            "(" (boats.len()) ")"
        }

        // Hidden inputs for form submission
        @for boat in boats {
            input type="hidden" name="boat_ids[]" value=(boat.id.as_int());
        }

        // Submit button
        div class="pt-2 mb-4" {
            button
                type="submit"
                class="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition disabled:opacity-50 disabled:cursor-not-allowed"
                disabled[boats.is_empty()]
            {
                "Record Boat Uses"
            }
        }

        // Selected boats list
        div class="flex-grow overflow-y-auto border-t border-gray-200 dark:border-gray-600 pt-3" {
            @if boats.is_empty() {
                p class="text-gray-500 dark:text-gray-400 text-center py-8 text-sm" {
                    "No boats selected yet"
                }
            } @else {
                div class="space-y-2" {
                    @for boat in boats {
                        (selected_boat_row(boat))
                    }
                }
            }
        }
    }
}

fn selected_boat_row(boat: &Boat) -> Markup {
    let boat_type = if let Some(bt) = boat.boat_type() {
        format!("{} {}", boat.weight_class, bt)
    } else {
        boat.weight_class.to_string()
    };

    html! {
        div class="flex items-center justify-between p-2 bg-gray-50 dark:bg-slate-600 rounded" {
            div class="flex-grow" {
                div class="font-medium text-gray-900 dark:text-white" { (boat.name) }
                div class="text-sm text-gray-600 dark:text-gray-400" { (boat_type) }
                (boat_indicator(
                    boat.weight_class,
                    boat.seat_count.count(),
                    boat.oars_per_seat.count() == 2,
                ))
            }
            button
                type="button"
                class="text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 font-bold"
                hx-post=(format!("/api/batches/session/remove/{}", boat.id.as_int()))
                hx-target="#selected-boats-container"
                hx-swap="innerHTML"
            {
                "✕"
            }
        }
    }
}

/// Boat search results (for HTMX response)
pub fn boat_search_results(boats: &[BoatAndStats], search_query: Option<&str>) -> Markup {
    html! {
        @if boats.is_empty() {
            p class="text-gray-500 dark:text-gray-400 text-center py-8" {
                @if let Some(query) = search_query {
                    "No boats found matching \"" (query) "\""
                } @else {
                    "No boats found"
                }
            }
        } @else {
            div class="space-y-2" {
                @for boat in boats {
                    (boat_selectable_row(boat))
                }
            }
        }
    }
}

/// Individual boat row that can be added to selection
fn boat_selectable_row(boat: &BoatAndStats) -> Markup {
    let boat_type = if let Some(bt) = boat.boat.boat_type() {
        format!("{} {}", boat.boat.weight_class, bt)
    } else {
        boat.boat.weight_class.to_string()
    };

    html! {
        div
            class="flex items-center justify-between p-2 bg-gray-50 dark:bg-slate-600 rounded hover:bg-gray-100 dark:hover:bg-slate-500 cursor-pointer transition"
            hx-post=(format!("/api/batches/session/add/{}", boat.boat.id.as_int()))
            hx-target="#selected-boats-container"
            hx-swap="innerHTML"
        {
            div class="flex-grow" {
                div class="font-medium text-gray-900 dark:text-white" {
                    (boat.boat.name)
                }
                div class="text-sm text-gray-600 dark:text-gray-400" {
                    (boat_type)
                }
                (boat_indicator(
                    boat.boat.weight_class,
                    boat.boat.seat_count.count(),
                    boat.boat.oars_per_seat.count() == 2,
                ))
            }
            span class="text-blue-500 text-xl" { "+" }
        }
    }
}
