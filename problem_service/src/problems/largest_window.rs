use rand::Rng;

use crate::problems::problem_kind::Problem;
use contracts::SolveRequest;

pub struct LargestWindow;
impl Problem for LargestWindow {
    type Data = Vec<i64>;
    fn create() -> Vec<i64> {
        let mut rng = rand::rng();

        let mut data = Vec::new();
        for _ in 0..20 {
            data.push(rng.random_range(-20..20));
        }
        data
    }

    fn into_request(data: Self::Data) -> SolveRequest {
        SolveRequest::LargestWindowInArray { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_size_and_range_is_valid() {
        let data = LargestWindow::create();
        assert!(*data.iter().max().unwrap() < 21);
        assert!(*data.iter().min().unwrap() > -21);
        assert!(data.len() == 20);
    }
}
