use std::sync::{Arc, Mutex};

use tracing::error;

use crate::kv::Store;

mod kv;
mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // 1. Set up raft state.
    let store = Arc::new(Mutex::new(Store::new()));

    // 2. Join raft cluster.
    if let Err(e) = web::listen_client_requests(store).await {
        error!("A fatal error occurred: {e}");
    }
}
