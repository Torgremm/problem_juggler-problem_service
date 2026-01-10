use crate::solvers::*;
use contracts::{SolveRequest, SolveResponse};

pub struct SolverService;
impl SolverService {
    pub async fn solve(req: SolveRequest) -> SolveResponse {
        log::debug!("received request for: {:?}", req);
        tokio::task::spawn_blocking(move || match_and_solve(req))
            .await
            .unwrap_or(SolveResponse::Fault)
    }
}

fn match_and_solve(req: SolveRequest) -> SolveResponse {
    match req {
        SolveRequest::LargestWindowInArray { data } => solve_largest_window_in_array(data),
        SolveRequest::TestProblem { data } => solve_test_problem(data),
        SolveRequest::SizeOfIsland { data } => solve_size_of_island(data),
        _ => {
            log::error!("Unimplemented problem request");
            SolveResponse::Fault
        }
    }
}
