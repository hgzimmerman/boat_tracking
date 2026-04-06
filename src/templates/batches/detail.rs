use maud::{html, Markup};
use crate::db::{boat::Boat, use_event::UseEvent, use_event_batch::UseEventBatch};

/// Batch detail page
pub fn batch_detail_page(
    batch: &UseEventBatch,
    boats: &[(UseEvent, Boat)],
) -> Markup {
    crate::templates::layout::page("Batch Details", batch_detail_content(batch, boats))
}

/// Batch detail content (without page wrapper)
pub fn batch_detail_content(
    batch: &UseEventBatch,
    boats: &[(UseEvent, Boat)],
) -> Markup {
    html! {
        div class="overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)]" {
            div class="flex-grow flex flex-col items-center bg-gray-50 dark:bg-gray-600 p-4" {
                div class="w-full max-w-6xl" {
                    // Header with back button
                    div class="mb-4 flex items-center gap-4" {
                        a
                            href="/batches"
                            class="text-blue-500 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
                        {
                            "← Back to Batches"
                        }
                    }

                    // Batch metadata card
                    div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-6 mb-4" {
                        h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4" {
                            "Batch Details"
                        }
                        div class="grid grid-cols-1 md:grid-cols-3 gap-4" {
                            div {
                                label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1" {
                                    "Batch ID"
                                }
                                p class="text-lg text-gray-900 dark:text-white" {
                                    (batch.id.as_int())
                                }
                            }
                            div {
                                label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1" {
                                    "Date & Time"
                                }
                                p class="text-lg text-gray-900 dark:text-white" {
                                    (batch.recorded_at.format("%Y-%m-%d %H:%M"))
                                }
                            }
                            div {
                                label class="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-1" {
                                    "Use Scenario"
                                }
                                p class="text-lg text-gray-900 dark:text-white" {
                                    (batch.use_scenario.to_string())
                                }
                            }
                        }
                    }

                    // Boats used in this batch
                    div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-6" {
                        h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4" {
                            "Boats Used ("(boats.len())")"
                        }

                        @if boats.is_empty() {
                            p class="text-gray-500 dark:text-gray-400 text-center py-8" {
                                "No boats were used in this batch."
                            }
                        } @else {
                            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4" {
                                @for (_event, boat) in boats {
                                    (boat_card(boat))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Individual boat card
fn boat_card(boat: &Boat) -> Markup {
    let boat_type = if let Some(bt) = boat.boat_type() {
        format!("{} {}", boat.weight_class, bt)
    } else {
        boat.weight_class.to_string()
    };

    html! {
        a
            href=(format!("/boats/{}", boat.id.as_int()))
            class="block bg-gray-50 dark:bg-slate-600 rounded-lg p-4 hover:bg-gray-100 dark:hover:bg-slate-500 transition"
        {
            div class="font-semibold text-gray-900 dark:text-white text-lg mb-2" {
                (boat.name)
            }
            div class="text-sm text-gray-600 dark:text-gray-400" {
                (boat_type)
            }
        }
    }
}
