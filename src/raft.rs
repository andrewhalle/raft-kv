use tonic::{Request, Response, Status};

use rpc::{
    raft_rpc_service_server::{RaftRpcService, RaftRpcServiceServer},
    AppendEntriesReply, AppendEntriesRequest,
};

#[allow(unused)]
enum State {
    Follower,
    Candidate,
    Leader,
}

mod rpc {
    tonic::include_proto!("raft");
}

struct Service;

#[tonic::async_trait]
impl RaftRpcService for Service {
    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesReply>, Status> {
        todo!()
    }
}
