use maud::{html, Markup};

/// New issue form page (placeholder for now)
pub fn new_issue_page() -> Markup {
    crate::templates::layout::page("New Issue", html! {
        div .container .mx-auto .p-8 {
            h1 .text-3xl .font-bold .mb-6 { "Report New Issue" }

            div class="bg-yellow-50 dark:bg-yellow-900 border border-yellow-200 dark:border-yellow-700 p-4 rounded mb-6" {
                p class="text-yellow-800 dark:text-yellow-200" {
                    "⚠️ This form will be implemented in a later phase (Phase 2: Forms)."
                }
            }

            div class="bg-white dark:bg-gray-800 shadow-md rounded p-6" {
                p .mb-4 { "Issue reporting functionality coming soon..." }

                a class="inline-block border border-gray-500 rounded py-2 px-4 bg-gray-500 hover:bg-gray-700 text-white transition"
                  href="/issues" {
                    "← Back to Issues"
                }
            }
        }
    })
}
