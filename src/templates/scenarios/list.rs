use maud::{html, Markup};
use crate::db::use_scenario::UseScenario;
use crate::templates::components::common::{page_content, page_header, empty_state, BTN_PRIMARY};

/// Scenario list page
pub fn scenario_list_page(scenarios: &[UseScenario]) -> Markup {
    crate::templates::layout::page("Scenarios", page_content(html! {
        div class="flex flex-col flex-grow xl:px-12 w-full bg-gray-50 dark:bg-slate-600 md:min-w-96 max-w-xxl" {
            (page_header("Scenarios", html! {
                a href="/scenarios/new" class=(BTN_PRIMARY) {
                    "+ New Scenario"
                }
            }))

            @if scenarios.is_empty() {
                div class="p-4" {
                    (empty_state("No scenarios found."))
                }
            } @else {
                div class="bg-white dark:bg-slate-700 shadow-md overflow-x-auto" {
                    table class="w-full" {
                        thead class="dark:text-white" {
                            tr {
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Name" }
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Default Time" }
                                th class="px-4 py-3 text-right font-bold uppercase text-xs tracking-wider" { "Actions" }
                            }
                        }
                        tbody class="divide-y dark:divide-gray-600" {
                            @for scenario in scenarios {
                                tr class="hover:bg-gray-50 dark:hover:bg-gray-600 dark:text-white" {
                                    td class="px-4 py-3 text-sm font-medium" {
                                        (scenario.name)
                                    }
                                    td class="px-4 py-3 text-sm" {
                                        @if let Some(time) = scenario.default_time {
                                            (time.format("%-I:%M %p"))
                                        } @else {
                                            span class="text-gray-400" { "-" }
                                        }
                                    }
                                    td class="px-4 py-3 text-sm text-right" {
                                        a
                                            href=(format!("/scenarios/{}/edit", scenario.id.as_int()))
                                            class="text-blue-600 hover:underline dark:text-blue-400"
                                        {
                                            "Edit"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }))
}
