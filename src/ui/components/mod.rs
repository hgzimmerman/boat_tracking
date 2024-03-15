pub mod boat_list;
pub mod boat;
pub mod new_boat;
pub mod new_issue;
pub mod issue_list;

use new_boat::NewBoatPage;
use boat_list::BoatListPage;
use boat::BoatPage;
use issue_list::IssueListPage;
use new_issue::NewIssuePage;

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
        NewIssuePage
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
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::BoatListPage {}, "Boats" } }
                li { Link { to: Route::NewBoatPage{}, "New Boat" } }
                li { Link { to: Route::IssueListPage{}, "Issues" } }
            }
        }
        Outlet::<Route> {}
    }
}