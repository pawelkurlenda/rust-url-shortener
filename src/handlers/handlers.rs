use std::io::{Error, ErrorKind};

use bytes::BytesMut;
use http_body_util::{combinators::BoxBody, BodyExt, BodyStream};
use hyper::{body::{Buf, Bytes, Incoming}, header, Request, Response, StatusCode};
use validator::Validate;

use crate::{
    id, models::{LinkRecord, ShortenRequest}, routes::{App, Resp}, store::store::Store
};

const MAX: usize = 1024 * 16;

pub async fn get_url_by_slug<S: Store>(
    app: App<S>,
    req: Request<Incoming>,
    id: String
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

    let res = match parse_and_validate::<ShortenRequest>(&mut req).await{
        Ok(r) => r,
        Err(e) => {
            let response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(full(format!(r#"{{"error": "{}"}}"#, e)))?;
            return Ok(response);
        }
    };
    
    let a = LinkRecord {
        id: "test".to_string(),
        target: res.url,
        created_at: chrono::Utc::now(),
        expires_at: res.expires_at,
    };
    
    app.store.put(a).await.unwrap(); // handle error properly

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
    id: String
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

async fn get_body(req: &mut Request<Incoming>) -> Result<Vec<u8>, Error> {
    let body = req.body_mut();
    let mut buf = BytesMut::with_capacity(1024);

    while let Some(frame) = body.frame().await {
        let frame = frame.map_err(|_| Error::new(ErrorKind::InvalidInput, "Failed to read frame"))?;
        if let Some(chunk) = frame.data_ref() {
            // deny if size would exceed limit
            if buf.len() + chunk.len() > MAX {
                return Err(Error::new(ErrorKind::InvalidInput, "Request body too large"));
            }
            buf.extend_from_slice(chunk);
        }
    }
    Ok(buf.to_vec())
}

async fn parse_and_validate<T>(req: &mut Request<Incoming>) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + Validate,
{
    let bytes = match get_body(req).await {
        Ok(b) => b,
        Err(e) => return Err(e), // 500
    };

    let value: T = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, format!("Failed to parse body: {}", e))), // 400
    };

    if let Err(e) = value.validate() {
        return Err(Error::new(ErrorKind::InvalidInput, format!("Validation failed: {}", e)));
    }

    Ok(value)
}