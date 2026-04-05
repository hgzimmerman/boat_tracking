use maud::{html, Markup};
use crate::db::issue::Issue;

/// Issue list page
pub fn issue_list_page(issues: &[Issue]) -> Markup {
    crate::templates::layout::page("Issues", html! {
        div .container .mx-auto .p-8 {
            div .flex .justify-between .items-center .mb-6 {
                h1 .text-3xl .font-bold { "Issues" }
                a class="inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white transition"
                  href="/issues/new" {
                    "+ New Issue"
                }
            }

            @if issues.is_empty() {
                div class="bg-gray-100 dark:bg-gray-700 p-8 rounded text-center" {
                    p class="text-gray-600 dark:text-gray-300" {
                        "No issues found. All boats are in good condition!"
                    }
                }
            } @else {
                div class="bg-white dark:bg-gray-800 shadow-md rounded overflow-hidden" {
                    table .w-full {
                        thead class="bg-gray-200 dark:bg-gray-700" {
                            tr {
                                th class="px-4 py-3 text-left text-sm font-semibold" { "Recorded" }
                                th class="px-4 py-3 text-left text-sm font-semibold" { "Boat ID" }
                                th class="px-4 py-3 text-left text-sm font-semibold" { "Note" }
                                th class="px-4 py-3 text-left text-sm font-semibold" { "Status" }
                                th class="px-4 py-3 text-left text-sm font-semibold" { "Resolved" }
                            }
                        }
                        tbody .divide-y {
                            @for issue in issues {
                                (issue_row(issue))
                            }
                        }
                    }
                }
            }
        }
    })
}

fn issue_row(issue: &Issue) -> Markup {
    let is_resolved = issue.resolved_at.is_some();

    html! {
        tr class="hover:bg-gray-50 dark:hover:bg-gray-700" {
            td class="px-4 py-3 text-sm" {
                (issue.recorded_at.format("%Y-%m-%d %H:%M"))
            }
            td class="px-4 py-3 text-sm" {
                @if let Some(boat_id) = issue.boat_id {
                    a class="text-blue-600 hover:underline"
                      href=(format!("/boats/{}", boat_id.as_int())) {
                        "Boat #" (boat_id.as_int())
                    }
                } @else {
                    span class="text-gray-400" { "N/A" }
                }
            }
            td class="px-4 py-3 text-sm" {
                (issue.note)
            }
            td class="px-4 py-3 text-sm" {
                @if is_resolved {
                    span class="px-2 py-1 bg-green-200 text-green-800 rounded text-xs font-semibold" {
                        "Resolved"
                    }
                } @else {
                    span class="px-2 py-1 bg-red-200 text-red-800 rounded text-xs font-semibold" {
                        "Open"
                    }
                }
            }
            td class="px-4 py-3 text-sm" {
                @if let Some(resolved) = issue.resolved_at {
                    (resolved.format("%Y-%m-%d"))
                } @else {
                    span class="text-gray-400" { "-" }
                }
            }
        }
    }
}
