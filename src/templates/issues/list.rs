use maud::{html, Markup};
use crate::db::{issue::Issue, boat::Boat};
use crate::handlers::PaginationMeta;
use crate::templates::components::common::{page_content, page_header, empty_state, status_badge, pagination_controls, BTN_PRIMARY, BTN_SM_GREEN, BTN_SM_YELLOW};

/// Issue list content (without page wrapper)
pub fn issue_list_content(issues: &[(Issue, Option<Boat>)], meta: &PaginationMeta) -> Markup {
    page_content(html! {
        div class="flex flex-col flex-grow xl:px-12 w-full bg-gray-50 dark:bg-slate-600 md:min-w-96 max-w-xxl" {
            (page_header("Issues", html! {
                a href="/issues/new" class=(BTN_PRIMARY) {
                    "+ New Issue"
                }
            }))

            @if issues.is_empty() {
                div class="p-4" {
                    (empty_state("No issues found. All boats are in good condition!"))
                }
            } @else {
                div class="bg-white dark:bg-slate-700 shadow-md overflow-x-auto" {
                    table class="w-full" {
                        thead class="dark:text-white" {
                            tr {
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Recorded" }
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Boat" }
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Note" }
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Status" }
                                th class="px-4 py-3 text-left font-bold uppercase text-xs tracking-wider" { "Resolved" }
                                th class="px-4 py-3 text-right font-bold uppercase text-xs tracking-wider" { "Actions" }
                            }
                        }
                        tbody class="divide-y dark:divide-gray-600" {
                            @for (issue, boat) in issues {
                                (issue_row(issue, boat.as_ref()))
                            }
                        }
                    }
                }
                (pagination_controls(meta, "/issues"))
            }
        }
    })
}

fn issue_row(issue: &Issue, boat: Option<&Boat>) -> Markup {
    let is_resolved = issue.resolved_at.is_some();

    html! {
        tr class="hover:bg-gray-50 dark:hover:bg-gray-600 dark:text-white" {
            td class="px-4 py-3 text-sm" {
                (issue.recorded_at.with_timezone(&chrono::Local).format("%Y-%m-%d %-I:%M %p"))
            }
            td class="px-4 py-3 text-sm" {
                @if let Some(boat) = boat {
                    a class="text-blue-600 hover:underline dark:text-blue-400"
                      href=(format!("/boats/{}", boat.id.as_int())) {
                        (boat.name)
                    }
                } @else {
                    span class="text-gray-400" { "N/A" }
                }
            }
            td class="px-4 py-3 text-sm max-w-md truncate" {
                (issue.note)
            }
            td class="px-4 py-3 text-sm" {
                (status_badge(is_resolved))
            }
            td class="px-4 py-3 text-sm" {
                @if let Some(resolved) = issue.resolved_at {
                    (resolved.with_timezone(&chrono::Local).format("%Y-%m-%d"))
                } @else {
                    span class="text-gray-400" { "-" }
                }
            }
            td class="px-4 py-3 text-sm text-right" {
                @if is_resolved {
                    button
                        hx-post=(format!("/issues/{}/unresolve", issue.id.as_int()))
                        hx-target="body"
                        class=(BTN_SM_YELLOW)
                    {
                        "Reopen"
                    }
                } @else {
                    button
                        hx-post=(format!("/issues/{}/resolve", issue.id.as_int()))
                        hx-target="body"
                        class=(BTN_SM_GREEN)
                    {
                        "Resolve"
                    }
                }
            }
        }
    }
}
