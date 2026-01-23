use contracts::solver::SolveRequest;
use contracts::solver::SolveResponse;

use crate::client::Client;

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

impl Client for RemoteSolverClient {
    type Req = SolveRequest;
    type Recv = SolveResponse;
    fn get_addr(&self) -> &str {
        self.addr
    }
}
