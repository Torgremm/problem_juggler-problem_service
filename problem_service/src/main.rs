#![allow(dead_code)]
use contracts::ValidationRequest;
use contracts::ValidationResponse;
use std::sync::Arc;

use anyhow::Result;
use contracts::{ProblemRequest, ProblemResponse, ProblemServiceRequest};
use env_logger::Env;
use problem_service::ProblemService;
use std::sync::OnceLock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;

static SERVICE: OnceLock<ProblemService> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let service = ProblemService::default().await;
    let _ = SERVICE.set(service);

    let listener: TcpListener = TcpListener::bind("127.0.0.1:4001").await?;
    let sem = Arc::new(Semaphore::new(100));
    log::info!("ProblemService listening on 127.0.0.1:4001");

    loop {
        let permit = match sem.clone().acquire_owned().await {
            Ok(p) => p,
            Err(e) => {
                log::error!("Failed to aquire permit from semaphore: {}", e);
                continue;
            }
        };
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let _permit = permit;
            let mut len_buf = [0u8; 8];
            if socket.read_exact(&mut len_buf).await.is_err() {
                return;
            }

            let len = u64::from_be_bytes(len_buf);
            let mut buf = vec![0u8; len as usize];
            if socket.read_exact(&mut buf).await.is_err() {
                return;
            }
            let _ = match wincode::deserialize(&buf) {
                Ok(r) => match_request(r, &mut socket),
                Err(_) => {
                    log::error!("Failed to serialize a request");
                    write_response(ProblemResponse::Fault, &mut socket).await;
                    return;
                }
            }
            .await;
        });
    }
}

async fn write_response(response: ProblemResponse, socket: &mut TcpStream) {
    let resp = wincode::serialize(&response).unwrap();
    if let Err(e) = socket.write_all(&(resp.len() as u64).to_be_bytes()).await {
        log::error!("Failed to write response length: {}", e);
        return;
    }
    if let Err(e) = socket.write_all(&resp).await {
        log::error!("Failed to write response: {}", e);
    }
}
async fn write_validation(response: ValidationResponse, socket: &mut TcpStream) {
    let resp = wincode::serialize(&response).unwrap();
    if let Err(e) = socket.write_all(&(resp.len() as u64).to_be_bytes()).await {
        log::error!("Failed to write response length: {}", e);
        return;
    }
    if let Err(e) = socket.write_all(&resp).await {
        log::error!("Failed to write response: {}", e);
    }
}

async fn match_request(req: ProblemServiceRequest, socket: &mut TcpStream) {
    match req {
        ProblemServiceRequest::Problem(r) => handle_problem_request(r, socket).await,
        ProblemServiceRequest::Validation(r) => handle_validation_request(r, socket).await,
    }
}

async fn handle_problem_request(req: ProblemRequest, socket: &mut TcpStream) {
    let resp = match SERVICE
        .get()
        .expect("Catastrophic failure, service gone")
        .get_dispatch(req)
        .await
    {
        Ok(r) => r.to_response(),
        Err(_) => {
            log::error!("Failed to generate problem");
            write_response(ProblemResponse::Fault, socket).await;
            return;
        }
    };

    write_response(resp, socket).await;
}
async fn handle_validation_request(req: ValidationRequest, socket: &mut TcpStream) {
    let resp = match SERVICE
        .get()
        .expect("Catastrophic failure, service gone")
        .validate(req.problem_id, req.answer)
        .await
    {
        Ok(r) => r,
        Err(_) => {
            log::error!("Failed to generate problem");
            write_response(ProblemResponse::Fault, socket).await;
            return;
        }
    };
    write_validation(resp, socket).await;
}
