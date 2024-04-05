use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::db::boat::types::BoatId;


#[component]
pub fn BoatMonthlyUses(id: BoatId) -> Element {
    let uses_fut = use_resource(use_reactive!(
        |id| async move { get_events_for_boat(id).await }
    ));

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            BoatUsesChart {
                use_events: uses_fut.value()()?,
                date_formatting: BoatUsesDateFormatting::Daily
            }
        }
    }
}

#[component]
pub fn BoatYearlyUses(id: BoatId) -> Element {
    let uses_fut = use_resource(use_reactive!(|id| async move {
        get_monthly_events_for_boat(id).await
    }));

    rsx! {
        div {
            class: "overflow-y-auto flex flex-col flex-grow",
            BoatUsesChart {
                use_events: uses_fut.value()()?,
                date_formatting: BoatUsesDateFormatting::Monthly
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BoatUsesDateFormatting {
    #[default]
    Daily,
    Monthly,
}

#[component]
fn BoatUsesChart(
    use_events: Result<Vec<(NaiveDate, f32)>, ServerFnError>,
    date_formatting: Option<BoatUsesDateFormatting>,
) -> Element {
    let date_format: Box<dyn Fn(NaiveDate) -> String> = match date_formatting.unwrap_or_default() {
        BoatUsesDateFormatting::Daily => {
            Box::new(|time: NaiveDate| time.format("%m-%d").to_string())
        }
        BoatUsesDateFormatting::Monthly => {
            Box::new(|time: NaiveDate| time.format("%y-%m").to_string())
        }
    };

    match use_events {
        Ok(timed_counts) => {
            rsx! {
                div {
                    class: "px-4",
                    h3 {
                        class: "text-lg",
                        "Uses"
                    }
                    if !timed_counts.iter().any(|(_date, uses)| *uses > 0.0 ) {
                        div {
                            "Boat has not been used."
                        }
                    } else {
                        dioxus_charts::BarChart {
                            // height: "calc(100% - 60px)",
                            height: "80%",
                            // width: "1000px",
                            // width: "calc(100% - 60px)",
                            width: "80%",
                            padding_top: 10,
                            padding_left: 40,
                            padding_bottom: 20,
                            padding_right: 40,
                            show_grid_ticks: true,
                            bar_width: "2%",
                            horizontal_bars: true,
                            label_interpolation: (|v| {
                                if v == 0.0 {
                                    "".to_string()
                                } else {
                                    format!("{v}")
                                }
                            }) as fn(f32) -> String,
                            series: vec![timed_counts.iter().map(|( _time, count,)| *count).collect::<Vec<_>>()],
                            labels: timed_counts.into_iter().map(|(time, _count)| date_format(time)).collect::<Vec<_>>(),
                        }
                    }
                }
            }
        }
        Err(error) => {
            rsx! {
                div {
                    "error: ",
                    {error.to_string()}
                }
            }
        }
    }
}


/// Currently gets a month's worth of data, by day
#[server(GetBoatEvents)]
pub(crate) async fn get_events_for_boat(
    id: BoatId,
) -> Result<Vec<(NaiveDate, f32)>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;

    conn.interact(move |conn| {
        let start = chrono::Utc::now().naive_local()
            - chrono::TimeDelta::try_days(30).expect("should be able to create a month");
        crate::db::use_event::UseEvent::daily_timeseries_for_boat(conn, id, start, None)
            .map_err(ServerFnError::from)
            .map(|x| {
                x.into_iter()
                    .map(|(date, count)| (date, count as f32))
                    .collect()
            })
    })
    .await?
}

/// Gets a year's worth of data for the boat
#[server(GetYearBoatEvents)]
pub(crate) async fn get_monthly_events_for_boat(
    id: BoatId,
) -> Result<Vec<(NaiveDate, f32)>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;

    conn.interact(move |conn| {
        let start = chrono::Utc::now().naive_local()
            - chrono::TimeDelta::try_days(365).expect("Should create year delta");
        crate::db::use_event::UseEvent::monthly_timeseries_for_boat(conn, id, start, None)
            .map_err(ServerFnError::from)
            .map(|x| {
                x.into_iter()
                    .map(|(date, count)| (date, count as f32))
                    .collect()
            })
    })
    .await?
}