use maud::{html, Markup};
use crate::db::boat::Boat;

/// New issue form page
pub fn new_issue_page(boats: &[Boat]) -> Markup {
    crate::templates::layout::page("New Issue", html! {
        div class="overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)]" {
            div class="flex-grow flex flex-col items-center bg-gray-50 dark:bg-gray-600 p-4" {
                div class="w-full max-w-2xl" {
                    // Header with back button
                    div class="mb-4" {
                        a
                            href="/issues"
                            class="text-blue-500 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
                        {
                            "← Back to Issues"
                        }
                    }

                    // Form card
                    div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-6" {
                        h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6" {
                            "Report New Issue"
                        }

                        form
                            hx-post="/issues"
                            hx-target="body"
                            class="space-y-4"
                        {
                            // Boat selector
                            div {
                                label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1" {
                                    "Boat (optional)"
                                }
                                select
                                    name="boat_id"
                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white"
                                {
                                    option value="" { "-- No specific boat --" }
                                    @for boat in boats {
                                        option value=(boat.id.as_int()) {
                                            (boat.name) " (" (boat.weight_class)
                                            @if let Some(bt) = boat.boat_type() {
                                                " " (bt)
                                            }
                                            ")"
                                        }
                                    }
                                }
                            }

                            // Issue note
                            div {
                                label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1" {
                                    "Issue Description"
                                }
                                textarea
                                    name="note"
                                    rows="6"
                                    required
                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white"
                                    placeholder="Describe the issue in detail..."
                                {}
                            }

                            // Recorded at (optional)
                            div {
                                label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1" {
                                    "Date & Time (optional)"
                                }
                                input
                                    type="datetime-local"
                                    name="recorded_at"
                                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded dark:bg-slate-600 dark:text-white";
                                p class="text-xs text-gray-500 dark:text-gray-400 mt-1" {
                                    "Leave empty to use current time"
                                }
                            }

                            // Submit button
                            div class="pt-2" {
                                button
                                    type="submit"
                                    class="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition"
                                {
                                    "Submit Issue"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
