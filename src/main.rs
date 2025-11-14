use std::{net::SocketAddr, sync::Arc};

use tokio::net::TcpListener;

use crate::{
    app_state::AppState, cuckoo_filter::cuckoo_filter::CuckooFilter, store::memory::MemoryStore,
};

mod api_error;
mod app_settings;
mod app_state;
mod cuckoo_filter;
mod handlers;
mod id;
mod models;
mod router;
mod store;
use app_settings::settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    app_settings::resolve_settings()?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;

    let app = AppState {
        store: Arc::new(MemoryStore::default()),
        cuckoo_filter: Arc::new(CuckooFilter::new(1024)),
    };

    let app_router = router::build_router(app);

    axum::serve(listener, app_router).await.unwrap();

    // let config = axum_server::tls_rustls::RustlsConfig::from_pem_file(
    //         Path::new(&cert_path),
    //         Path::new(&key_path),
    //     ).await?;

    // axum_server::bind_rustls(addr, config);

    Ok(())
}
