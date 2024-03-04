#![allow(non_snake_case)]
use std::{ops::DerefMut, sync::Arc};

use dioxus::prelude::*;

use crate::{db::{self, boat::BoatAndStats}, };

pub type ScopeState<'a> = Scope<'a, AppState>;

#[derive(Clone)]
pub struct AppState {
    // conn: std::sync::Arc<tokio::sync::Mutex<diesel::SqliteConnection>>,
    pool: Arc<deadpool_diesel::sqlite::Pool>,
}
impl AppState {
    pub fn new(conn_str: &str) -> Self {
        let manager = deadpool_diesel::sqlite::Manager::new(conn_str, deadpool_diesel::Runtime::Tokio1);
        let pool = deadpool_diesel::sqlite::Pool::builder(manager).max_size(40).build().expect("should build pool");

        let pool = Arc::new(pool);
        Self { 
            pool 
         }
    }
    // pub fn conn(&self) -> std::sync::Arc<tokio::sync::Mutex<diesel::SqliteConnection>> {
    //     self.conn.clone()
    // }
    pub async fn conn(&self) -> Result<deadpool_diesel::sqlite::Connection, anyhow::Error> {
        self.pool.get().await.map_err(From::from)
    }
    pub fn pool(&self) ->  Arc<deadpool_diesel::sqlite::Pool> {
        self.pool.clone()
    }
}



#[derive(Props)]
struct BoatRowProps<'a> {
    boat: &'a BoatAndStats
}

fn BoatRow<'a>(cx: Scope<'a, BoatRowProps<'a>>) -> Element<'a> {
    let boat = cx.props.boat;
    cx.render(rsx! {
        div {
            "style": "display:flex; flex-direction: horizontal; flex-grow: 1; gap: 10px; border: solid black 1px; padding: 6px",
            onclick: move |event| {
                // now, outer won't be triggered
                event.stop_propagation();
                
            },
            div {
                "style": "display:flex; flex-direction: column; flex-grow: 1; gap: 10px;",
                div {
                    "style": "min-width: 160px; font-size: x-large; font-weight: 500",
                    boat.boat.name.clone(),
                }
                div {
                    format!("{:?} {:?}",boat.boat.weight_class, boat.boat.boat_type().unwrap())
                }
            }
            
            boat.boat.acquired_at.map(|x| rsx! {
                div {
                    "Acquired at : ",
                    x.to_string()
                }
            })
            div {
                label {
                    "Uses: "
                }
                format!("{}",boat.total_uses.unwrap_or_default() )
            }
            div {
                label {
                    "Monthly Uses: "
                }
                format!("{}",boat.uses_last_thirty_days.unwrap_or_default())
            }
            div {
                label {
                    "Open Issues: "
                }
                format!("{}",boat.open_issues.unwrap_or_default())
            }
        }
    })
}


pub fn app(cx: Scope<AppState>) -> Element {

    let pool = cx.props.pool();
    let f = use_future(cx, (), |_| async move {
        let conn = pool.get().await.map_err(anyhow::Error::from)?;
        conn 
            .interact(|conn| {
                BoatAndStats::get_boats(conn).map_err(anyhow::Error::from)
            })
            .await
            .map_err(|e| anyhow::anyhow!("{}",e.to_string()))?
    });
    let x = match f.value() {
        Some(Ok(boats)) => {
            rsx! {
                div {
                    boats.iter().map(|boat| rsx! {
                        BoatRow {
                            boat: boat
                        } 
                    })
                }
            }
            
        },
        Some(Err(error)) => {
            rsx!{
                div {
                    "error: ",
                    error.to_string()
                }
            }
        }
        None => rsx! {
            div {
                "Loading"
            }
        },
    };
    
    cx.render(rsx! {
        div {
            "style": "background: beige; padding: 10px",
            x
        }
    })
}