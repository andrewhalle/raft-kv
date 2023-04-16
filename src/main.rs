use std::sync::{Arc, Mutex};

use tracing::{error, info};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

use crate::kv::Store;

mod kv;
mod raft;
mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive("debug".parse().unwrap())
                .from_env_lossy(),
        )
        .with_span_events(FmtSpan::NEW)
        .init();
    info!("Initialized logging.");
    splash();

    // TODO: Set up raft state.
    let store = Arc::new(Mutex::new(Store::new()));
    // TODO: Join raft cluster.
    info!("Starting web server.");
    if let Err(e) = web::listen_client_requests(store).await {
        error!("A fatal error occurred: {e}");
    }
}

fn splash() {
    let text = r#"
  _____        __ _   _  __
 |  __ \      / _| | | |/ /
 | |__) |__ _| |_| |_| ' /_   __
 |  _  // _` |  _| __|  <\ \ / /
 | | \ \ (_| | | | |_| . \\ V /
 |_|  \_\__,_|_|  \__|_|\_\\_/
"#;
    println!("{}", &text[1..]);
}
