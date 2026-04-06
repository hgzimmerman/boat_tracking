use maud::{html, Markup, DOCTYPE};

/// Base page layout with navigation, HTMX, Alpine.js, and Tailwind CSS
pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " - GGRC Boat Tracker" }

                // Tailwind CSS
                link rel="stylesheet" href="/tailwind.css";

                // HTMX 2.0
                script src="/htmx.min.js" {}

                // Alpine.js
                script src="/alpine.min.js" defer {}

                // Toast auto-dismiss
                script src="/toast.js" {}
            }
            body class="bg-slate-50 dark:bg-slate-500 min-h-screen flex flex-col" {
                (navbar())

                div #content class="flex flex-col flex-grow max-h-[calc(100vh-42px)]" {
                    (content)
                }

                // Toast notification container
                div #toast-container .fixed .top-8 .right-8 .space-y-2 .z-50 {}
            }
        }
    }
}

/// Top-level navigation bar
fn navbar() -> Markup {
    html! {
        nav #main-nav .bg-ggrc .sticky .px-6 .py-3 .top-0 .z-40 {
            ul .flex .items-center .space-x-6 {
                li {
                    a href="/batches"
                      class="text-white font-semibold px-3 py-2 rounded hover:bg-white/25 cursor-pointer transition"
                      hx-get="/batches"
                      hx-target="body"
                      hx-push-url="true"
                      {
                        "Practices & Regattas"
                    }
                }
                li {
                    a href="/boats"
                      class="text-white font-semibold px-3 py-2 rounded hover:bg-white/25 cursor-pointer transition"
                      hx-get="/boats"
                      hx-target="body"
                      hx-push-url="true"
                      {
                        "Boats"
                    }
                }
                li {
                    a href="/issues"
                      class="text-white font-semibold px-3 py-2 rounded hover:bg-white/25 cursor-pointer transition"
                      hx-get="/issues"
                      hx-target="body"
                      hx-push-url="true"
                      {
                        "Issues"
                    }
                }
            }
        }
    }
}

/// Simple test page for verifying the setup
pub fn test_page() -> Markup {
    page("Test Page", html! {
        div .container .mx-auto .p-8 {
            h1 .text-4xl .font-bold .mb-4 { "HTMX + Maud Test Page" }

            p .mb-4 {
                "This is a proof-of-concept page to verify that HTMX, Alpine.js, and Maud are working correctly."
            }

            // Test HTMX with a simple click
            div .mb-8 {
                h2 .text-2xl .font-bold .mb-2 { "HTMX Test" }
                button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                    hx-get="/test/htmx-response"
                    hx-target="#htmx-result"
                    hx-swap="innerHTML" {
                    "Click to Test HTMX"
                }
                div #htmx-result .mt-4 .p-4 .bg-gray-100 .rounded {
                    "HTMX response will appear here"
                }
            }

            // Test Alpine.js with a counter
            div .mb-8 x-data="{ count: 0 }" {
                h2 .text-2xl .font-bold .mb-2 { "Alpine.js Test" }
                button
                    class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                    "@click"="count++" {
                    "Click to Increment"
                }
                p .mt-4 {
                    "Count: "
                    span .font-bold x-text="count" {}
                }
            }

            // Test navigation with HTMX boost
            div .mb-8 {
                h2 .text-2xl .font-bold .mb-2 { "Navigation Test" }
                p .mb-2 { "These links use HTMX boosting:" }
                ul .list-disc .pl-6 {
                    li { a class="text-blue-600 hover:underline" href="/test" { "Reload this page" } }
                    li { a class="text-blue-600 hover:underline" href="/boats" { "Go to boats (not yet implemented)" } }
                }
            }

            // Show that Tailwind is working
            div .mb-8 {
                h2 .text-2xl .font-bold .mb-2 { "Tailwind CSS Test" }
                div .grid .grid-cols-3 .gap-4 {
                    div .bg-red-500 .text-white .p-4 .rounded { "Red Box" }
                    div .bg-green-500 .text-white .p-4 .rounded { "Green Box" }
                    div .bg-blue-500 .text-white .p-4 .rounded { "Blue Box" }
                }
            }
        }
    })
}

/// Simple HTMX response for testing
pub fn htmx_test_response() -> Markup {
    html! {
        div .text-green-600 .font-bold {
            "✓ HTMX is working correctly!"
        }
        p .text-sm .text-gray-600 .mt-2 {
            "This content was loaded via an HTMX GET request."
        }
    }
}
