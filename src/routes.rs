use http_body_util::{BodyExt, Full, combinators::BoxBody};
use hyper::{Request, Response, body::Bytes};

mod handlers;
use crate::handlers::get_url_by_slug;

pub async fn map_endpoint(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/api/v1/:slug") => handlers::get_url_by_slug(req).await,
        (&hyper::Method::GET, "/api/v1/info/:slug") => handlers::get_url_by_slug(req).await,
        (&hyper::Method::POST, "/api/v1/shorten") => handlers::get_url_by_slug(req).await,
        (&hyper::Method::DELETE, "/api/v1/:slug") => handlers::get_url_by_slug(req).await,

        _ => {
            let body = Full::new(Bytes::from("Not Found"));
            let mut response = Response::new(body.boxed());
            *response.status_mut() = hyper::StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}
