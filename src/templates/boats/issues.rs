use maud::{html, Markup};
use crate::db::{boat::types::BoatId, issue::Issue};

/// Boat issues page content (without page wrapper)
pub fn boat_issues_content(boat_id: BoatId, boat_name: &str, issues: &[Issue]) -> Markup {
    html! {
        div class="flex-grow flex flex-col bg-gray-50 dark:bg-gray-600" {
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
                h1 class="text-3xl font-bold text-gray-900 dark:text-white" {
                    (boat_name) " - Issues"
                }
            }

            @if issues.is_empty() {
                div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-8 text-center" {
                    p class="text-gray-600 dark:text-gray-300" {
                        "No issues found for this boat."
                    }
                }
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
        div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-4" {
            div class="flex justify-between items-start" {
                div class="flex-grow" {
                    div class="flex items-center gap-2 mb-2" {
                        @if is_resolved {
                            span class="px-2 py-1 bg-green-200 text-green-800 dark:bg-green-800 dark:text-green-200 rounded text-xs font-semibold" {
                                "Resolved"
                            }
                        } @else {
                            span class="px-2 py-1 bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200 rounded text-xs font-semibold" {
                                "Open"
                            }
                        }
                        span class="text-sm text-gray-500 dark:text-gray-400" {
                            "Recorded: " (issue.recorded_at.format("%Y-%m-%d %H:%M"))
                        }
                    }
                    p class="text-gray-900 dark:text-white" {
                        (issue.note)
                    }
                    @if let Some(resolved) = issue.resolved_at {
                        p class="text-sm text-gray-500 dark:text-gray-400 mt-2" {
                            "Resolved: " (resolved.format("%Y-%m-%d %H:%M"))
                        }
                    }
                }
                div class="ml-4" {
                    @if is_resolved {
                        button
                            hx-post=(format!("/issues/{}/unresolve", issue.id.as_int()))
                            hx-target="#content"
                            class="bg-yellow-500 hover:bg-yellow-600 text-white px-3 py-1 rounded text-sm font-semibold transition"
                        {
                            "Reopen"
                        }
                    } @else {
                        button
                            hx-post=(format!("/issues/{}/resolve", issue.id.as_int()))
                            hx-target="#content"
                            class="bg-green-500 hover:bg-green-600 text-white px-3 py-1 rounded text-sm font-semibold transition"
                        {
                            "Resolve"
                        }
                    }
                }
            }
        }
    }
}
