use maud::{html, Markup};
use crate::db::use_scenario::UseScenario;
use crate::templates::components::common::{page_content, form_card, form_label, INPUT_CLASS, BTN_PRIMARY};

/// New scenario form page
pub fn new_scenario_page() -> Markup {
    crate::templates::layout::page("New Scenario", page_content(html! {
        div class="w-full max-w-2xl p-4" {
            div class="mb-4" {
                a
                    href="/scenarios"
                    class="text-blue-500 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
                {
                    "\u{2190} Back to Scenarios"
                }
            }

            (form_card("New Scenario", html! {}, html! {
                form
                    hx-post="/scenarios"
                    hx-target="body"
                    class="space-y-4"
                {
                    (scenario_form_fields(None))

                    div class="pt-2" {
                        button
                            type="submit"
                            class=(format!("{BTN_PRIMARY} w-full justify-center"))
                        {
                            "Create Scenario"
                        }
                    }
                }
            }))
        }
    }))
}

/// Edit scenario form page
pub fn edit_scenario_page(scenario: &UseScenario) -> Markup {
    crate::templates::layout::page("Edit Scenario", page_content(html! {
        div class="w-full max-w-2xl p-4" {
            div class="mb-4" {
                a
                    href="/scenarios"
                    class="text-blue-500 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
                {
                    "\u{2190} Back to Scenarios"
                }
            }

            (form_card("Edit Scenario", html! {}, html! {
                form
                    hx-post=(format!("/scenarios/{}", scenario.id.as_int()))
                    hx-target="body"
                    class="space-y-4"
                {
                    (scenario_form_fields(Some(scenario)))

                    div class="pt-2" {
                        button
                            type="submit"
                            class=(format!("{BTN_PRIMARY} w-full justify-center"))
                        {
                            "Save Changes"
                        }
                    }
                }
            }))
        }
    }))
}

/// Shared form fields for create/edit
fn scenario_form_fields(existing: Option<&UseScenario>) -> Markup {
    html! {
        div {
            (form_label("name", "Name", true))
            input
                type="text"
                name="name"
                id="name"
                required
                class=(INPUT_CLASS)
                placeholder="e.g. Adult Practice"
                value=[existing.map(|s| s.name.as_str())];
        }

        div {
            (form_label("default_time", "Default Time (optional)", false))
            input
                type="time"
                name="default_time"
                id="default_time"
                class=(INPUT_CLASS)
                value=[existing.and_then(|s| s.default_time.map(|t| t.format("%H:%M").to_string()))];
            p class="text-xs text-gray-500 dark:text-gray-400 mt-1" {
                "Pre-populates the time field when this scenario is selected during batch creation"
            }
        }
    }
}
