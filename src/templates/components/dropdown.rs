use maud::{html, Markup};
use super::common::INPUT_CLASS;

/// HTML select dropdown component
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
                class=(INPUT_CLASS)
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
