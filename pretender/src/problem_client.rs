use contracts::problem::ProblemServiceRequest;
use contracts::problem::ProblemServiceResponse;

use crate::client::Client;

pub struct RemoteProblemClient {
    addr: &'static str,
}

impl RemoteProblemClient {
    pub fn new(addr: &'static str) -> Self {
        Self { addr }
    }
}

impl Default for RemoteProblemClient {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:4001",
        }
    }
}

impl Client for RemoteProblemClient {
    type Req = ProblemServiceRequest;
    type Recv = ProblemServiceResponse;
    fn get_addr(&self) -> &str {
        self.addr
    }
}
