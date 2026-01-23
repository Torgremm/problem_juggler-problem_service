use contracts::solver::SolveResponse;

pub fn solve_test_problem(data: String) -> SolveResponse {
    log::info!("Received test problem with: {}", data);
    SolveResponse::Solved(data.len() as i64)
}
