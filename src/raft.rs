#![allow(unused)]

use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{anyhow, Error, Result};
use clap::{builder::PossibleValue, ValueEnum};
use futures::{stream, StreamExt};
use tonic::{
    transport::{Server, Uri},
    Request, Response, Status,
};
use tracing::{debug, error, info, warn};

use rpc::{
    raft_rpc_service_client::RaftRpcServiceClient,
    raft_rpc_service_server::{RaftRpcService, RaftRpcServiceServer},
    AppendEntriesReply, AppendEntriesRequest, Entry,
};

#[derive(Debug, Default, Copy, Clone)]
pub(super) enum State {
    #[default]
    Follower,
    Candidate,
    Leader,
}

impl ValueEnum for State {
    fn value_variants<'a>() -> &'a [Self] {
        &[State::Follower, State::Candidate, State::Leader]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            State::Follower => Some(PossibleValue::new("follower")),
            State::Candidate => Some(PossibleValue::new("candidate")),
            State::Leader => Some(PossibleValue::new("leader")),
        }
    }
}

mod rpc {
    tonic::include_proto!("raft");
}

#[derive(Debug, Default)]
struct Service {
    raft_state: Arc<Mutex<RaftState>>,
}

#[tonic::async_trait]
impl RaftRpcService for Service {
    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesReply>, Status> {
        info!("Responding to append_entries()");
        let state = self.raft_state.lock().unwrap();
        // let request = request.into_inner();
        // if request.term < state.current_term {
        //     return Ok(Response::new(AppendEntriesReply {
        //         term: state.current_term,
        //         success: false,
        //     }));
        // }

        // TODO: Reply false if log doesn’t contain an entry at prevLogIndex
        // whose term matches prevLogTerm

        // TODO: If an existing entry conflicts with a new one (same index
        // but different terms), delete the existing entry and all that
        // follow it

        // TODO: Append any new entries not already in the log

        // TODO: If leaderCommit > commitIndex, set commitIndex =
        // min(leaderCommit, index of last new entry)

        Ok(Response::new(AppendEntriesReply {
            term: state.current_term,
            success: true,
        }))
    }
}

#[derive(Debug, Default)]
struct RaftState {
    /// TODO: This needs to be persistent.
    current_term: u64,
    /// TODO: This needs to be persistent.
    voted_for: Option<u64>,
    /// TODO: This needs to be persistent.
    log: Vec<LogEntry>,
    state: State,
    commit_index: u64,
    last_applied: u64,
}

#[derive(Debug)]
struct LogEntry {
    term: u64,
    entry: Entry,
}

/// Main loop for raft consensus.
///
/// **Note**
/// `state` is provided only temporarily, and should be removed in a later version of this
/// function.
pub(super) async fn run_raft(addr: SocketAddr, peers: Vec<SocketAddr>, state: State) -> Result<()> {
    tracing::info!("Starting raft");
    match state {
        State::Leader => run_raft_leader(peers).await,
        State::Follower => run_raft_follower(addr).await,
        _ => todo!(),
    }
}

/// The main loop for the raft leader.
async fn run_raft_leader(peers: Vec<SocketAddr>) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut clients = Vec::new();
    for peer in peers {
        let uri = Uri::builder()
            .authority(peer.to_string())
            .scheme("http")
            .path_and_query("/")
            .build()?;
        let client = RaftRpcServiceClient::connect(uri).await?;
        clients.push(client);
    }
    loop {
        debug!("Sending heartbeats");
        interval.tick().await;
        let responses = stream::iter(&mut clients)
            .for_each_concurrent(None, |client| async move {
                debug!("Sending heartbeat to client");
                let request = Request::new(AppendEntriesRequest {
                    term: 0,
                    leader_id: 0,
                    prev_log_index: 0,
                    prev_log_term: 0,
                    entries: Vec::new(),
                    leader_commit: 0,
                });
                match client.append_entries(request).await {
                    Ok(_) => {
                        info!("got a heartbeat response");
                    }
                    Err(e) => {
                        error!("failed to send heartbeat: {e}");
                    }
                }
            })
            .await;
    }

    Ok(())
}

/// The main loop for the raft followers.
async fn run_raft_follower(addr: SocketAddr) -> Result<()> {
    let service = Service::default();

    Server::builder()
        .add_service(RaftRpcServiceServer::new(service))
        .serve(addr)
        .await;

    Ok(())
}
