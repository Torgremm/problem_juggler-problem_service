#![allow(warnings)]
pub mod interface;
pub mod problem_handler;
pub mod problem_service;
pub mod problems;

pub use interface::solver::RemoteSolverClient;
pub use problem_handler::ProblemRepository;
pub use problem_service::ProblemService;

#[cfg(feature = "test-utils")]
pub mod test_utils {

    use super::*;
    pub async fn test_service() -> ProblemService {
        let repo = ProblemRepository::test_object().await;
        let solve_client = RemoteSolverClient::default();
        ProblemService::new(repo, solve_client)
    }
}
