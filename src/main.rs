use std::{net::SocketAddr, sync::Arc};

use tokio::net::TcpListener;

use crate::{app_state::AppState, store::memory::MemoryStore};

mod app_state;
mod handlers;
mod id;
mod models;
mod router;
mod store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;

    let app = AppState {
        store: Arc::new(MemoryStore::default()),
    };

    let app_router = router::build_router(app.clone());

    axum::serve(listener, app_router).await.unwrap();

    // let config = axum_server::tls_rustls::RustlsConfig::from_pem_file(
    //         Path::new(&cert_path),
    //         Path::new(&key_path),
    //     ).await?;

    // axum_server::bind_rustls(addr, config);

    Ok(())

    // loop {
    //     let (stream, _) = listener.accept().await?;

    //     let io = TokioIo::new(stream);

    //     let app_clone = app.clone();

    //     tokio::task::spawn(async move {
    //         if let Err(err) = http2::Builder::new(TokioExecutor)
    //             .serve_connection(
    //                 io,
    //                 service_fn(move |req: Request<Incoming>| {
    //                     let app = app_clone.clone();
    //                     async move { app.route(req).await }
    //                 }),
    //             )
    //             .await
    //         {
    //             eprintln!("Error serving connection: {}", err);
    //         }
    //     });
    // }
}
