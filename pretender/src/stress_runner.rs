#![allow(warnings)]
use contracts::{ProblemRequest, ProblemServiceRequest, UserCredentials, UserRequest};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::process::Child;
use std::thread;
use std::time::Duration;
use thiserror::Error;

use crate::client::Client;
use crate::problem_client::RemoteProblemClient;
use crate::user_client::RemoteUserClient;
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

impl StressRunner {
    pub async fn run(&self) -> Result<(), ServiceError> {
        let problem_client = RemoteProblemClient::default();
        let user_client = RemoteUserClient::default();

        let mut hasher = DefaultHasher::new();

        format!("password{}", 1).hash(&mut hasher);
        let user_req = UserRequest::Create(UserCredentials {
            name: format!("user{}", 1),
            hash: hasher.finish().to_string(),
        });

        let problem_req = ProblemServiceRequest::Problem(ProblemRequest::LargestWindowInArray);
        let _ = user_client.req(user_req).await;
        let _ = problem_client.req(problem_req).await;
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
impl StressRunner {
    pub async fn shutdown(&mut self) {
        let _ = self.problem_service.kill();
        let _ = self.problem_service.wait();

        let _ = self.solver_service.kill();
        let _ = self.solver_service.wait();

        let _ = self.user_service.kill();
        let _ = self.user_service.wait();
    }
}
