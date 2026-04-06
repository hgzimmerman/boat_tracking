use maud::{html, Markup};

/// Simple modal component
///
/// Usage:
/// ```
/// modal(false, html! {
///     h2 { "Modal Title" }
///     p { "Modal content..." }
/// })
/// ```
pub fn modal(hidden: bool, content: Markup) -> Markup {
    html! {
        div class=(if hidden { "hidden" } else { "block" }) {
            // Backdrop
            div class="fixed inset-0 bg-black bg-opacity-50 z-40" {}

            // Modal container
            div class="fixed inset-0 z-50 flex items-center justify-center p-4" {
                div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-lg w-full p-6" {
                    (content)
                }
            }
        }
    }
}
