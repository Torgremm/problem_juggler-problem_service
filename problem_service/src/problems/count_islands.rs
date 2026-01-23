use contracts::solver::SolveRequest;
use rand::Rng;
use std::collections::{HashSet, VecDeque};

use crate::problems::problem_kind::Problem;

pub struct CountIslands;

impl Problem for CountIslands {
    type Data = Vec<Vec<bool>>;
    fn create() -> Self::Data {
        let mut rng = rand::rng();
        let count: u8 = rng.random_range(50..=150);
        let size: usize = count as usize * rng.random_range(2..=4);

        create_islands(count, size)
    }

    fn into_request(data: Self::Data) -> SolveRequest {
        SolveRequest::CountIslands { data }
    }
}

fn create_islands(island_count: u8, size: usize) -> Vec<Vec<bool>> {
    let mut rng = rand::rng();
    let mut grid = vec![vec![false; size]; size];
    let mut seeds = HashSet::new();
    let min_distance = 2; // minimum spacing between islands

    while seeds.len() < island_count.into() {
        let row = rng.random_range(0..size);
        let col = rng.random_range(0..size);

        let too_close = seeds.iter().any(|&(r, c)| {
            let dr = r as isize - row as isize;
            let dc = c as isize - col as isize;
            dr.abs() <= min_distance && dc.abs() <= min_distance
        });

        if !too_close {
            seeds.insert((row, col));
            grid[row][col] = true;
        }
    }
    for seed in seeds {
        add_island_bfs(&mut grid, (seed.0, seed.1));
    }

    grid
}

fn add_island_bfs(grid: &mut [Vec<bool>], pos: (usize, usize)) {
    let mut rng = rand::rng();
    let cols = grid[0].len();
    let rows = grid.len();
    let size: usize = rng.random_range(1..=30);
    let mut queue = VecDeque::new();

    let mut island = HashSet::new();

    queue.push_back(pos);

    'bigloop: while let Some((r, c)) = queue.pop_front() {
        if island.len() == size {
            break;
        }
        if island.contains(&(r, c)) {
            continue;
        }
        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        let mut surroundings = Vec::new();
        for direction in directions {
            let cr = r as isize + direction.0;
            let cc = c as isize + direction.1;

            if cr >= 0 && cr < rows as isize && cc >= 0 && cc < cols as isize {
                if !grid[cr as usize][cc as usize] && rng.random_bool(0.7) {
                    surroundings.push((cr as usize, cc as usize));
                } else if grid[cr as usize][cc as usize]
                    && !island.contains(&(cr as usize, cc as usize))
                {
                    continue 'bigloop;
                }
            }
        }
        queue.extend(surroundings);
        grid[r][c] = true;
        island.insert((r, c));
    }
}
