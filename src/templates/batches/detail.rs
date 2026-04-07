use maud::{html, Markup};
use crate::db::{boat::Boat, use_event::UseEvent, use_event_batch::UseEventBatch};
use crate::templates::components::common::{page_content, card_mb, card, section_title, boat_indicator, BTN_PRIMARY};

/// Batch detail page
pub fn batch_detail_page(
    batch: &UseEventBatch,
    boats: &[(UseEvent, Boat)],
) -> Markup {
    crate::templates::layout::page("Batch Details", batch_detail_content(batch, boats))
}

/// Batch detail content (without page wrapper)
pub fn batch_detail_content(
    batch: &UseEventBatch,
    boats: &[(UseEvent, Boat)],
) -> Markup {
    page_content(html! {
        div class="w-full max-w-6xl p-4" {
            // Header with back button and template button
            div class="mb-4 flex items-center justify-between" {
                a
                    href="/batches"
                    class="text-blue-500 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
                {
                    "\u{2190} Back to Boat Uses"
                }
                a
                    href=(format!("/batches/new?template={}", batch.id.as_int()))
                    class=(BTN_PRIMARY)
                {
                    "Use as Template"
                }
            }

            // Batch metadata card
            (card_mb(html! {
                (section_title("Batch Details"))
                div class="grid grid-cols-1 md:grid-cols-3 gap-4" {
                    div {
                        label class="block text-sm font-medium text-gray-600 dark:text-gray-300 mb-1" {
                            "Batch ID"
                        }
                        p class="text-lg text-gray-900 dark:text-white" {
                            (batch.id.as_int())
                        }
                    }
                    div {
                        label class="block text-sm font-medium text-gray-600 dark:text-gray-300 mb-1" {
                            "Date & Time"
                        }
                        p class="text-lg text-gray-900 dark:text-white" {
                            (batch.recorded_at.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M"))
                        }
                    }
                    div {
                        label class="block text-sm font-medium text-gray-600 dark:text-gray-300 mb-1" {
                            "Use Scenario"
                        }
                        p class="text-lg text-gray-900 dark:text-white" {
                            (batch.use_scenario.to_string())
                        }
                    }
                }
            }))

            // Boats used in this batch
            (card(html! {
                (section_title(&format!("Boats Used ({})", boats.len())))

                @if boats.is_empty() {
                    p class="text-gray-500 dark:text-gray-400 text-center py-8" {
                        "No boats were used in this batch."
                    }
                } @else {
                    div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4" {
                        @for (_event, boat) in boats {
                            (boat_card(boat))
                        }
                    }
                }
            }))
        }
    })
}

/// Individual boat card
fn boat_card(boat: &Boat) -> Markup {
    let boat_type = if let Some(bt) = boat.boat_type() {
        format!("{} {}", boat.weight_class, bt)
    } else {
        boat.weight_class.to_string()
    };

    html! {
        a
            href=(format!("/boats/{}", boat.id.as_int()))
            class="block bg-gray-50 dark:bg-slate-600 rounded-lg p-4 hover:bg-gray-100 dark:hover:bg-slate-500 transition"
        {
            div class="font-semibold text-gray-900 dark:text-white text-lg mb-2" {
                (boat.name)
            }
            div class="text-sm text-gray-600 dark:text-gray-400" {
                (boat_type)
            }
            (boat_indicator(
                boat.weight_class,
                boat.seat_count.count(),
                boat.oars_per_seat.count() == 2,
            ))
        }
    }
}
