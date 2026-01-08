#![allow(dead_code)]
use anyhow::Result;
use contracts::{SolveRequest, SolveResponse};
use env_logger::Env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::solver_service::SolverService;
mod solver_service;
mod solvers;
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4000").await?;
    log::info!("SolverService listening on 127.0.0.1:4000");
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut len_buf = [0u8; 8];
            if socket.read_exact(&mut len_buf).await.is_err() {
                return;
            }
            let len = u64::from_be_bytes(len_buf);
            let mut buf = vec![0u8; len as usize];
            if socket.read_exact(&mut buf).await.is_err() {
                return;
            }

            let req: SolveRequest = match wincode::deserialize(&buf) {
                Ok(r) => r,
                Err(_) => {
                    log::error!("Failed to serialize a request");
                    write_response(SolveResponse::Fault, &mut socket).await;
                    return;
                }
            };

            let resp = SolverService::solve(req).await;
            write_response(resp, &mut socket).await;
        });
    }
}

async fn write_response(response: SolveResponse, socket: &mut TcpStream) {
    let resp = wincode::serialize(&response).unwrap();
    if let Err(e) = socket.write_all(&(resp.len() as u64).to_be_bytes()).await {
        log::error!("Failed to write response length: {}", e);
        return;
    }
    if let Err(e) = socket.write_all(&resp).await {
        log::error!("Failed to write response: {}", e);
    }
}
