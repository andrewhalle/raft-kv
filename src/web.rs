use std::sync::{Arc, Mutex};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use tracing::instrument;

use crate::kv::{self, Store};

fn router(state: WebState) -> Router {
    Router::new()
        .route(
            "/*key",
            get(kv_get).post(kv_post).put(kv_put).delete(kv_delete),
        )
        .with_state(state)
}

pub(super) async fn listen_client_requests(kv: Arc<Mutex<Store>>) -> Result<()> {
    let state = WebState { kv };
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router(state).into_make_service())
        .await?;

    Ok(())
}

#[derive(Clone)]
struct WebState {
    kv: Arc<Mutex<Store>>,
}

#[instrument(skip(state))]
async fn kv_get(State(state): State<WebState>, Path(key): Path<String>) -> impl IntoResponse {
    state
        .kv
        .lock()
        .unwrap()
        .get(&key)
        .map_err(handle_kv_error)
        .map(ToOwned::to_owned)
}

#[instrument(skip(state))]
async fn kv_post(
    State(state): State<WebState>,
    Path(key): Path<String>,
    value: String,
) -> impl IntoResponse {
    state
        .kv
        .lock()
        .unwrap()
        .insert(key, value)
        .map_err(handle_kv_error)
}

#[instrument(skip(state))]
async fn kv_put(
    State(state): State<WebState>,
    Path(key): Path<String>,
    value: String,
) -> impl IntoResponse {
    state
        .kv
        .lock()
        .unwrap()
        .update(&key, value)
        .map_err(handle_kv_error)
}

#[instrument(skip(state))]
async fn kv_delete(State(state): State<WebState>, Path(key): Path<String>) -> impl IntoResponse {
    state
        .kv
        .lock()
        .unwrap()
        .remove(&key)
        .map_err(handle_kv_error)
}

fn handle_kv_error(err: kv::Error) -> (StatusCode, String) {
    use kv::Error::*;

    match err {
        Exists => (StatusCode::CONFLICT, String::from("key exists")),
        DoesNotExist => (StatusCode::NOT_FOUND, String::from("key does not exist")),
    }
}
