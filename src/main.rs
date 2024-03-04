use anyhow::Error;
use boat_tracking::{
    db::{
        boat::{BoatAndStats, NewBoat},
        use_event::{NewUseEvent, UseEvent, UseScenario},
    }, ui::AppState 
};
use diesel::Connection;


use axum::{extract::WebSocketUpgrade, response::Html, routing::get, Router};
use dioxus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");


    let conn_string = "db.sql";
    {
        let mut conn = diesel::SqliteConnection::establish(conn_string)?;
        let conn = &mut conn;
        let new_boat = NewBoat::new(
            "a good boat name".to_string(),
            boat_tracking::db::boat::types::WeightClass::Medium,
            boat_tracking::db::boat::types::BoatType::Eight,
            Some(chrono::Utc::now().naive_utc().date()),
            None,
        );
        let boat = boat_tracking::db::boat::Boat::new_boat(conn, new_boat)?;
        let new_event = NewUseEvent {
            boat_id: boat.id,
            recorded_at: chrono::Utc::now().naive_utc(),
            use_scenario: UseScenario::AM,
            note: Some("we had a good row".to_string()),
        };
        UseEvent::new_event(conn, new_event)?;
        let _boats = BoatAndStats::get_boats(conn)?;
    }


    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let state = AppState::new(
        conn_string
    );

    let app = Router::new()
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move || async move {
                Html(format!(
                    r#"
                <!DOCTYPE html>
                <html>
                <head> <title>GRC Boat Tracker</title>  </head>
                <body style="margin: 0; padding: 0"> <div id="main"></div> </body>
                {glue}
                </html>
                "#,
                    // Create the glue code to connect to the WebSocket on the "/ws" route
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
                ))
            }),
        )
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    _ = view.launch_with_props(dioxus_liveview::axum_socket(socket), boat_tracking::ui::app, state).await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();



    Ok(())
}