use wincode::{SchemaRead, SchemaWrite};
fn host() -> String {
    std::env::var("PROBLEM_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}
fn port() -> String {
    std::env::var("PROBLEM_PORT").unwrap_or_else(|_| "4001".to_string())
}
pub fn url() -> String {
    format!("{}:{}", host(), port())
}

#[derive(Clone, Copy, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum ProblemRequest {
    LargestWindowInArray,
    CountIslands,
    TestProblem,
    SizeOfIsland,
    UnimplementedProblem,
}
#[derive(Clone, Copy, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub struct ValidationRequest {
    pub problem_id: i64,
    pub answer: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum ProblemServiceRequest {
    Problem(ProblemRequest),
    Validation(ValidationRequest),
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum ProblemServiceResponse {
    Problem(ProblemResponse),
    Validation(ValidationResponse),
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum ValidationResponse {
    Valid,
    Lower,
    Higher,
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub struct UserProblem {
    pub id: i64,
    pub data: String,
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum ProblemResponse {
    Ok(UserProblem),
    Fault(String),
}
