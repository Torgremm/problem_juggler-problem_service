#![allow(warnings)]
use contracts::problem::ProblemServiceResponse;
use contracts::problem::ValidationRequest;
use contracts::problem::ValidationResponse;
use contracts::Listener;
use futures::future::BoxFuture;
use std::sync::Arc;

use anyhow::Result;
use contracts::problem::{ProblemRequest, ProblemResponse, ProblemServiceRequest};
use env_logger::Env;
use problem_service::ProblemService;
use std::sync::OnceLock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;

static SERVICE: OnceLock<ProblemService> = OnceLock::new();

struct ProblemListener {
    addr: &'static str,
}

impl contracts::Listener for ProblemListener {
    type Recv = ProblemServiceRequest;
    fn get_addr(&self) -> &str {
        self.addr
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let service = ProblemService::default().await;
    let _ = SERVICE.set(service);

    let listener = ProblemListener {
        addr: "127.0.0.1:4001",
    };

    listener.listen(Arc::new(match_request)).await?;
    Ok(())
}
async fn write_response(response: ProblemServiceResponse, mut socket: TcpStream) -> Result<()> {
    let resp = wincode::serialize(&response)?;
    if let Err(e) = socket.write_all(&(resp.len() as u64).to_be_bytes()).await {
        log::error!("Failed to write response length: {}", e);
        return Err(e.into());
    }
    if let Err(e) = socket.write_all(&resp).await {
        log::error!("Failed to write response: {}", e);
        return Err(e.into());
    }
    Ok(())
}
async fn write_problem(response: ProblemResponse, mut socket: TcpStream) {
    write_response(ProblemServiceResponse::Problem(response), socket).await;
}
async fn write_validation(response: ValidationResponse, mut socket: TcpStream) {
    write_response(ProblemServiceResponse::Validation(response), socket).await;
}

fn match_request(req: ProblemServiceRequest, mut socket: TcpStream) -> BoxFuture<'static, ()> {
    Box::pin(async move {
        match req {
            ProblemServiceRequest::Problem(r) => handle_problem_request(r, socket).await,
            ProblemServiceRequest::Validation(r) => handle_validation_request(r, socket).await,
        }
    })
}

async fn handle_problem_request(req: ProblemRequest, mut socket: TcpStream) {
    let resp = match SERVICE
        .get()
        .expect("Catastrophic failure, service gone")
        .get_dispatch(req)
        .await
    {
        Ok(r) => r.to_response(),
        Err(e) => {
            log::error!("Failed to generate problem: {}", e);
            write_problem(ProblemResponse::Fault(e.to_string()), socket).await;
            return;
        }
    };

    write_problem(resp, socket).await;
}
async fn handle_validation_request(req: ValidationRequest, mut socket: TcpStream) {
    let resp = match SERVICE
        .get()
        .expect("Catastrophic failure, service gone")
        .validate(req.problem_id, req.answer)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to validate problem: {}", e);
            write_problem(ProblemResponse::Fault(e.to_string()), socket).await;
            return;
        }
    };
    write_validation(resp, socket).await;
}
