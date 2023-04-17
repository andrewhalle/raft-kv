#[allow(unused)]
enum State {
    Follower,
    Candidate,
    Leader,
}

mod rpc {
    tonic::include_proto!("raft");
}
