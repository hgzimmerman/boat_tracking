use anyhow::Error;


// TODO:
// Keep working on date parsing
// Make sure that the templates still work.
// don't allow editing things in the view practice screen.
// It seems like timezone offsets are getting applied twice in some places


fn main() -> Result<(), Error> {
    #[cfg(feature = "web")]
    {
        use tracing_subscriber::prelude::*;
        use tracing_web::MakeWebConsoleWriter;
        
        let filter = tracing_subscriber::filter::Targets::new()
            .with_target("boat_tracking", tracing::Level::DEBUG)
            .with_default(tracing::Level::WARN);

        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false) // Only partially supported across browsers
            .without_time() // std::time is not available in browsers, see note below
            .with_writer(MakeWebConsoleWriter::new()) // write events to the console
            .with_filter(filter);
        let perf_layer = tracing_web::performance_layer().with_details_from_fields(tracing_subscriber::fmt::format::Pretty::default());

        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(perf_layer)
            .init(); // Install these as subscribers to tracing events

        tracing::info!(dev_version = 1, "Starting app");

        dioxus_web::launch::launch_cfg(
            boat_tracking::ui::app,
            dioxus_web::Config::new().hydrate(false),
        );
    }

    #[cfg(feature = "ssr")]
    {
        use axum::routing::*;
        use boat_tracking::ui::state::AppState;
        use dioxus_fullstack::prelude::*;
        use std::{path::PathBuf, sync::Arc};
        use tokio::net::TcpListener;
        use tracing_subscriber::prelude::*;

        let conn_string = "db.sql";

        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(std::io::stdout)
            .with_filter(tracing::level_filters::LevelFilter::DEBUG);

        tracing_subscriber::registry().with(fmt_layer).init(); // Install these as subscribers to tracing events

        // Doesn't really work
        async fn state_populate_middleware(
            axum::extract::State(state): axum::extract::State<boat_tracking::ui::state::AppState>,
            mut request: axum::extract::Request,
            next: axum::middleware::Next,
        ) -> axum::response::Response {
            let mut context = DioxusServerContext::default();
            let _ = context.insert(state.clone());
            request.extensions_mut().insert(context);

            let response = next.run(request).await;

            response
        }

        tokio::runtime::Runtime::new()
            .expect("Should create runtime")
            .block_on(async move {
                let cfg = ServeConfigBuilder::new()
                    .assets_path(PathBuf::from("dist"))
                    .build();
                let ssr_state = SSRState::new(&cfg);

                let state = AppState::new(conn_string);
                let state1 = state.clone();

                let dom_factory =
                    || dioxus::dioxus_core::VirtualDom::new(boat_tracking::ui::empty_app);
                // let dom_factory = || dioxus::dioxus_core::VirtualDom::new(boat_tracking::ui::app);

                // build our application with some routes
                let app = Router::new()
                    .route("/uses_export.csv", get(boat_tracking::api::export_uses_csv_handler))
                    .route("/boats_export.csv", get(boat_tracking::api::export_boats_csv_handler))
                    .serve_static_assets("dist")
                    .await
                    .connect_hot_reload()
                    // .register_server_fns_with_handler("", |func| {
                    //     let state = state.clone();
                    //     move |req: axum::http::Request<axum::body::Body>| {
                    //         let mut context = DioxusServerContext::default();
                    //         let _ = context.insert(state.clone());
                    //         let mut service = dioxus_fullstack::server_fn_service(context, func);
                    //         async move {
                    //             let (req, body) = req.into_parts();
                    //             let req = axum::http::Request::from_parts(req, body);
                    //             let res = service.0.run(req);
                    //             match res.await {
                    //                 Ok(res) => Ok::<_, std::convert::Infallible>(res.map(|b| b.into())),
                    //                 Err(e) => {
                    //                     let mut res = axum::response::Response::new(axum::body::Body::from(e.to_string()));
                    //                     *res.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    //                     Ok(res)
                    //                 }
                    //             }
                    //         }
                    //     }
                    // })
                    .register_server_fns()
                    .fallback(get(render_handler_with_context).with_state((
                        move |ctx| {
                            ctx.insert::<AppState>(state.clone()).expect("should be able to add state");
                        },
                        cfg,
                        ssr_state,
                        Arc::new(dom_factory),
                    )))
                    .route_layer(axum::middleware::from_fn_with_state(
                        state1.clone(),
                        state_populate_middleware,
                    )) // this doesn't really work
                    .with_state(state1);

                // run it
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
                println!("running at http://{addr}");
                let tcp_lisener = TcpListener::bind(addr).await.expect("should bind to tcp");
                axum::serve(tcp_lisener, app.into_make_service())
                    .await
                    .expect("Should run server");
            });
    }

    Ok(())
}

// #[server]
// #[middleware(tower_http::timeout::TimeoutLayer::new(std::time::Duration::from_secs(1)))]
