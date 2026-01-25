#![allow(warnings)]
use std::sync::Arc;

use anyhow::Result;
use contracts::Listener;
use contracts::user::{UserRequest, UserResponse};
use env_logger::Env;
use std::sync::OnceLock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
use user_service::user_service::UserService;

static SERVICE: OnceLock<UserService> = OnceLock::new();
struct UserListener {
    addr: &'static str,
}

impl contracts::Listener for UserListener {
    type Recv = UserRequest;
    fn get_addr(&self) -> &str {
        self.addr
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let service = UserService::default().await;
    let _ = SERVICE.set(service);
    let listener = UserListener {
        addr: "127.0.0.1:4002",
    };
    listener.listen(handle_request).await?;
    Ok(())
}
async fn handle_request(request: UserRequest, mut socket: TcpStream) {
    let resp = match request {
        UserRequest::Login(u) => SERVICE.get().unwrap().login(u).await,
        UserRequest::Create(u) => SERVICE.get().unwrap().create_user(u).await,
    };
    write_response(resp, &mut socket).await;
}

async fn write_response(response: UserResponse, socket: &mut TcpStream) {
    let resp = wincode::serialize(&response).unwrap();
    if let Err(e) = socket.write_all(&(resp.len() as u64).to_be_bytes()).await {
        log::error!("Failed to write response length: {}", e);
        return;
    }
    if let Err(e) = socket.write_all(&resp).await {
        log::error!("Failed to write response: {}", e);
    }
}
