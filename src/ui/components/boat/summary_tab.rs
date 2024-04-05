use dioxus::prelude::*;

use crate::db::boat::types::BoatId;



#[component]
pub fn BoatSummary(id: BoatId) -> Element {
    let boat_fut = use_resource(use_reactive!(|id| async move { super::get_boat(id).await }));

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow space-y-1",
            {
                match boat_fut.value().as_ref()?.clone() {
                    Ok(boat) => rsx!{
                        LabeledValue {
                            label: "Manufactured at:",
                            value: boat.boat.manufactured_at.as_ref().map(ToString::to_string)
                        }
                        LabeledValue {
                            label: "Acquired at:",
                            value: boat.boat.acquired_at.as_ref().map(ToString::to_string)
                        }
                        LabeledValue {
                            label: "Sold at:",
                            value: boat.boat.relinquished_at.as_ref().map(ToString::to_string)
                        }
                        LabeledValue {
                            label: "Open Issues:",
                            value: boat.open_issues.as_ref().map(ToString::to_string)
                        }
                        LabeledValue {
                            label: "Uses (30 days):",
                            value: boat.uses_last_thirty_days.as_ref().map(ToString::to_string)
                        }
                        LabeledValue {
                            label: "Uses (all time):",
                            value: boat.total_uses.as_ref().map(ToString::to_string)
                        }
                    },
                    Err(_) => return None
                }
            }
        }
    }
}

#[component]
fn LabeledValue(label: &'static str, value: Option<String>) -> Element {
    rsx!{
        div {
            class: "space-x-4",
            label {
                class: "text-lg",
                {label}
            }
            span {
                {value?}
            }
        }
    } 
}