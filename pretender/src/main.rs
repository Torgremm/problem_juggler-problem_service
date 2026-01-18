mod client;
mod problem_client;
mod stress_runner;
mod user_client;
use std::path::PathBuf;
use std::process::{Child, Command};

use env_logger::Env;

use crate::stress_runner::StressRunner;
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let current = std::env::current_dir().expect("Failed to get current directory");
    let parent = current.parent().expect("Failed to find patent");
    let user_service_dir = parent.join("user_service");
    let solver_service_dir = parent.join("solver_service");
    let problem_service_dir = parent.join("provlem_service");

    let user_service = start_service(user_service_dir).expect("Failed to start user service");
    let solver_service = start_service(solver_service_dir).expect("Failed to start solver service");
    let problem_service =
        start_service(problem_service_dir).expect("Failed to start problem service");

    let runner = StressRunner::new(user_service, solver_service, problem_service);
    if let Err(e) = runner.run() {
        log::error!("{}", e);
    };
}

fn start_service(dir: PathBuf) -> Result<Child, std::io::Error> {
    Command::new("cargo").arg("run").current_dir(&dir).spawn()
}
