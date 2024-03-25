use std::{ops::Deref, str::FromStr};

use crate::{
    db::{
        use_event::UseScenario,
        use_event_batch::BatchAndCounts,
    },
    ui::components::Route,
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
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    conn.interact(move |conn| {
        crate::db::use_event_batch::UseEventBatch::get_most_recent_batches_and_their_use_count(conn, scenario, offset, limit)
            .map_err(ServerFnError::from)
    })
    .await?
}

#[component]
pub fn BatchList(offset: usize, limit: Signal<usize>) -> Element {
    let scenario: Signal<Option<UseScenario>> = use_signal(|| None);
    let batches_fut = use_server_future(use_reactive(
        (&offset, &*limit.read(), &*scenario.read()),
        move |(offset, limit, scenario)| {
            async move {
                get_batches(scenario, offset, limit)
                    .await
                    .map(|batch_data| {
                        tracing::debug!(?batch_data);
                        // batch_data_state.set(batch_data.clone());
                        batch_data
                    })
                    .map_err(|error| {
                        tracing::warn!(?error, "Colud not fetch");
                        error
                    })
            }
        },
    ))?;

    let b = batches_fut.value();
    let b = b.read();
    let batches = b.deref().as_ref().unwrap().as_ref().unwrap();
    rsx! {
        div {
            class: "divide-x-2 flex flex-col overflow-auto grow",
            {
                batches.iter().map(|BatchAndCounts { batch, use_counts }| {
                    rsx!{
                        div {
                            class: "flex flex-row h-16 items-center ",
                            div {
                                class: "m-2 w-20",
                                {batch.use_scenario.to_string()}
                            }
                            div {
                                class: "m-2 w-40",
                                {batch.recorded_at.to_string()}
                            }
                            div {
                                class: "m-2 w-28",
                                {format!("{use_counts} boats used")}
                            }
                            // ->  batch/:batch_id
                            Link {
                                class: "btn btn-blue",
                                to: Route::BatchViewingPage { id: batch.id },
                                "View"
                            }
                            // -> batch/edit/:batch_id
                            Link {
                                class: "btn btn-blue",
                                to: Route::BatchEditPage { id: batch.id },
                                "Edit"
                            }
                            // -> batch/new/:batch_id
                            Link {
                                class: "btn btn-blue",
                                to: Route::BatchTemplateCreationPage{ id: batch.id },
                                "Use as Template"
                            }
                        }
                    }
                })
            }
        }
    }
}

#[component]
pub fn BatchListPage(page: PageQueryParams) -> Element {
    let page = page.page;
    tracing::debug!(?page, "rendering batch list page");

    let limit_state: Signal<usize> = use_signal(|| 20);
    let offset_state: Memo<usize> = use_memo(use_reactive(
        (&page, &*limit_state.read()),
        |(page, limit): (usize, usize)| (page.saturating_sub(1)) * limit,
    ));
    rsx! {
        div {
            class: "flex flex-col overflow-hide grow max-h-[calc(100vh-42px)]",
            // page header/nav
            div {
                class: "h-16 bg-ggrc flex flex-row",
                div {
                    class: "grow",
                    if *offset_state.read() != 0 {
                        Link {
                            class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                            to: Route::BatchListPage{page: PageQueryParams{page: page.saturating_sub(1)}},
                            "Newer"
                        }
                    }
                    Link {
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::BatchListPage{page: PageQueryParams{page: page.saturating_add(1)}},
                        "Older"
                    }
                }
                div {
                    Link {
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::BatchCreationPage,
                        "Record New Practice or Regatta"
                    }
                }


            }
            // the controls
            BatchList {
                offset: offset_state.read().clone(),
                limit: limit_state.clone()
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Page(usize);

impl FromStr for Page {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let search = "page=";
        if s.len() < search.len() {
            return Err("Input to small".to_string());
        }
        if &s[0..search.len()] == search {
            Ok(Page(
                usize::from_str(&s[search.len()..]).map_err(|e| e.to_string())?,
            ))
        } else {
            Err("Missing 'page='".to_string())
        }
    }
}
impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let page = self.0;
        f.write_fmt(format_args!("page={page}"))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PageQueryParams {
    page: usize,
}

impl Default for PageQueryParams {
    fn default() -> Self {
        // 1 indexed
        Self { page: 1 }
    }
}
impl FromQueryArgument for PageQueryParams {
    type Err = String;
    fn from_query_argument(query: &str) -> Result<Self, String> {
        Ok(Self {
            page: Page::from_str(query)?.0,
        })
    }
}
impl std::fmt::Display for PageQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let page = Page(self.page);
        page.fmt(f)
    }
}
