use wincode::{SchemaRead, SchemaWrite};

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
    Fault,
}

#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub struct User {
    pub name: String,
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub struct UserCredentials {
    pub name: String,
    pub hash: String,
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum UserRequest {
    Login(UserCredentials),
    Create(UserCredentials),
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum UserResponse {
    Valid(User),
    Invalid,
    Fault(String),
}
