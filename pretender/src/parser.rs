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
    let data = parse_vector(problem)?;
    solve(SolveRequest::LargestWindowInArray { data }, client).await
}

async fn count_islands(problem: String, client: &RemoteSolverClient) -> Result<i64, ServiceError> {
    type Target = Vec<Vec<bool>>;
    let data = parse_bool_grid(problem)?;

    solve(SolveRequest::CountIslands { data }, client).await
}

async fn size_of_island(problem: String, client: &RemoteSolverClient) -> Result<i64, ServiceError> {
    type Target = Vec<Vec<bool>>;
    let data = parse_bool_grid(problem)?;

    solve(SolveRequest::SizeOfIsland { data }, client).await
}

fn parse_vector(input: String) -> Result<Vec<i64>, ServiceError> {
    input
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|v| {
            v.trim()
                .parse::<i64>()
                .map_err(|e| ServiceError::fault("solver", e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()
}

fn parse_bool_grid(input: String) -> Result<Vec<Vec<bool>>, ServiceError> {
    input
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split("], [")
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
        .collect::<Result<Vec<Vec<bool>>, _>>()
}

async fn solve(solver_req: SolveRequest, client: &RemoteSolverClient) -> Result<i64, ServiceError> {
    match client.req(solver_req).await {
        Ok(SolveResponse::Solved(val)) => Ok(val),
        _ => return Err(ServiceError::fault("solver", "".into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn we_can_parse_bool_grids() {
        let vector_string = "[[1,0,1,0,1,0,1,0,1,0,1,0], [1,0,1,0,1,0,1,0,1,0,1,0], [1,0,1,0,1,0,1,0,1,0,1,0], [1,0,1,0,1,0,1,0,1,0,1,0]]".to_string();
        let vector = parse_bool_grid(vector_string).expect("Failed to parse to Vec<Vec<bool>>");

        let should_be = vec![
            vec![
                true, false, true, false, true, false, true, false, true, false, true, false,
            ],
            vec![
                true, false, true, false, true, false, true, false, true, false, true, false,
            ],
            vec![
                true, false, true, false, true, false, true, false, true, false, true, false,
            ],
            vec![
                true, false, true, false, true, false, true, false, true, false, true, false,
            ],
        ];
        assert_eq!(should_be, vector);
    }
    #[test]
    fn we_can_parse_vectors() {
        let vector_string = "[7,6,1,-2,18,-17,19,-22,22,5]".to_string();
        let vector = parse_vector(vector_string).expect("Failed to parse to Vec<i64>");
        let should_be = vec![7, 6, 1, -2, 18, -17, 19, -22, 22, 5];
        assert_eq!(should_be, vector);
    }
}
