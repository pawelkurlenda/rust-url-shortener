use std::sync::Arc;

use http_body_util::Full;
use hyper::{
    Method, Request, Response,
    body::{Bytes, Incoming},
};

use crate::{
    handlers::handlers::{create_shortened_url, delete_url_by_slug, get_url_by_slug},
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

pub type Resp = Response<Full<Bytes>>;

impl<S: Store> App<S> {
    pub async fn route(self, req: Request<Incoming>) -> Result<Resp, hyper::Error> {
        let method = req.method();
        let path = req.uri().path();

        let response = match (method, path) {
            (&Method::GET, "/health") => Ok(self.response_empty_ok()),
            (&Method::POST, "/api/shorten") => create_shortened_url(self, req).await,
            (&Method::GET, p) if p.starts_with("/api/") => {
                if let Some(id) = self.get_id_from_path(path) {
                    get_url_by_slug(self, req, id).await
                } else {
                    Ok(self.response_not_found())
                }
            }
            (&Method::DELETE, p) if p.starts_with("/api/") => {
                if let Some(id) = self.get_id_from_path(path) {
                    delete_url_by_slug(self, req, id).await
                } else {
                    Ok(self.response_not_found())
                }
            }
            _ => Ok(self.response_not_found()),
        };

        // match (method, path) {
        //     (&Method::GET, "/api/v1/:slug") => get_url_by_slug(self, req).await,
        //     (&Method::GET, "/api/v1/info/:slug") => get_url_by_slug(self, req).await,
        //     (&Method::POST, "/api/v1/shorten") => create_shortened_url(self, req).await,
        //     (&Method::DELETE, "/api/v1/:slug") => delete_url_by_slug(self, req).await,
        //     (&Method::GET, "/health") => Ok(self.response_empty_ok()),
        //     _ => Ok(self.response_not_found()),
        // }

        return response;
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

    fn get_id_from_path(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 4 {
            Some(parts[3].to_string())
        } else {
            None
        }
    }
}
