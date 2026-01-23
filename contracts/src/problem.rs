use wincode::{SchemaRead, SchemaWrite};

#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum ProblemRequest {
    LargestWindowInArray,
    CountIslands,
    TestProblem,
    SizeOfIsland,
    UnimplementedProblem,
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub struct ValidationRequest {
    pub problem_id: i64,
    pub answer: i64,
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
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
