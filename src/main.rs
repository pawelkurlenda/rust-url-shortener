use std::{convert::Infallible, net::SocketAddr};

use hyper::{Request, Response, body::Bytes, server::conn::http2, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use http_body_util::{BodyExt, Full, combinators::BoxBody};

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

async fn map_endpoint(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/api/v1/:slug") => Ok(),
        (&hyper::Method::GET, "/api/v1/info/:slug") => Ok(),
        (&hyper::Method::POST, "/api/v1/shorten") => Ok(),
        (&hyper::Method::DELETE, "/api/v1/:slug") => Ok(),

        _ => {
            let body = Full::new(Bytes::from("Not Found"));
            let mut response = Response::new(body.boxed());
            *response.status_mut() = hyper::StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}

#[derive(Clone)]
// An Executor that uses the tokio runtime.
pub struct TokioExecutor;

// Implement the `hyper::rt::Executor` trait for `TokioExecutor` so that it can be used to spawn
// tasks in the hyper runtime.
// An Executor allows us to manage execution of tasks which can help us improve the efficiency and
// scalability of the server.
impl<F> hyper::rt::Executor<F> for TokioExecutor
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        tokio::task::spawn(fut);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Handle the connection from the client using HTTP/2 with an executor and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = http2::Builder::new(TokioExecutor)
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }
}
