use std::{net::SocketAddr, sync::Arc};

use hyper::{Request, body::Incoming, server::conn::http2, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::{routes::App, store::memory::MemoryStore};

mod handlers;
mod models;
mod routes;
mod shortener;
mod store;

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

    let listener = TcpListener::bind(addr).await?;

    let app = App {
        store: Arc::new(MemoryStore::default()),
    };

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        let app_clone = app.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http2::Builder::new(TokioExecutor)
                .serve_connection(
                    io,
                    service_fn(move |req: Request<Incoming>| {
                        let app = app_clone.clone();
                        async move { app.route(req).await }
                    }),
                )
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }
}
