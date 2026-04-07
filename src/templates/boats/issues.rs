use maud::{html, Markup};
use crate::db::{boat::types::BoatId, issue::Issue};
use crate::templates::components::common::{card, empty_state, status_badge, BTN_SM_GREEN, BTN_SM_YELLOW};

/// Boat issues page content (without page wrapper)
pub fn boat_issues_content(boat_id: BoatId, boat_name: &str, issues: &[Issue]) -> Markup {
    html! {
        div class="flex-grow flex flex-col bg-gray-50 dark:bg-slate-600" {
            (super::detail::boat_tabs(boat_id.as_int(), "issues"))
            div class="p-8" {
                (boat_issues(boat_name, issues))
            }
        }
    }
}

/// Boat issues component
fn boat_issues(boat_name: &str, issues: &[Issue]) -> Markup {
    html! {
        div class="max-w-4xl mx-auto" {
            div class="flex justify-between items-center mb-6" {
                h1 class="text-2xl font-bold text-gray-900 dark:text-white" {
                    (boat_name) " - Issues"
                }
            }

            @if issues.is_empty() {
                (empty_state("No issues found for this boat."))
            } @else {
                div class="space-y-4" {
                    @for issue in issues {
                        (issue_card(issue))
                    }
                }
            }
        }
    }
}

/// Individual issue card
fn issue_card(issue: &Issue) -> Markup {
    let is_resolved = issue.resolved_at.is_some();

    html! {
        (card(html! {
            div class="flex justify-between items-start" {
                div class="flex-grow" {
                    div class="flex items-center gap-2 mb-2" {
                        (status_badge(is_resolved))
                        span class="text-sm text-gray-500 dark:text-gray-400" {
                            "Recorded: " (issue.recorded_at.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M"))
                        }
                    }
                    p class="text-gray-900 dark:text-white" {
                        (issue.note)
                    }
                    @if let Some(resolved) = issue.resolved_at {
                        p class="text-sm text-gray-500 dark:text-gray-400 mt-2" {
                            "Resolved: " (resolved.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M"))
                        }
                    }
                }
                div class="ml-4" {
                    @if is_resolved {
                        button
                            hx-post=(format!("/issues/{}/unresolve", issue.id.as_int()))
                            hx-target="#content"
                            class=(BTN_SM_YELLOW)
                        {
                            "Reopen"
                        }
                    } @else {
                        button
                            hx-post=(format!("/issues/{}/resolve", issue.id.as_int()))
                            hx-target="#content"
                            class=(BTN_SM_GREEN)
                        {
                            "Resolve"
                        }
                    }
                }
            }
        }))
    }
}
