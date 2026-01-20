#![allow(warnings)]
use contracts::{
    ProblemRequest, ProblemResponse, ProblemServiceRequest, ProblemServiceResponse, SolveRequest,
    SolveResponse, UserCredentials, UserRequest, ValidationRequest,
};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::process::Child;
use std::thread;
use std::time::Duration;
use thiserror::Error;

use crate::client::Client;
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
        for n in 0..=1 {
            format!("password{}", n).hash(&mut hasher);
            let user_req = UserRequest::Create(UserCredentials {
                name: format!("user{}", n),
                hash: hasher.finish().to_string(),
            });

            let problem_req = ProblemServiceRequest::Problem(ProblemRequest::LargestWindowInArray);
            let _ = user_client
                .req(user_req)
                .await
                .expect("user service failed");
            let resp = match problem_client
                .req(problem_req)
                .await
                .expect("problem service failed")
            {
                ProblemServiceResponse::Problem(ProblemResponse::Ok(p)) => p,
                ProblemServiceResponse::Problem(ProblemResponse::Fault) => {
                    unreachable!("problem service returned fault")
                }
                _ => unreachable!("problem service returned the wrong response"),
            };
            let solver_req = SolveRequest::LargestWindowInArray {
                data: resp
                    .data
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<i64>>(),
            };

            let solution = match solver_client.req(solver_req).await {
                Ok(SolveResponse::Solved(val)) => val,
                _ => unreachable!("Solver service failed a request"),
            };

            problem_client.req(ProblemServiceRequest::Validation(ValidationRequest {
                problem_id: resp.id,
                answer: solution,
            }));
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
