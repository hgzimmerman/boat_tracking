use maud::{html, Markup, PreEscaped};
use crate::db::boat::types::WeightClass;
use crate::handlers::PaginationMeta;

/// Standard page content wrapper — consistent outer shell for all pages
pub fn page_content(content: Markup) -> Markup {
    html! {
        div class="overflow-y-auto flex flex-col flex-grow max-h-[calc(100vh-42px)]" {
            div class="flex-grow flex flex-col items-center bg-gray-50 dark:bg-slate-600" {
                (content)
            }
        }
    }
}

/// Page header bar — title on left, action buttons on right
pub fn page_header(title: &str, actions: Markup) -> Markup {
    html! {
        div class="flex justify-between items-center p-4 bg-white dark:bg-slate-700 shadow-md" {
            h2 class="text-2xl font-bold text-gray-900 dark:text-white" { (title) }
            div class="flex gap-2" {
                (actions)
            }
        }
    }
}

/// Card wrapper — consistent background, shadow, rounding, padding
pub fn card(content: Markup) -> Markup {
    html! {
        div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-6" {
            (content)
        }
    }
}

/// Card with bottom margin
pub fn card_mb(content: Markup) -> Markup {
    html! {
        div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-6 mb-6" {
            (content)
        }
    }
}

/// Empty state — centered message inside a card
pub fn empty_state(message: &str) -> Markup {
    html! {
        div class="bg-white dark:bg-slate-700 rounded-lg shadow-md p-8 text-center" {
            p class="text-gray-500 dark:text-gray-300" { (message) }
        }
    }
}

/// Status badge — "Open" (red) or "Resolved" (green)
pub fn status_badge(resolved: bool) -> Markup {
    if resolved {
        html! {
            span class="px-2 py-1 bg-green-200 text-green-800 dark:bg-green-800 dark:text-green-200 rounded text-xs font-semibold" {
                "Resolved"
            }
        }
    } else {
        html! {
            span class="px-2 py-1 bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200 rounded text-xs font-semibold" {
                "Open"
            }
        }
    }
}

// -- Button CSS class constants --

/// Primary button: blue
pub const BTN_PRIMARY: &str = "btn btn-primary";

/// Secondary button: gray
pub const BTN_SECONDARY: &str = "btn btn-secondary";

/// Danger button: red
pub const BTN_DANGER: &str = "btn btn-danger";

/// Small green button (for resolve actions)
pub const BTN_SM_GREEN: &str = "btn btn-sm bg-green-500 hover:bg-green-600 text-white";

/// Small yellow button (for reopen actions)
pub const BTN_SM_YELLOW: &str = "btn btn-sm bg-yellow-500 hover:bg-yellow-600 text-white";

// -- Form helpers --

/// Standard input CSS class (no error)
pub const INPUT_CLASS: &str = "w-full bg-white border border-gray-300 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-slate-600 dark:border-gray-500 dark:text-white";

/// Input CSS class with error border
pub const INPUT_CLASS_ERROR: &str = "w-full bg-white border border-red-500 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-slate-600 dark:border-gray-500 dark:text-white";

/// Returns the appropriate input class based on whether there's an error
pub fn input_class(has_error: bool) -> &'static str {
    if has_error { INPUT_CLASS_ERROR } else { INPUT_CLASS }
}

/// Section title inside a card
pub fn section_title(title: &str) -> Markup {
    html! {
        h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4" { (title) }
    }
}

/// Detail field — small label on top, value below
pub fn detail_field(label: &str, value: Markup) -> Markup {
    html! {
        div {
            p class="text-sm text-gray-600 dark:text-gray-300" { (label) }
            p class="font-semibold dark:text-white" { (value) }
        }
    }
}

/// Form field label
pub fn form_label(for_id: &str, text: &str, required: bool) -> Markup {
    html! {
        label for=(for_id) class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" {
            (text)
            @if required {
                span class="text-red-500" { "*" }
            }
        }
    }
}

/// Form field error message
pub fn form_error(error: Option<&String>) -> Markup {
    html! {
        @if let Some(err) = error {
            p class="mt-1 text-sm text-red-600" { (err) }
        }
    }
}

/// Form card wrapper — consistent card styling for forms
pub fn form_card(title: &str, errors_markup: Markup, content: Markup) -> Markup {
    html! {
        div class="bg-white dark:bg-slate-700 shadow-md rounded-lg px-8 pt-6 pb-8 w-full max-w-2xl" {
            h2 class="mb-6 text-2xl font-bold text-gray-900 dark:text-white" {
                (title)
            }
            (errors_markup)
            (content)
        }
    }
}

