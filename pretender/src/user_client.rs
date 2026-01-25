use contracts::user::UserRequest;
use contracts::user::UserResponse;

pub struct RemoteUserClient {
    addr: String,
}

impl Default for RemoteUserClient {
    fn default() -> Self {
        Self {
            addr: contracts::user::url(),
        }
    }
}

impl contracts::Client for RemoteUserClient {
    type Req = UserRequest;
    type Recv = UserRequest;
    fn get_addr(&self) -> String {
        self.addr.clone()
    }
}
