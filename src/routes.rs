use std::sync::Arc;

use http_body_util::{BodyExt, Full};
use hyper::{
    Method, Request, Response,
    body::{Body, Bytes, Incoming},
};

use crate::{
    handlers::{
        self,
        handlers::{create_shortened_url, delete_url_by_slug, get_url_by_slug},
    },
    store::store::Store,
};

pub struct App<S: Store> {
    pub store: Arc<S>,
    //pub cuckoo_filter:
}

impl<S: Store> Clone for App<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}

type Resp = Response<Full<Bytes>>;

impl<S: Store> App<S> {
    pub async fn route(self, req: Request<Incoming>) -> Result<Resp, hyper::Error> {
        let method = req.method();
        let path = req.uri().path();

        match (method, path) {
            (&Method::GET, "/api/v1/:slug") => get_url_by_slug(self, req).await,
            (&Method::GET, "/api/v1/info/:slug") => get_url_by_slug(self, req).await,
            (&Method::POST, "/api/v1/shorten") => create_shortened_url(self, req).await,
            (&Method::DELETE, "/api/v1/:slug") => delete_url_by_slug(self, req).await,
            (&Method::GET, "/health") => Ok(self.response_empty_ok()),
            _ => Ok(self.response_not_found()),
        }
    }

    fn response_empty_ok(&self) -> Resp {
        Response::builder()
            .status(hyper::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Full::new(Bytes::new()))
            .unwrap()
    }

    fn response_not_found(&self) -> Resp {
        Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .header("Content-Type", "application/json")
            .body(Full::new(Bytes::from("{\"error\":\"Not Found\"}")))
            .unwrap()
    }
}
