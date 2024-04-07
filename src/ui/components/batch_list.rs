use crate::{
    db::{
        boat::Boat,
        use_event::UseScenario,
        use_event_batch::{BatchAndCounts, BatchId},
    },
    ui::{components::Route, util::MaskIcon},
};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::{components::Link, routable::FromQueryArgument};

#[server(GetBatches)]
async fn get_batches(
    scenario: Option<UseScenario>,
    offset: usize,
    limit: usize,
) -> Result<Vec<BatchAndCounts>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;

    conn.interact(move |conn| {
        crate::db::use_event_batch::UseEventBatch::get_most_recent_batches_and_their_use_count(
            conn, scenario, offset, limit,
        )
        .map_err(ServerFnError::from)
    })
    .await?
}

#[server(GetBoatsForBatch)]
async fn get_boats_for_batch(batch_id: BatchId) -> Result<Vec<Boat>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;

    conn.interact(move |conn| {
        crate::db::use_event_batch::UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)
            .map_err(ServerFnError::from)
            .map(|list| list.into_iter().map(|(_event, boat)| boat).collect())
    })
    .await?
}

#[component]
pub fn BatchList(offset: usize, limit: Signal<usize>) -> Element {
    let scenario: Signal<Option<UseScenario>> = use_signal(|| None);
    let batches_fut = use_resource(use_reactive(
        (&offset, &*limit.read(), &*scenario.read()),
        move |(offset, limit, scenario)| {
            async move {
                get_batches(scenario, offset, limit)
                    .await
                    .map(|batch_data| {
                        tracing::debug!(?batch_data);
                        batch_data
                    })
                    .map_err(|error| {
                        tracing::warn!(?error, "Colud not fetch");
                        error
                    })
            }
        },
    ));

    match batches_fut.value().read().as_ref()? {
        Ok(batches) => {
            rsx! {
                div {
                    id: "batch-page-background",
                    class: "flex flex-grow w-full bg-gray-50 dark:bg-gray-400 md:min-w-96 max-w-xxl  overflow-auto",
                    div {
                        id: "batch-list-background", // needed to get the margin
                        class: "flex-grow xl:mx-12 grow bg-gray-100 dark:bg-gray-500 shadow-md",
                        div {
                            id: "batch-list-container",
                            class: "divide-y-2 flex flex-col grow bg-gray-100 dark:bg-gray-500",
                            {
                                batches.iter().map(|batch_and_counts| {
                                    rsx!{
                                        BatchListRow {
                                            batch_and_counts: batch_and_counts.clone()
                                        }
                                    }
                                })
                            }
                        }
                    }
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

#[component]
fn BatchListRow(batch_and_counts: BatchAndCounts) -> Element {
    let BatchAndCounts { batch, use_counts } = batch_and_counts;
    let mut id: Signal<Option<BatchId>> = use_signal(|| None);

    // Some reason this won't fire for the first element on dioxus 0.5.0-alpha.2
    let boats_in_the_batch = use_resource(move || async move {
        let id = id();
        tracing::info!(?id, "getting boats for batch");
        if let Some(id) = id {
            let x = Some(get_boats_for_batch(id).await);
            tracing::trace!(?x);
            x
        } else {
            None
        }
    });

    let local_recorded_at = crate::ui::util::time::render_local(batch.recorded_at);
    rsx! {
        div {
            class: "flex flex-row h-16 items-center py-1 px-4",
            onmouseout: move |_event| {
                id.set(None);
            },
            div {
                class: "m-2 w-36",
                {batch.use_scenario.to_string()}
            }
            div {
                class: "m-2 w-40",
                {local_recorded_at}
            }
            div {
                class: "m-2 w-28",
                onmouseover: move |_event| {
                    tracing::debug!(?batch.id, "mouseover");
                    id.set(Some(batch.id));
                },
                {format!("{use_counts} boats used")}

                match boats_in_the_batch.value().as_ref() {
                    Some(x) => {
                        match x.as_ref() {
                            Some(Ok(boats)) => rsx!{
                                div {
                                    class: "relative",
                                    div {
                                        class: "absolute top-1 bg-slate-100 dark:bg-slate-600 rounded border-2 border-slate-200 dark:border-white z-50 p-2 min-w-48",
                                        ul {
                                            class: "",
                                            {
                                                boats.iter().map(|boat| rsx! {
                                                    li {
                                                        {boat.name.clone()}
                                                    }
                                                })
                                            }
                                        },
                                    }
                                }
                            },
                            Some(Err(error)) => rsx!{
                                div {
                                    {error.to_string()}
                                }
                            },
                            None => rsx!{}
                        }
                    },
                    None => rsx!{}
                }
            }
            // take up space
            div {
                class: "grow"
            }
            div {
                class: "",
                // ->  batch/:batch_id
                Link {
                    class: "btn btn-blue inline-flex items-center",
                    to: Route::BatchViewingPage { id: batch.id },
                    MaskIcon {
                        class: "fill-current w-4 h-4 mr-1 bg-white",
                        url: "/eye.svg"
                    }
                    span {
                        "View"
                    }
                }
                // -> batch/edit/:batch_id
                Link {
                    class: "btn btn-blue inline-flex items-center",
                    to: Route::BatchEditPage { id: batch.id },
                    MaskIcon {
                        class: "fill-current w-4 h-4 mr-1 bg-white",
                        url: "/pencil.svg"
                    }
                    span {
                        "Edit"
                    }
                }
                // -> batch/new/:batch_id
                Link {
                    class: "btn btn-blue inline-flex items-center",
                    to: Route::BatchTemplateCreationPage{ id: batch.id },
                    MaskIcon {
                        class: "fill-current w-4 h-4 mr-1 bg-white",
                        url: "/clipboard.svg"
                    }
                    span {
                        "Use as Template"
                    }
                }
            }
        }
    }
}

#[component]
pub fn BatchListPage(page: ReadOnlySignal<Page>) -> Element {
    let page = page.read().0;
    tracing::debug!(?page, "rendering batch list page");

    let limit_state: Signal<usize> = use_signal(|| 20);
    let offset_state: Memo<usize> = use_memo(use_reactive(
        (&page, &*limit_state.read()),
        |(page, limit): (usize, usize)| (page.saturating_sub(1)) * limit,
    ));
    rsx! {
        div {
            class: "flex flex-col overflow-hidden grow",
            // page header/nav
            div {
                class: "h-16 bg-ggrc flex flex-row items-center px-4",
                div {
                    class: "grow",
                    if *offset_state.read() != 0 {
                        Link {
                            class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                            to: Route::BatchListPage{page: Page(page.saturating_sub(1))},
                            "Newer"
                        }
                    } else {
                        // Link is disabled, we don't want it to do anything
                        a {
                            class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white cursor-not-allowed opacity-50",
                            href: "javascript:void(0);",
                            onclick: |e| {
                                e.stop_propagation()
                            },
                            "Newer"
                        }
                    }
                    Link {
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::BatchListPage{page: Page(page.saturating_add(1))},
                        "Older"
                    }
                    a {
                        class: "inline-flex items-center p-4",
                        href: format!("/uses_export.csv"),
                        target: "_blank",
                        MaskIcon {
                            class: "fill-current w-4 h-4 mr-1 bg-black",
                            url: "/download.svg"
                        }
                        span {
                            "Export all to CSV"
                        }
                    }
                }
                div {
                    Link {
                        class: "inline-flex items-center border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::BatchCreationPage,

                        MaskIcon {
                            class: "fill-current w-4 h-4 mr-1 bg-white",
                            url: "/plus.svg"
                        }
                        span {
                            "Record New Practice or Regatta"
                        }
                    }
                }
            }

            // the controls
            BatchList {
                offset: *offset_state.read(),
                limit: limit_state
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Page(usize);

impl Default for Page {
    fn default() -> Self {
        // 1 indexed
        Self(1)
    }
}
impl FromQueryArgument for Page {
    type Err = std::num::ParseIntError;
    fn from_query_argument(query: &str) -> Result<Self, Self::Err> {
        use std::str::FromStr;
        usize::from_str(query).map(Page)
    }
}
impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))?;

        Ok(())
    }
}
