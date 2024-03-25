use std::{path::PathBuf, sync::Arc};

use anyhow::Error;


// #[tokio::main]
fn main() -> Result<(), Error> {

    let conn_string = "db.sql";

    #[cfg(feature = "web")]
    {

        use tracing_web::{MakeWebConsoleWriter, performance_layer};
        use tracing_subscriber::fmt::format::Pretty;
        use tracing_subscriber::prelude::*;

        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false) // Only partially supported across browsers
            .without_time()   // std::time is not available in browsers, see note below
            .with_writer(MakeWebConsoleWriter::new()) // write events to the console
            .with_filter(tracing::level_filters::LevelFilter::DEBUG);
        let perf_layer = performance_layer()
            .with_details_from_fields(Pretty::default());

        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(perf_layer)
            .init(); // Install these as subscribers to tracing events

        dioxus_web::launch::launch_cfg(boat_tracking::ui::app, dioxus_web::Config::new().hydrate(true));
    }

    #[cfg(feature = "ssr")]
    {
        use axum::routing::*;
        use dioxus_fullstack::prelude::*;
        use boat_tracking::ui::state::AppState;
        use tokio::net::TcpListener;




        // Doesn't really work
        async fn state_populate_middleware(
            axum::extract::State(state): axum::extract::State<boat_tracking::ui::state::AppState>,
            mut request: axum::extract::Request,
            next: axum::middleware::Next,
        ) -> axum::response::Response {
            let mut context = DioxusServerContext::default();
            let _ = context.insert(state.clone());
            request.extensions_mut().insert(context);
            println!("running middleware");

            let response = next.run(request).await;


            response
        }
        
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let cfg = ServeConfigBuilder::new()
                    .assets_path(PathBuf::from("dist"))
                    .build();
                let ssr_state = SSRState::new(&cfg);

                let state = AppState::new(conn_string);
                let state1 = state.clone();

                let dom_factory = || dioxus::dioxus_core::VirtualDom::new(boat_tracking::ui::app);

                // build our application with some routes
                let app = Router::new()
                    .serve_static_assets("dist")
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
                        move |ctx|{
                            ctx.insert::<AppState>(state.clone()).unwrap();
                        },
                        cfg,
                        ssr_state,
                        Arc::new(dom_factory)
                    )))
                    .route_layer(axum::middleware::from_fn_with_state(state1.clone(), state_populate_middleware)) // this doesn't really work
                    .with_state(state1);

                // run it
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
                println!("running at http://{addr}");
                let tcp_lisener = TcpListener::bind(addr).await.expect("should bind to tcp");
                axum::serve(tcp_lisener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }

    Ok(())
}


// #[server]
// #[middleware(tower_http::timeout::TimeoutLayer::new(std::time::Duration::from_secs(1)))]