use std::collections::{HashSet, VecDeque};

use rand::Rng;

use crate::problems::problem_kind::Problem;
use contracts::solver::SolveRequest;

pub struct SizeOfIsland;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
impl Problem for SizeOfIsland {
    type Data = Vec<Vec<bool>>;
    fn create() -> Self::Data {
        let mut rng = rand::rng();

        let row = rng.random_range(0..20);
        let col = rng.random_range(0..20);
        let size = rng.random_range(20..=80);

        let mut count = 0;
        let mut grid = vec![vec![false; 20]; 20];
        let mut queue = VecDeque::new();

        queue.push_back((row, col));
        let mut unique = HashSet::new();
        while let Some((r, c)) = queue.pop_front() {
            if !unique.insert((r, c)) {
                continue;
            }

            for dir in DIRECTIONS {
                let rn = r + dir.0;
                let cn = c + dir.1;

                if (0..20).contains(&rn)
                    && (0..20).contains(&cn)
                    && rng.random_bool(0.5)
                    && !grid[rn as usize][cn as usize]
                {
                    queue.push_back((rn, cn));
                }
            }
            grid[r as usize][c as usize] = true;
            count += 1;
            if count == size {
                break;
            }
        }
        grid
    }

    fn into_request(data: Self::Data) -> SolveRequest {
        SolveRequest::SizeOfIsland { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_size_is_valid() {
        let data = SizeOfIsland::create();
        let mut total = 0;
        for row in data {
            for cell in row {
                if cell {
                    total += 1;
                }
            }
        }

        assert!((0..=80).contains(&total))
    }
}
