use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, body::Incoming};

use crate::{
    routes::{App, Resp},
    store::store::Store,
};

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
    // Aggregate the body...
    let whole_body = req.collect().await?.aggregate();
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
