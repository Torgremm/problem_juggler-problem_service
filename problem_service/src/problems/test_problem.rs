use rand::Rng;

use crate::problems::problem_kind::Problem;
use contracts::solver::SolveRequest;

pub struct TestProblem;

impl Problem for TestProblem {
    type Data = String;

    fn create() -> Self::Data {
        let mut rng = rand::rng();
        let len = rng.random_range(10..20);
        std::iter::repeat('A').take(len).collect()
    }

    fn into_request(data: Self::Data) -> SolveRequest {
        SolveRequest::TestProblem { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_size_is_valid() {
        let data = SizeOfIsland::create();

        assert!((10..=20).contains(&data.len()))
    }
}
