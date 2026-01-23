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
