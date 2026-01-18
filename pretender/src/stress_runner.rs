use std::process::Child;
use std::thread;
use std::time::Duration;
use thiserror::Error;
pub struct StressRunner {
    user_service: Child,
    solver_service: Child,
    problem_service: Child,
}

impl StressRunner {
    pub fn new(user_service: Child, solver_service: Child, problem_service: Child) -> Self {
        Self {
            user_service,
            solver_service,
            problem_service,
        }
    }
}

impl Drop for StressRunner {
    fn drop(&mut self) {
        let _ = self.problem_service.kill();
        let _ = self.problem_service.wait();
        let _ = self.solver_service.kill();
        let _ = self.solver_service.wait();
        let _ = self.user_service.kill();
        let _ = self.user_service.wait();
    }
}

impl StressRunner {
    pub fn run(&self) -> Result<(), ServiceError> {
        Ok(())
    }
}
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("failed to start service in '{dir}'")]
    Start {
        dir: String,
        #[source]
        source: std::io::Error,
    },
}
