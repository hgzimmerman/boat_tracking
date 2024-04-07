use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::db::{boat::types::BoatId, use_event::UseScenario};

#[component]
pub fn BoatSummary(id: BoatId) -> Element {
    let boat_fut = use_resource(use_reactive!(|id| async move { super::get_boat(id).await }));

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow space-y-1",
            div {
                {
                    match boat_fut.value().as_ref()?.clone() {
                        Ok(boat) => rsx!{
                            LabeledValue {
                                label: "Manufactured at".to_string(),
                                value: boat.boat.manufactured_at.as_ref().map(ToString::to_string)
                            }
                            LabeledValue {
                                label: "Acquired at".to_string(),
                                value: boat.boat.acquired_at.as_ref().map(ToString::to_string)
                            }
                            LabeledValue {
                                label: "Sold at".to_string(),
                                value: boat.boat.relinquished_at.as_ref().map(ToString::to_string)
                            }
                            LabeledValue {
                                label: "Open Issues".to_string(),
                                value: boat.open_issues.as_ref().map(ToString::to_string)
                            }
                            LabeledValue {
                                label: "Uses (30 days)".to_string(),
                                value: boat.uses_last_thirty_days.as_ref().map(ToString::to_string)
                            }
                            LabeledValue {
                                label: "Uses (all time)".to_string(),
                                value: boat.total_uses.as_ref().map(ToString::to_string)
                            }
                        },
                        Err(_) => return None
                    }
                }
            }
            UsageBreakdown{
                id
            }
        }

    }
}

/// Convience component for simplifying the styling of the summary statistics.
#[component]
fn LabeledValue(label: String, value: Option<String>) -> Element {
    rsx! {
        div {
            class: "space-x-4",
            label {
                class: "text-lg",
                {label}":"
            }
            span {
                {value?}
            }
        }
    }
}

#[component]
fn LabeledValue2(label: String, value: Option<String>) -> Element {
    rsx! {
        div {
            class: "space-x-2 text-sm",
            label {
                {label}":"
            }
            span {
                {value?}
            }
        }
    }
}

/// Pie chart + legend that indicates what the distribution of 
/// uses for different types of practices and regattas is.
/// 
/// Not shown if the boat has not been used
#[component]
fn UsageBreakdown(id: ReadOnlySignal<BoatId>) -> Element {
    use dioxus_charts::{charts::pie::LabelPosition, PieChart};

    let events = use_resource(move || get_event_list_for_boat(*id.read()));

    match events.value()()? {
        Ok(events) => {
            // Rendering is too goofy if the SVG can't be drawn due to not having data,
            // so just don't render anything until we can solve that better.
            if events.is_empty() {
                return None
            }
            let legend = rsx! {
                div {
                    class: "flex flex-col",
                    div {
                        "style": "margin-left: -70px;",
                        class: "p-2 m-2 border",
                        label {
                            class: "text-lg",
                            "Legend"
                        }
                        div {
                            {
                                events.iter().map(|label_and_count| rsx!{
                                    LabeledValue2 {
                                        label: label_and_count.label.to_string(),
                                        value: label_and_count.count.to_string()
                                    }
                                })
                            }
                        }
                    }
                    div {
                        class: "grow"
                    }
                }
            };
            rsx! {
                div {
                    class: "flex flex-row flex-shrink border max-w-min min-w-max",
                    {
                        let events = EventCountsAndLabelLists::from(events);
                        rsx!{
                            div {
                                "style": "margin-left: -45px; font-size: x-small",
                                PieChart{
                                    width: "500px",
                                    height: "300px",
                                    viewbox_width: 500,
                                    viewbox_height: 300,
                                    series: events.counts,
                                    labels: events.labels,
                                    label_position: LabelPosition::Center,
                                    label_offset: 100.0,
                                    donut: true,
                                    donut_width: 90.0
                                }
                            }
                        }
                    }
                    {legend}
                }
            }
        }
        Err(error) => rsx! {
            div {
                {error.to_string()}
            }
        },
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventCountsAndLabel {
    label: UseScenario,
    count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventCountsAndLabelLists {
    labels: Vec<String>,
    counts: Vec<f32>,
}

impl From<Vec<EventCountsAndLabel>> for EventCountsAndLabelLists {
    fn from(value: Vec<EventCountsAndLabel>) -> Self {
        let len = value.len();
        value.into_iter().fold(
            EventCountsAndLabelLists {
                labels: Vec::with_capacity(len),
                counts: Vec::with_capacity(len),
            },
            |mut acc, EventCountsAndLabel { label, count }| {
                acc.labels.push(label.to_string());
                acc.counts.push(count as f32);
                acc
            },
        )
    }
}

#[server(GetCountAndLabelListForBoat)]
pub(crate) async fn get_event_list_for_boat(
    id: BoatId,
) -> Result<Vec<EventCountsAndLabel>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();

    let conn = state.pool().get().await?;

    let events = conn
        .interact(move |conn| {
            crate::db::use_event::UseEvent::events_for_boat(conn, id).map_err(ServerFnError::new)
        })
        .await??;

    let mut list_of_counts_by_use_scenario = events
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, next| {
            *acc.entry(next.use_scenario).or_default() += 1;
            acc
        })
        .into_iter()
        .map(|(label, count)| EventCountsAndLabel { label, count })
        .collect::<Vec<_>>();
    list_of_counts_by_use_scenario.sort_by_key(|x| x.label);
    Ok(list_of_counts_by_use_scenario)
}
