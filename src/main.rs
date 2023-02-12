use tracing::error;

mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // 1. Set up raft state.
    // 2. Join raft cluster.
    if let Err(e) = web::listen_client_requests().await {
        error!("A fatal error occurred: {e}");
    }
}
