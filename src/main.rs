use anyhow::Error;


// #[tokio::main]
fn main() -> Result<(), Error> {

    let conn_string = "db.sql";
    #[cfg(feature = "ssr")]
    {
        use diesel::Connection;
        use boat_tracking::db;
        let mut conn = diesel::SqliteConnection::establish(conn_string)?;
        let conn = &mut conn;
        let new_boat = db::boat::NewBoat::new(
            "a good boat name".to_string(),
            db::boat::types::WeightClass::Medium,
            db::boat::types::BoatType::Eight,
            Some(chrono::Utc::now().naive_utc().date()),
            None,
        );
        let boat = db::boat::Boat::new_boat(conn, new_boat)?;
        let new_event = db::use_event::NewUseEvent {
            boat_id: boat.id,
            recorded_at: chrono::Utc::now().naive_utc(),
            use_scenario: db::use_event::UseScenario::AM,
            note: Some("we had a good row".to_string()),
        };
        db::use_event::UseEvent::new_event(conn, new_event)?;
        let boats = db::boat::BoatAndStats::get_boats(conn)?;
        println!("{}", boats.len())
    }


    #[cfg(feature = "web")]
    dioxus_web::launch_cfg(boat_tracking::ui::app, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "ssr")]
    {
        use axum::routing::*;
        use dioxus_fullstack::prelude::*;
        use boat_tracking::ui::state::AppState;
        
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let cfg = ServeConfigBuilder::new(boat_tracking::ui::app, ())
                    .assets_path("dist")
                    .build();
                let ssr_state = SSRState::new(&cfg);

                let state = AppState::new(conn_string);

                // build our application with some routes
                let app = Router::new()
                    .serve_static_assets("dist")
                    .connect_hot_reload()
                    // .register_server_fns("")
                    .register_server_fns_with_handler("", |func| {
                        let state = state.clone();
                        move |req: axum::http::Request<axum::body::Body>| {
                            let mut context = DioxusServerContext::default();
                            let _ = context.insert(state.clone());
                            let mut service = dioxus_fullstack::server_fn_service(context, func);
                            async move {
                                let (req, body) = req.into_parts();
                                let req = axum::http::Request::from_parts(req, body);
                                let res = service.0.run(req);
                                match res.await {
                                    Ok(res) => Ok::<_, std::convert::Infallible>(res.map(|b| b.into())),
                                    Err(e) => {
                                        let mut res = axum::response::Response::new(axum::body::Body::from(e.to_string()));
                                        *res.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        Ok(res)
                                    }
                                }
                            }
                        }
                    })
                    // .fallback(get(render_handler).with_state((cfg, ssr_state)));
                    .fallback(get(render_handler_with_context).with_state((
                        move |ctx|{
                            ctx.insert::<AppState>(state.clone()).unwrap();
                        },
                        cfg,
                        ssr_state,
                    )));
                    // dioxus_fullstack::axum_adapter::DioxusRouterExt

                // run it
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
                println!("running at http://{addr}");
                axum::Server::bind(&addr).serve(app.into_make_service())
                    .await
                    .unwrap();
            });
    }

    Ok(())
}
