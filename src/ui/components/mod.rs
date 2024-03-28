pub mod batch;
pub mod boat;
pub mod boat_list;
pub mod issue_list;
pub mod new_issue;
pub mod toast;
mod batch_list;
mod modal;

use batch::BatchViewingPage;
use batch::{BatchCreationPage, BatchEditPage, BatchTemplateCreationPage};
use batch_list::BatchListPage;
use boat::{BoatSummary, BoatMonthlyUses, BoatYearlyUses, BoatEdit, BoatIssues};
use boat_list::BoatListPage;
use issue_list::IssueListPage;
// use new_boat::NewBoatPage;
use boat::creation_edit_form::NewBoatPage;
use new_issue::NewIssuePage;
use boat::BoatNav;

use crate::db::{boat::types::BoatId, use_event_batch::BatchId};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use self::batch_list::PageQueryParams;
use self::boat_list::BoatListNav;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[redirect("/", || Route::BatchListPage{page: PageQueryParams::default()})]
    #[nest("/boats")]
        #[layout(BoatListNav)]
        #[route("/")]
        BoatListPage,
        #[end_layout]

        #[route("/new")]
        NewBoatPage,

        #[layout(BoatNav)]
            #[route("/:id")]
            BoatSummary{id: BoatId},
            #[route("/:id/monthly")]
            BoatMonthlyUses{id: BoatId},
            #[route("/:id/yearly")]
            BoatYearlyUses{id: BoatId},
            #[route("/:id/issues")]
            BoatIssues{id: BoatId},
            #[route("/:id/edit")]
            BoatEdit{id: BoatId},
        #[end_layout]
    #[end_nest]
    #[nest("/issues")]
        #[route("/")]
        IssueListPage,
        #[route("/new")]
        NewIssuePage,
    #[end_nest]
    #[nest("/batches")]
        #[route("/?:page")]
        BatchListPage{page: PageQueryParams},
        #[route("/:id")]
        BatchViewingPage{id: BatchId},
        #[route("/edit/:id")]
        BatchEditPage{id: BatchId},
        #[nest("/new")]
            #[route("/")]
            BatchCreationPage,
            #[route("/:id")]
            BatchTemplateCreationPage{id: BatchId}
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            id: "main-nav",
            class: "bg-ggrc sticky top-0",
            ul {
                class: "flex items-center",
                li {
                    class: "mr-3",
                     Link {
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::BatchListPage{page: PageQueryParams::default()},
                         "Practices and Regattas"
                    }
                }
                li {
                    class: "mr-3",
                    Link {
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::BoatListPage {},
                        "Boats"
                    }
                }
            }
        }
        div {
            id: "content-wrapper",
            class: "flex flex-col flex-grow max-h-[calc(100vh-42px)] bg-slate-50 dark:bg-slate-500",
            Outlet::<Route> {}
        }
    }
}
