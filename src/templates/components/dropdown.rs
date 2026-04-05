use maud::{html, Markup};

/// Alpine.js dropdown component for forms
///
/// Usage:
/// ```
/// dropdown(
///     "weight_class",
///     "Weight Class",
///     &["Light", "Medium", "Heavy", "Tubby"],
///     Some("Medium")
/// )
/// ```
pub fn dropdown(
    name: &str,
    label: &str,
    options: &[(&str, &str)], // (value, display)
    selected: Option<&str>,
) -> Markup {
    let selected_display = selected
        .and_then(|val| options.iter().find(|(v, _)| *v == val))
        .map(|(_, display)| *display)
        .unwrap_or("Select...");

    html! {
        div class="mb-4" x-data=(format!("{{ open: false, selected: '{}' }}", selected.unwrap_or(""))) {
            label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" {
                (label)
            }
            div class="relative" {
                // Dropdown button
                button
                    type="button"
                    "@click"="open = !open"
                    class="w-full bg-white border border-gray-300 rounded-lg px-4 py-2.5 text-left hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:hover:bg-gray-600"
                    {
                    span x-text=(format!("selected || '{}'", selected_display)) {}
                    // Dropdown arrow icon
                    svg class="absolute right-3 top-3 w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" {}
                    }
                }

                // Dropdown menu
                div
                    x-show="open"
                    "@click.away"="open = false"
                    class="absolute z-10 w-full mt-1 bg-white border border-gray-300 rounded-lg shadow-lg max-h-60 overflow-auto dark:bg-gray-700 dark:border-gray-600"
                    {
                    @for (value, display) in options {
                        button
                            type="button"
                            "@click"=(format!("selected = '{}'; open = false", display))
                            class="w-full px-4 py-2 text-left hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-white"
                            {
                            (display)
                            input
                                type="radio"
                                name=(name)
                                value=(value)
                                x-model="selected"
                                checked[selected == Some(*value)]
                                class="hidden";
                        }
                    }
                }
            }
        }
    }
}

/// Simpler dropdown variant without Alpine.js (pure HTML select)
pub fn simple_select(
    name: &str,
    label: &str,
    options: &[(&str, &str)],
    selected: Option<&str>,
) -> Markup {
    html! {
        div class="mb-4" {
            label for=(name) class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" {
                (label)
            }
            select
                name=(name)
                id=(name)
                class="w-full bg-white border border-gray-300 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                {
                @for (value, display) in options {
                    option value=(value) selected[selected == Some(*value)] {
                        (display)
                    }
                }
            }
        }
    }
}
