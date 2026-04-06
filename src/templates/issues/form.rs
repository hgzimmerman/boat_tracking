use maud::{html, Markup};
use crate::db::boat::Boat;
use crate::templates::components::common::{page_content, form_card, form_label, INPUT_CLASS, BTN_PRIMARY};

/// New issue form page
pub fn new_issue_page(boats: &[Boat]) -> Markup {
    crate::templates::layout::page("New Issue", page_content(html! {
        div class="w-full max-w-2xl p-4" {
            // Header with back button
            div class="mb-4" {
                a
                    href="/issues"
                    class="text-blue-500 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
                {
                    "\u{2190} Back to Issues"
                }
            }

            (form_card("Report New Issue", html! {}, html! {
                form
                    hx-post="/issues"
                    hx-target="body"
                    class="space-y-4"
                {
                    // Boat selector
                    div {
                        (form_label("boat_id", "Boat (optional)", false))
                        select
                            name="boat_id"
                            id="boat_id"
                            class=(INPUT_CLASS)
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
                        (form_label("note", "Issue Description", true))
                        textarea
                            name="note"
                            id="note"
                            rows="6"
                            required
                            class=(INPUT_CLASS)
                            placeholder="Describe the issue in detail..."
                        {}
                    }

                    // Recorded at (optional)
                    div {
                        (form_label("recorded_at", "Date & Time (optional)", false))
                        input
                            type="datetime-local"
                            name="recorded_at"
                            id="recorded_at"
                            class=(INPUT_CLASS);
                        p class="text-xs text-gray-500 dark:text-gray-400 mt-1" {
                            "Leave empty to use current time"
                        }
                    }

                    // Submit button
                    div class="pt-2" {
                        button
                            type="submit"
                            class=(format!("{BTN_PRIMARY} w-full justify-center"))
                        {
                            "Submit Issue"
                        }
                    }
                }
            }))
        }
    }))
}
