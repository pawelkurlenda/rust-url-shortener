use std::sync::Arc;

use http_body_util::{BodyExt, Full};
use hyper::{
    Request, Response,
    body::{Body, Bytes, Incoming},
};

use crate::store::Store;

pub struct App<S: Store> {
    pub store: Arc<S>,
}

impl<S: Store> Clone for App<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}

impl<S: Store> App<S> {
    pub async fn route(self, req: Request<Incoming>) -> Result<Response<Incoming>, hyper::Error> {
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        match (method, path.as_str()) {
            // (&hyper::Method::GET, "/health") => handlers::get_url_by_slug(req).await,
            // (&hyper::Method::GET, "/api/v1/:slug") => handlers::get_url_by_slug(req).await,
            // (&hyper::Method::GET, "/api/v1/info/:slug") => handlers::get_url_by_slug(req).await,
            // (&hyper::Method::POST, "/api/v1/shorten") => handlers::get_url_by_slug(req).await,
            // (&hyper::Method::DELETE, "/api/v1/:slug") => handlers::get_url_by_slug(req).await,
            _ => {
                let body = Full::new(Bytes::from("Not Found"));
                let mut response = Response::new(body.boxed());
                *response.status_mut() = hyper::StatusCode::NOT_FOUND;
                Ok(response)
            }
        }
    }
}

// pub async fn map_endpoint(
//     req: Request<hyper::body::Incoming>,
// ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
//     match (req.method(), req.uri().path()) {
//         (&hyper::Method::GET, "/health") => handlers::get_url_by_slug(req).await,
//         (&hyper::Method::GET, "/api/v1/:slug") => handlers::get_url_by_slug(req).await,
//         (&hyper::Method::GET, "/api/v1/info/:slug") => handlers::get_url_by_slug(req).await,
//         (&hyper::Method::POST, "/api/v1/shorten") => handlers::get_url_by_slug(req).await,
//         (&hyper::Method::DELETE, "/api/v1/:slug") => handlers::get_url_by_slug(req).await,

//         _ => {
//             let body = Full::new(Bytes::from("Not Found"));
//             let mut response = Response::new(body.boxed());
//             *response.status_mut() = hyper::StatusCode::NOT_FOUND;
//             Ok(response)
//         }
//     }
// }
