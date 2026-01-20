#![allow(warnings)]
mod client;
mod problem_client;
mod solver_client;
mod stress_runner;
mod user_client;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

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

    let user_service = start_service(user_service_dir).expect("Failed to start user service");
    let solver_service = start_service(solver_service_dir).expect("Failed to start solver service");
    let problem_service =
        start_service(problem_service_dir).expect("Failed to start problem service");

    let mut runner = StressRunner::new(user_service, solver_service, problem_service);

    let _res = runner.run().await?;

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
