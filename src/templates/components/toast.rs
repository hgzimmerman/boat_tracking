use maud::{html, Markup};

/// Toast message types
#[derive(Debug, Clone, Copy)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

/// Create a toast notification for HTMX out-of-band swap
///
/// Usage in a handler:
/// ```rust
/// html! {
///     // Main response content
///     div { "Operation completed" }
///
///     // Toast (out-of-band)
///     (toast("success-123", ToastType::Success, "Boat created successfully!"))
/// }
/// ```
pub fn toast(id: &str, toast_type: ToastType, message: &str) -> Markup {
    let (bg_color, icon) = match toast_type {
        ToastType::Success => ("bg-green-500", "✓"),
        ToastType::Error => ("bg-red-500", "✗"),
        ToastType::Warning => ("bg-yellow-500", "⚠"),
        ToastType::Info => ("bg-blue-500", "ℹ"),
    };

    html! {
        div hx-swap-oob="beforeend:#toast-container" {
            div
                id=(format!("toast-{}", id))
                class=(format!("{} text-white px-6 py-4 rounded shadow-lg flex items-center gap-3 min-w-64", bg_color))
                data-auto-dismiss="4000"
            {
                span .text-xl .font-bold { (icon) }
                span { (message) }
            }
        }
    }
}

/// Simple toast without OOB swap (for direct rendering)
pub fn simple_toast(toast_type: ToastType, message: &str) -> Markup {
    let (bg_color, icon) = match toast_type {
        ToastType::Success => ("bg-green-500", "✓"),
        ToastType::Error => ("bg-red-500", "✗"),
        ToastType::Warning => ("bg-yellow-500", "⚠"),
        ToastType::Info => ("bg-green-500", "ℹ"),
    };

    html! {
        div
            id=(format!("toast-{}", chrono::Utc::now().timestamp_millis()))
            class=(format!("{} text-white px-6 py-4 rounded shadow-lg flex items-center gap-3 min-w-64", bg_color))
            data-auto-dismiss="4000"
        {
            span .text-xl .font-bold { (icon) }
            span { (message) }
        }
    }
}

/// Convenience functions for common toast types
pub fn success_toast(message: &str) -> Markup {
    toast(&format!("{}", chrono::Utc::now().timestamp_millis()), ToastType::Success, message)
}

pub fn error_toast(message: &str) -> Markup {
    toast(&format!("{}", chrono::Utc::now().timestamp_millis()), ToastType::Error, message)
}

pub fn warning_toast(message: &str) -> Markup {
    toast(&format!("{}", chrono::Utc::now().timestamp_millis()), ToastType::Warning, message)
}

pub fn info_toast(message: &str) -> Markup {
    toast(&format!("{}", chrono::Utc::now().timestamp_millis()), ToastType::Info, message)
}

/// Simple convenience functions (no OOB)
pub fn simple_success_toast(message: &str) -> Markup {
    simple_toast(ToastType::Success, message)
}
