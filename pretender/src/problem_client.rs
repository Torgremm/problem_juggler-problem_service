use contracts::Client;
use contracts::problem::ProblemServiceRequest;
use contracts::problem::ProblemServiceResponse;

pub struct RemoteProblemClient {
    addr: String,
}

impl Default for RemoteProblemClient {
    fn default() -> Self {
        Self {
            addr: contracts::problem::url(),
        }
    }
}

impl contracts::Client for RemoteProblemClient {
    type Req = ProblemServiceRequest;
    type Recv = ProblemServiceResponse;
    fn get_addr(&self) -> String {
        self.addr.clone()
    }
}
