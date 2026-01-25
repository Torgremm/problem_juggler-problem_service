use anyhow::Result;
use contracts::solver::SolveRequest;
use contracts::solver::SolveResponse;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct RemoteSolverClient {
    addr: String,
}

impl Default for RemoteSolverClient {
    fn default() -> Self {
        Self {
            addr: contracts::solver::url(),
        }
    }
}
impl contracts::Client for RemoteSolverClient {
    type Req = SolveRequest;
    type Recv = SolveResponse;

    fn get_addr(&self) -> String {
        self.addr.clone()
    }
}
