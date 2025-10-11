use bytes::BytesMut;
use http_body_util::{combinators::BoxBody, BodyExt, BodyStream};
use hyper::{body::{Buf, Bytes, Incoming}, header, Request, Response, StatusCode};
use tokio::stream;

use crate::{
    routes::{App, Resp},
    store::store::Store,
};

const MAX: usize = 1024 * 16;

pub async fn get_url_by_slug<S: Store>(
    app: App<S>,
    req: Request<Incoming>,
) -> Result<Resp, hyper::Error> {
    let whole_body = req.collect().await?.aggregate();
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;

    app.store.delete(id)

    data["test"] = serde_json::Value::from("test_value");
    // And respond with the new JSON.
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(json))?;
    Ok(response)
}

pub async fn create_shortened_url<S: Store>(
    app: App<S>,
    req: Request<Incoming>,
) -> Result<Resp, hyper::Error> {    
    let mut body = req.body_mut();
    let mut buf = BytesMut::with_capacity(1024);

    while let Some(frame) = body.frame().await {
        let frame = frame.map_err(|_| ())?;
        if let Some(chunk) = frame.data_ref() {
            // deny if size would exceed limit
            if buf.len() + chunk.len() > MAX {
                return Err(()); // 413
            }
            buf.extend_from_slice(chunk);
        }
    }
    let a = buf.to_vec()

    let whole_body = req.collect().await?.aggregate();
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    
    data["test"] = serde_json::Value::from("test_value");

    let bytes = hyper::body::to_bytes(req.body_mut())
        .await
        .map_err(|_| server_error())?;
    
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(json))?;
    Ok(response)
}

pub async fn delete_url_by_slug<S: Store>(
    app: App<S>,
    req: Request<Incoming>,
) -> Result<Resp, hyper::Error> {
    // Aggregate the body...
    let whole_body = req.body()..collect().await?.aggregate();
    // Decode as JSON...
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    // Change the JSON...
    data["test"] = serde_json::Value::from("test_value");
    // And respond with the new JSON.
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(json))?;
    Ok(response)
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}