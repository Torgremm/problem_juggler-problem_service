#![allow(warnings)]
use std::sync::Arc;

use crate::solver_service::SolverService;
use anyhow::Result;
use contracts::Listener;
use contracts::solver::{SolveRequest, SolveResponse};
use env_logger::Env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
mod solver_service;
mod solvers;

struct SolverListener {
    addr: &'static str,
}

impl contracts::Listener for SolverListener {
    type Recv = SolveRequest;
    fn get_addr(&self) -> &str {
        self.addr
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let listener = SolverListener {
        addr: "127.0.0.1:4000",
    };
    listener.listen(handle_request).await?;
    Ok(())
}

async fn handle_request(req: SolveRequest, mut socket: TcpStream) {
    let sol = SolverService::solve(req).await;
    write_response(sol, &mut socket).await;
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
