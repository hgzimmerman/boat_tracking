use deadpool_diesel::sqlite::Pool;
use dioxus_fullstack::prelude::{server_fn::middleware::BoxedService, FromServerContext};
use std::sync::Arc;
use dioxus::prelude::*;

pub type ArcPool = Arc<Pool>;

#[derive(Clone)]
pub struct AppState {
    pool: ArcPool,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
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
    pub async fn conn(&self) -> Result<deadpool_diesel::sqlite::Connection, anyhow::Error> {
        self.pool.get().await.map_err(From::from)
    }
    pub fn pool(&self) ->  Arc<deadpool_diesel::sqlite::Pool> {
        self.pool.clone()
    }
}


/// A type was not found in the server context
pub struct NotFoundInServerContext<T: 'static>(std::marker::PhantomData<T>);

impl<T: 'static> std::fmt::Debug for NotFoundInServerContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        write!(f, "`{type_name}` not found in server context")
    }
}

impl<T: 'static> std::fmt::Display for NotFoundInServerContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        write!(f, "`{type_name}` not found in server context")
    }
}

impl<T: 'static> std::error::Error for NotFoundInServerContext<T> {}


#[async_trait::async_trait]
impl FromServerContext<dioxus_fullstack::prelude::Axum> for AppState {
    type Rejection = NotFoundInServerContext<AppState>;

    async fn from_request(req: &dioxus_fullstack::prelude::DioxusServerContext) ->  Result<Self,Self::Rejection> {
        req.get().ok_or_else(|| {
            NotFoundInServerContext::<AppState>(std::marker::PhantomData::<AppState>)
        } )
    }
}

#[async_trait::async_trait]
impl axum::extract::FromRequestParts<AppState> for AppState {
    type Rejection = String;

    async fn from_request_parts(_parts: &mut axum::http::request::Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        Ok(state.clone())
    }
}



// impl<S> dioxus_fullstack::prelude::server_fn::middleware::Layer for StateProviderLayer {
//     fn layer(&self, inner: S) -> Self::Service {
//         BoxedService::new(inner, )
//         // Timeout::new(inner, self.timeout)
//     }
// }


// pub struct StateProviderLayer {
//     // state: AppState
// }

// impl<S> Layer<S> for StateProviderLayer {
//     type Service = LogService<S>;

//     fn layer(&self, service: S) -> Self::Service {
//         LogService {
//             // target: self.target,
//             service
//         }
//     }
// }

// // This service implements the Log behavior
// pub struct StateProviderService<S> {
//     service: S,
//     // state: AppState
// }

// impl<S, Request> Service<Request> for LogService<S>
// where
//     S: Service<Request>,
//     Request: fmt::Debug,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = S::Future;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&mut self, request: Request) -> Self::Future {
//         request.extract()
//         // Insert log statement here or other functionality
//         // println!("request = {:?}, target = {:?}", request, self.target);
//         self.service.call(request)
//     }
// }






// pub trait Layer<Req, Res>: Send + Sync + 'static {
//     /// Adds this layer to the inner service.
//     fn layer(&self, inner: BoxedService<Req, Res>) -> BoxedService<Req, Res>;
// }

// /// A type-erased service, which takes an HTTP request and returns a response.
// pub struct BoxedService<Req, Res>(pub Box<dyn Service<Req, Res> + Send>);

// impl<Req, Res> BoxedService<Req, Res> {
//     /// Constructs a type-erased service from this service.
//     pub fn new(service: impl Service<Req, Res> + Send + 'static) -> Self {
//         Self(Box::new(service))
//     }
// }

// /// A service converts an HTTP request into a response.
// pub trait Service<Request, Response> {
//     /// Converts a request into a response.
//     fn run(
//         &mut self,
//         req: Request,
//     ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
// }