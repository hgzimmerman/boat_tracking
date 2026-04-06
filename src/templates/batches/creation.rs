use maud::{html, Markup};
use crate::db::{boat::{Boat, BoatAndStats}, use_event::UseEvent};

/// Batch creation page with two-pane interface
pub fn batch_creation_page(template_boats: Option<&[(UseEvent, Boat)]>) -> Markup {
    crate::templates::layout::page("Create Batch", batch_creation_content(template_boats))
}

/// Batch creation content
pub fn batch_creation_content(template_boats: Option<&[(UseEvent, Boat)]>) -> Markup {
    // Build initial selectedBoats JSON array for Alpine.js
    let initial_boats_json = if let Some(boats) = template_boats {
        let boats_array: Vec<String> = boats.iter().map(|(_event, boat)| {
            let boat_type = if let Some(bt) = boat.boat_type() {
                format!("{} {}", boat.weight_class, bt)
            } else {
                boat.weight_class.to_string()
            };
            format!(
                r#"{{ id: {}, name: '{}', type: '{}' }}"#,
                boat.id.as_int(),
                boat.name.replace('\'', "\\'"),
                boat_type.replace('\'', "\\'")
            )
        }).collect();
        format!("[{}]", boats_array.join(", "))
    } else {
        "[]".to_string()
    };

    html! {
        div class="overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)]" {
            div class="flex-grow flex flex-col bg-gray-50 dark:bg-gray-600 p-4" {
                // Header
                div class="mb-4" {
                    h2 class="text-2xl font-bold text-gray-900 dark:text-white" { "Create New Batch" }
                    p class="text-sm text-gray-600 dark:text-gray-300 mt-1" {
                        "Select boats for this practice or regatta session"
                    }
                }

                // Alpine.js data store for selected boats
                div
                    x-data=(format!("{{ selectedBoats: {}, useScenario: 'Adult', recordedAt: '' }}", initial_boats_json))
                    class="flex flex-col md:flex-row gap-4 flex-grow"
                {
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
                                    class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white text-sm"
                                    hx-post="/api/batches/search"
                                    hx-trigger="change"
                                    hx-target="#boat-search-results"
                                    hx-include="[name='search'], [name^='filter_']"
                                {
                                    option value="" { "All Weight Classes" }
                                    option value="Light" { "Light" }
                                    option value="Medium" { "Medium" }
                                    option value="Heavy" { "Heavy" }
                                    option value="Tubby" { "Tubby" }
                                }

                                select
                                    name="filter_oars"
                                    class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white text-sm"
                                    hx-post="/api/batches/search"
                                    hx-trigger="change"
                                    hx-target="#boat-search-results"
                                    hx-include="[name='search'], [name^='filter_']"
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
                                    class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white text-sm"
                                    hx-post="/api/batches/search"
                                    hx-trigger="change"
                                    hx-target="#boat-search-results"
                                    hx-include="[name='search'], [name^='filter_']"
                                {
                                    option value="" { "Coxed/Coxless" }
                                    option value="coxed" { "Coxed" }
                                    option value="coxless" { "Coxless" }
                                }

                                select
                                    name="filter_boat_type"
                                    class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white text-sm"
                                    hx-post="/api/batches/search"
                                    hx-trigger="change"
                                    hx-target="#boat-search-results"
                                    hx-include="[name='search'], [name^='filter_']"
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
                            span class="text-sm font-normal text-gray-600 dark:text-gray-400" {
                                "(" span x-text="selectedBoats.length" {} ")"
                            }
                        }

                        // Form for batch metadata
                        form
                            hx-post="/batches"
                            hx-target="body"
                            class="mb-4 space-y-3"
                        {
                            // Use scenario selector
                            div {
                                label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1" {
                                    "Use Scenario"
                                }
                                select
                                    name="use_scenario"
                                    x-model="useScenario"
                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white"
                                    required
                                {
                                    option value="YouthGgrc" { "Youth-GGRC" }
                                    option value="YouthSomerville" { "Youth-Somerville" }
                                    option value="Adult" selected { "Adult" }
                                    option value="LearnToRow" { "Learn to Row" }
                                    option value="ScullingSaturday" { "Sculling Saturday" }
                                    option value="PrivateSession" { "Private Session" }
                                    option value="Regatta" { "Regatta" }
                                    option value="Other" { "Other" }
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

                            // Hidden inputs for selected boat IDs
                            template x-for="boat in selectedBoats" x-bind:key="boat.id" {
                                input type="hidden" name="boat_ids[]" x-bind:value="boat.id";
                            }

                            // Submit button
                            div class="pt-2" {
                                button
                                    type="submit"
                                    class="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition disabled:opacity-50 disabled:cursor-not-allowed"
                                    x-bind:disabled="selectedBoats.length === 0"
                                {
                                    "Create Batch"
                                }
                            }
                        }

                        // Selected boats list
                        div class="flex-grow overflow-y-auto border-t border-gray-200 dark:border-gray-600 pt-3" {
                            template x-if="selectedBoats.length === 0" {
                                p class="text-gray-500 dark:text-gray-400 text-center py-8 text-sm" {
                                    "No boats selected yet"
                                }
                            }
                            template x-if="selectedBoats.length > 0" {
                                div class="space-y-2" {
                                    template x-for="boat in selectedBoats" x-bind:key="boat.id" {
                                        div class="flex items-center justify-between p-2 bg-gray-50 dark:bg-slate-600 rounded" {
                                            div class="flex-grow" {
                                                div class="font-medium text-gray-900 dark:text-white" x-text="boat.name" {}
                                                div class="text-sm text-gray-600 dark:text-gray-400" x-text="boat.type" {}
                                            }
                                            button
                                                type="button"
                                                class="text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 font-bold"
                                                x-on:click="selectedBoats = selectedBoats.filter(b => b.id !== boat.id)"
                                            {
                                                "✕"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
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
            x-on:click=(format!(
                "if (!selectedBoats.find(b => b.id === {})) {{ selectedBoats.push({{ id: {}, name: '{}', type: '{}' }}) }}",
                boat.boat.id.as_int(),
                boat.boat.id.as_int(),
                boat.boat.name.replace('\'', "\\'"),
                boat_type.replace('\'', "\\'")
            ))
        {
            div class="flex-grow" {
                div class="font-medium text-gray-900 dark:text-white" {
                    (boat.boat.name)
                }
                div class="text-sm text-gray-600 dark:text-gray-400" {
                    (boat_type)
                }
            }
            span class="text-blue-500 text-xl" { "+" }
        }
    }
}
