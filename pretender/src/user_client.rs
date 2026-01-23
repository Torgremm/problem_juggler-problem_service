use contracts::user::UserRequest;
use contracts::user::UserResponse;

use crate::client::Client;

pub struct RemoteUserClient {
    addr: &'static str,
}

impl RemoteUserClient {
    pub fn new(addr: &'static str) -> Self {
        Self { addr }
    }
}

impl Default for RemoteUserClient {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:4002",
        }
    }
}

impl Client for RemoteUserClient {
    type Req = UserRequest;
    type Recv = UserRequest;
    fn get_addr(&self) -> &str {
        self.addr
    }
}
