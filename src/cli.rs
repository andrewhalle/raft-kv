use std::net::SocketAddr;

use clap::Parser;

use crate::raft::State;

/// The CLI for a raft node.
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub(super) struct Args {
    #[arg(long, value_enum)]
    pub(super) to_remove_raft_state: State,

    /// The peers for this node.
    #[arg(
        long = "peer",
        value_name = "PEER",
        help = "A peer for this node. Can be specified multiple times."
    )]
    pub(super) peers: Vec<SocketAddr>,

    /// The address for this node to bind to.
    #[arg(long)]
    pub(super) addr: SocketAddr,

    /// The HTTP address for this node to bind to.
    #[arg(long)]
    pub(super) http_addr: SocketAddr,
}
