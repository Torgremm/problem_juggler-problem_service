#![allow(warnings)]
mod parser;
mod problem_client;
mod solver_client;
mod stress_runner;
mod user_client;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use tokio::net::TcpStream;
use tokio::time::{Duration, Instant, sleep};

use env_logger::Env;

use crate::stress_runner::{ServiceError, StressRunner};
#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let cur = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let parent = cur.parent().unwrap();
    let target = parent.join("target").join("debug");
    let user_service_dir = target.join(bin_name("user_service"));
    let solver_service_dir = target.join(bin_name("solver_service"));
    let problem_service_dir = target.join(bin_name("problem_service"));

    let user_service = start_and_wait_service(
        &user_service_dir,
        &contracts::user::url(),
        Duration::from_secs(5),
    )
    .await?;
    let solver_service = start_and_wait_service(
        &solver_service_dir,
        &contracts::solver::url(),
        Duration::from_secs(5),
    )
    .await?;
    let problem_service = start_and_wait_service(
        &problem_service_dir,
        &contracts::solver::url(),
        Duration::from_secs(5),
    )
    .await?;

    let mut runner = StressRunner::new(user_service, solver_service, problem_service);

    match runner.run().await {
        Ok(()) => {}
        Err(e) => {
            runner.shutdown().await;
            return Err(e);
        }
    }

    runner.shutdown().await;
    Ok(())
}

fn start_service(bin_path: PathBuf) -> Result<Child, std::io::Error> {
    Command::new(bin_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
}
fn bin_name(name: &str) -> String {
    if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}
pub async fn start_and_wait_service(
    bin_path: &std::path::Path,
    addr: &str,
    timeout: Duration,
) -> Result<Child, ServiceError> {
    let mut child = Command::new(bin_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| ServiceError::fault("service", format!("Failed to spawn: {}", e)))?;

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return Err(ServiceError::fault(
                    "service",
                    format!("Service exited immediately with status: {}", status),
                ));
            }
            Ok(None) => {}
            Err(e) => {
                return Err(ServiceError::fault(
                    "service",
                    format!("Failed to check child status: {}", e),
                ));
            }
        }

        if TcpStream::connect(addr).await.is_ok() {
            break;
        }

        if start.elapsed() > timeout {
            return Err(ServiceError::fault(
                "service",
                format!("Service did not start listening within {:?}", timeout),
            ));
        }

        sleep(Duration::from_millis(50)).await;
    }

    Ok(child)
}
