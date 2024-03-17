pub mod boat_list;
pub mod boat;
pub mod new_boat;
pub mod new_issue;
pub mod issue_list;
pub mod batch;

use new_boat::NewBoatPage;
use boat_list::BoatListPage;
use boat::BoatPage;
use issue_list::IssueListPage;
use new_issue::NewIssuePage;
use batch::BatchCreationPage;

use dioxus_router::prelude::*;
use dioxus::prelude::*;
use crate::db::boat::types::BoatId;


#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home, 
    #[nest("/boats")]
        #[route("/")]
        BoatListPage,
        #[route("/new")]
        NewBoatPage,
        #[route("/:id")]
        BoatPage{id: BoatId},
    #[end_nest]
    #[nest("/issues")]
        #[route("/")]
        IssueListPage,
        #[route("/new")]
        NewIssuePage,
    #[end_nest]
    #[nest("/batches")]
        #[route("/new")]
        BatchCreationPage
}
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Home Page, will remove this eventually"
        }
    })
}


#[component]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            class: "bg-ggrc",
            ul {
                class: "flex items-center justify-between",
                li { 
                    class: "mr-3", 
                    Link {
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::Home {}, 
                        "Home"
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
                li { 
                    class: "mr-3", 
                    Link { 
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::NewBoatPage{}, 
                        "New Boat" 
                    } 
                }
                li { 
                    class: "mr-3",
                     Link { 
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::IssueListPage{},
                         "Issues"
                    }
                }
                li { 
                    class: "mr-3",
                     Link { 
                        class: "inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white",
                        to: Route::BatchCreationPage{},
                         "Record Boats Used" 
                    } 
                }
            }
        }
        Outlet::<Route> {}
    }
}