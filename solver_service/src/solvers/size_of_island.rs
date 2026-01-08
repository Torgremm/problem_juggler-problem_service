use contracts::SolveResponse;

pub fn solve_size_of_island(data: Vec<Vec<bool>>) -> SolveResponse {
    let total = data
        .iter()
        .map(|row| row.iter().filter(|&&v| v).count())
        .sum::<usize>() as i64;
    if total == 0 {
        return SolveResponse::BadData("No island found".to_string());
    }
    SolveResponse::Solved(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn should_equal(value: SolveResponse, shouldequal: i64) {
        if let SolveResponse::Solved(v) = value {
            assert_eq!(v, shouldequal)
        } else {
            panic!("Expected SizeOfIsland response")
        }
    }

    #[test]
    fn t1() {
        let data = vec![vec![false; 20]; 20];
        let answer = solve_size_of_island(data);
        assert!(answer == SolveResponse::BadData("No island found".to_string()));
    }
    #[test]
    fn t2() {
        let data = vec![
            vec![false, false, false, false, false],
            vec![false, true, false, false, false],
            vec![true, true, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];

        let answer = solve_size_of_island(data);
        should_equal(answer, 3);
    }
    #[test]
    fn t3() {
        let data = vec![
            vec![false, false, false, false, false],
            vec![false, true, false, false, false],
            vec![true, true, false, true, false],
            vec![false, true, false, true, false],
            vec![false, false, false, false, false],
        ];

        let answer = solve_size_of_island(data);
        should_equal(answer, 6);
    }
}
