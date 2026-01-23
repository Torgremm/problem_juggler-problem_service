use anyhow::Result;
use async_trait::async_trait;
use contracts::solver::SolveRequest;
use contracts::solver::SolveResponse;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct RemoteSolverClient {
    addr: &'static str,
}

impl RemoteSolverClient {
    pub fn new(addr: &'static str) -> Self {
        Self { addr }
    }
}

impl Default for RemoteSolverClient {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:4000",
        }
    }
}
#[async_trait]
impl contracts::Client for RemoteSolverClient {
    type Req = SolveRequest;
    type Recv = SolveResponse;

    fn get_addr(&self) -> &str {
        self.addr
    }
}
