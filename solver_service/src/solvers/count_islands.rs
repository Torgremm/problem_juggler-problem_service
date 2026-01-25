use contracts::solver::SolveResponse;
use std::collections::VecDeque;

type Grid = Vec<Vec<bool>>;
pub fn solve_count_islands(data: Grid) -> SolveResponse {
    let row_count = data.len();
    if row_count == 0 {
        return SolveResponse::BadData("Received empty grid".to_string());
    }
    let col_count = data[0].len();
    if row_count == 0 {
        return SolveResponse::BadData("Received empty grid".to_string());
    }

    let mut map = Map {
        row_c: row_count as isize,
        col_c: col_count as isize,
        grid: data,
    };
    let mut count = 0;

    for row in 0..row_count {
        for col in 0..col_count {
            if map.grid[row][col] {
                delete_island(row, col, &mut map);
                count += 1;
            }
        }
    }

    SolveResponse::Solved(count)
}

struct Map {
    row_c: isize,
    col_c: isize,
    grid: Grid,
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
type Point = (usize, usize);
trait Walk {
    fn check(&self, p: Point, dir: (isize, isize)) -> Option<Point>;
}
impl Walk for Map {
    fn check(&self, p: Point, dir: (isize, isize)) -> Option<Point> {
        let rc = p.0 as isize + dir.0;
        let cc = p.1 as isize + dir.1;
        if rc < 0 || rc >= self.row_c || cc < 0 || cc >= self.col_c {
            return None;
        }
        let rc = rc as usize;
        let cc = cc as usize;

        if !self.grid[rc][cc] {
            return None;
        }

        Some((rc, cc))
    }
}
fn delete_island(row: usize, col: usize, map: &mut Map) {
    let mut queue = VecDeque::new();
    queue.push_back((row, col));
    while let Some(p) = queue.pop_front() {
        for d in DIRECTIONS {
            if let Some(new_point) = map.check(p, d) {
                queue.push_back(new_point);
                map.grid[new_point.0][new_point.1] = false;
            }
        }

        map.grid[p.0][p.1] = false;
    }
}
