use crate::ServiceError;
use contracts::Client;
use contracts::problem::ProblemRequest;
use contracts::problem::ProblemResponse;
use contracts::problem::ValidationResponse;
use contracts::solver::SolveRequest;
use contracts::solver::SolveResponse;

use crate::solver_client::RemoteSolverClient;

pub async fn validate(
    problem: String,
    problem_type: ProblemRequest,
    client: &RemoteSolverClient,
) -> Result<i64, ServiceError> {
    match problem_type {
        ProblemRequest::LargestWindowInArray => window(problem, client).await,
        ProblemRequest::CountIslands => count_islands(problem, client).await,
        ProblemRequest::SizeOfIsland => size_of_island(problem, client).await,
        _ => todo!(),
    }
}

async fn window(problem: String, client: &RemoteSolverClient) -> Result<i64, ServiceError> {
    type Target = Vec<i64>;

    let solver_req = {
        let data = problem
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|v| {
                v.trim()
                    .parse::<i64>()
                    .map_err(|e| ServiceError::fault("solver", e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        SolveRequest::LargestWindowInArray { data }
    };
    solve(solver_req, client).await
}

async fn count_islands(problem: String, client: &RemoteSolverClient) -> Result<i64, ServiceError> {
    type Target = Vec<Vec<bool>>;
    let data: Target = problem
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split("],[")
        .map(|row| {
            row.trim_matches(&['[', ']'][..])
                .split(',')
                .map(|v| {
                    let v = v.trim();
                    match v {
                        "1" | "true" => Ok(true),
                        "0" | "false" => Ok(false),
                        _ => Err(ServiceError::fault("solver", format!("invalid bool: {v}"))),
                    }
                })
                .collect::<Result<Vec<bool>, _>>()
        })
        .collect::<Result<Vec<Vec<bool>>, _>>()?;
    solve(SolveRequest::CountIslands { data }, client).await
}

async fn size_of_island(problem: String, client: &RemoteSolverClient) -> Result<i64, ServiceError> {
    type Target = Vec<Vec<bool>>;

    let data: Target = problem
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split("],[")
        .map(|row| {
            row.trim_matches(&['[', ']'][..])
                .split(',')
                .map(|v| {
                    let v = v.trim();
                    match v {
                        "1" | "true" => Ok(true),
                        "0" | "false" => Ok(false),
                        _ => Err(ServiceError::fault("solver", format!("invalid bool: {v}"))),
                    }
                })
                .collect::<Result<Vec<bool>, _>>()
        })
        .collect::<Result<Vec<Vec<bool>>, _>>()?;
    solve(SolveRequest::SizeOfIsland { data }, client).await
}

async fn solve(solver_req: SolveRequest, client: &RemoteSolverClient) -> Result<i64, ServiceError> {
    match client.req(solver_req).await {
        Ok(SolveResponse::Solved(val)) => Ok(val),
        _ => return Err(ServiceError::fault("solver", "".into())),
    }
}
