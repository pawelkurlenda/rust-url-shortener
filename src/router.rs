use crate::AppState;
use crate::handlers;
use crate::store::store::Store;
use axum::{
    Router,
    routing::{get, post},
};
//use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};

const MAX_JSON: usize = 16 * 1024;

pub fn build_router<S: Store>(state: AppState<S>) -> Router {
    Router::new()
        .route("/api/shorten", post(handlers::handlers::create))
        .route(
            "/api/:id",
            get(handlers::handlers::redirect_by_id).delete(handlers::handlers::delete_by_id),
        )
        .route(
            "/api/:id/metadata",
            get(handlers::handlers::get_metadata_by_id),
        )
        //.layer(TraceLayer::new_for_http())
        //.layer(RequestBodyLimitLayer::new(MAX_JSON))
        .with_state(state)
}
