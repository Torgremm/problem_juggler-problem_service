use contracts::solver::SolveRequest;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum ProblemKind {
    LargestWindowInArray,
    CountIslands,
    SizeOfIsland,
    TestProblem,
}

pub trait Problem {
    type Data: DBColumn;

    fn create() -> Self::Data;
    fn into_request(data: Self::Data) -> SolveRequest;
}

pub trait DBColumn {
    fn to_db_entry(&self) -> String;
}

macro_rules! impl_db_column {
    ($($t:ty),* $(,)?) => {
        $(
            impl DBColumn for $t {
                fn to_db_entry(&self) -> String {
                    self.to_string()
                }
            }
        )*

    };
}

impl_db_column!(i64, usize, bool, char, String, &str);

impl<T> DBColumn for Vec<T>
where
    T: DBColumn,
{
    fn to_db_entry(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(|v| v.to_db_entry())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
