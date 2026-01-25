use wincode::{SchemaRead, SchemaWrite};
fn host() -> String {
    std::env::var("SOLVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}
fn port() -> String {
    std::env::var("SOLVER_PORT").unwrap_or_else(|_| "4000".to_string())
}

pub fn url() -> String {
    format!("{}:{}", host(), port())
}

#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum SolveResponse {
    Solved(i64),
    Fault,
    BadData(String),
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum SolveRequest {
    LargestWindowInArray { data: Vec<i64> },
    TestProblem { data: String },
    SizeOfIsland { data: Vec<Vec<bool>> },
    CountIslands { data: Vec<Vec<bool>> },
    UnimplementedProblem { data: String },
}