/// Error banner for form validation errors
pub fn error_banner(errors: &[&str]) -> Markup {
    if errors.is_empty() {
        return html! {};
    }
    html! {
        div class="mb-4 p-4 bg-red-100 border border-red-400 text-red-700 rounded" {
            p class="font-bold" { "Please fix the following errors:" }
            ul class="list-disc list-inside mt-2" {
                @for err in errors {
                    li { (err) }
                }
            }
        }
    }
}

/// CSV export link button
pub fn csv_export_link(href: &str) -> Markup {
    html! {
        a
            href=(href)
            target="_blank"
            class=(BTN_SECONDARY)
        {
            img src="/download.svg" alt="Download" class="w-4 h-4 mr-2 invert";
            "Export CSV"
        }
    }
}

// -- Boat indicator --

/// Color for weight class
fn weight_color(wc: WeightClass) -> &'static str {
    match wc {
        WeightClass::Light => "#22c55e",  // green-500
        WeightClass::Medium => "#3b82f6", // blue-500
        WeightClass::Heavy => "#f97316",  // orange-500
        WeightClass::Tubby => "#ef4444",  // red-500
    }
}

/// Build the raw SVG string for the boat indicator.
///
/// Renders an SVG bar with N dashes colored by weight class.
/// Dash count = seat_count (number of rowers).
/// Sculling dashes are rounded, sweep dashes are square.
fn boat_indicator_svg(weight_class: WeightClass, seat_count: i32, is_sculling: bool) -> String {
    let color = weight_color(weight_class);
    let dash_count = seat_count as usize;

    let svg_width = 120;
    let svg_height = 6;
    let gap = 4;
    let total_gaps = if dash_count > 1 { (dash_count - 1) * gap } else { 0 };
    let dash_width = (svg_width - total_gaps) / dash_count;
    let rx = if is_sculling { svg_height / 2 } else { 0 };

    let mut rects = String::new();
    for i in 0..dash_count {
        let x = i * (dash_width + gap);
        rects.push_str(&format!(
            r#"<rect x="{x}" y="0" width="{dash_width}" height="{svg_height}" rx="{rx}" fill="{color}"/>"#,
        ));
    }

    let label = format!("{} weight, {} seat{}", weight_class, seat_count, if seat_count == 1 { "" } else { "s" });
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{svg_width}" height="{svg_height}" class="mt-1" aria-label="{label}">{rects}</svg>"#,
    )
}

/// Bottom dash indicator for a boat (as Markup for templates).
pub fn boat_indicator(weight_class: WeightClass, seat_count: i32, is_sculling: bool) -> Markup {
    html! {
        (PreEscaped(boat_indicator_svg(weight_class, seat_count, is_sculling)))
    }
}

/// Bottom dash indicator as a raw string (for embedding in JS/Alpine.js data).
pub fn boat_indicator_raw(weight_class: WeightClass, seat_count: i32, is_sculling: bool) -> String {
    boat_indicator_svg(weight_class, seat_count, is_sculling)
}

/// Pagination controls: Previous / "Page X of Y" / Next
///
/// Uses HTMX to swap page content without a full reload.
/// `base_url` is the path without query params (e.g., "/batches").
pub fn pagination_controls(meta: &PaginationMeta, base_url: &str) -> Markup {
    if meta.total_pages <= 1 {
        return html! {};
    }
    html! {
        div class="flex justify-between items-center px-4 py-3 bg-white dark:bg-slate-700 border-t dark:border-gray-600" {
            @if meta.has_previous() {
                a class="text-sm text-blue-600 dark:text-blue-400 hover:underline cursor-pointer"
                    hx-get=(format!("{}?page={}&per_page={}", base_url, meta.current_page - 1, meta.per_page))
                    hx-target="#content"
                    hx-push-url="true"
                {
                    "Previous"
                }
            } @else {
                span class="text-sm text-gray-400 dark:text-gray-500" { "Previous" }
            }
            span class="text-sm text-gray-600 dark:text-gray-300" {
                "Page " (meta.current_page) " of " (meta.total_pages)
            }
            @if meta.has_next() {
                a class="text-sm text-blue-600 dark:text-blue-400 hover:underline cursor-pointer"
                    hx-get=(format!("{}?page={}&per_page={}", base_url, meta.current_page + 1, meta.per_page))
                    hx-target="#content"
                    hx-push-url="true"
                {
                    "Next"
                }
            } @else {
                span class="text-sm text-gray-400 dark:text-gray-500" { "Next" }
            }
        }
    }
}
