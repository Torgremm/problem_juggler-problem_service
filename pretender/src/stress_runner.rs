#![allow(warnings)]
use anyhow::Error as AnyError;
use contracts::Client;
use contracts::{
    problem::ProblemRequest, problem::ProblemResponse, problem::ProblemServiceRequest,
    problem::ProblemServiceResponse, problem::ValidationRequest, problem::ValidationResponse,
    solver::SolveRequest, solver::SolveResponse, user::UserCredentials, user::UserRequest,
};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::process::Child;
use std::thread;
use std::time::Duration;
use thiserror::Error;

use crate::parser::validate;
use crate::problem_client::RemoteProblemClient;
use crate::solver_client::RemoteSolverClient;
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
        let solver_client = RemoteSolverClient::default();

        let mut hasher = DefaultHasher::new();
        for n in 0..=10 {
            format!("password{}", n).hash(&mut hasher);
            let user_req = UserRequest::Create(UserCredentials {
                name: format!("user{}", n),
                hash: hasher.finish().to_string(),
            });

            let problem_req = ProblemServiceRequest::Problem(ProblemRequest::LargestWindowInArray);
            let _ = user_client
                .req(user_req)
                .await
                .map_err(|e| ServiceError::fault("user", e.to_string()))?;
            let resp = match problem_client
                .req(problem_req)
                .await
                .map_err(|e| ServiceError::fault("problem", e.to_string()))?
            {
                ProblemServiceResponse::Problem(ProblemResponse::Ok(p)) => p,
                ProblemServiceResponse::Problem(ProblemResponse::Fault(e)) => {
                    return Err(ServiceError::fault("problem", e));
                }
                _ => {
                    return Err(ServiceError::fault(
                        "problem",
                        "Problem service returned the wrong response".into(),
                    ));
                }
            };
            let sol = validate(
                resp.data,
                ProblemRequest::LargestWindowInArray,
                &solver_client,
            )
            .await?;

            let validation_req = ProblemServiceRequest::Validation(ValidationRequest {
                problem_id: resp.id,
                answer: sol,
            });

            let validation = match problem_client.req(validation_req).await {
                Ok(ProblemServiceResponse::Validation(ValidationResponse::Valid)) => {}
                _ => return Err(ServiceError::fault("solver", "".into())),
            };
        }

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
    #[error("service failed: '{service}'")]
    Fault {
        service: String,
        #[source]
        source: AnyError,
    },
}

impl ServiceError {
    pub fn fault(service: &str, message: String) -> Self {
        ServiceError::Fault {
            service: service.to_string(),
            source: anyhow::anyhow!(message),
        }
    }
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
    pub async fn panic_shutdown(&mut self, message: String) {
        self.shutdown().await;
        log::error!("Stress runner failed: {}", message);
        panic!()
    }
}
