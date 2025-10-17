use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    app_state::AppState,
    handlers::models::{ShortenRequest, ShortenResponse},
    store::store::Store,
};

pub async fn create<S: Store>(
    State(state): State<AppState<S>>,
    Json(mut body): Json<ShortenRequest>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("{e}") })),
        )
            .into_response();
    }

    // todo logic

    let id = "test".to_string(); //id::generate_id(6);

    let resp = ShortenResponse { id: id.clone() };
    (StatusCode::CREATED, Json(resp)).into_response()
}

pub async fn get_metadata_by_id<S: Store>(
    State(state): State<AppState<S>>,
    Path(id): Path<String>,
) -> impl IntoResponse {

    // todo logic
}

pub async fn delete_by_id<S: Store>(
    State(state): State<AppState<S>>,
    Path(id): Path<String>,
) -> impl IntoResponse {

    // todo logic
}

pub async fn redirect_by_id<S: Store>(
    State(state): State<AppState<S>>,
    Path(id): Path<String>,
) -> impl IntoResponse {

    // todo logic
}
