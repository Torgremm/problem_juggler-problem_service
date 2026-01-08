use contracts::SolveResponse;
pub fn solve_largest_window_in_array(data: Vec<i64>) -> SolveResponse {
    log::debug!("Solving largest window in array with: {:?}", data);
    let mut best: i64 = data[0];
    let mut current = data[0];

    for &x in &data[1..] {
        current = x.max(current + x);
        best = best.max(current);
    }

    SolveResponse::Solved(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn should_equal(value: SolveResponse, shouldequal: i64) {
        if let SolveResponse::Solved(v) = value {
            assert_eq!(v, shouldequal)
        } else {
            panic!("Expected response")
        }
    }

    #[test]
    fn t1() {
        let data = vec![-100, 0, 1, 0, 1, 0, 1];
        let best = solve_largest_window_in_array(data);

        should_equal(best, 3);
    }
    #[test]
    fn t2() {
        let data = vec![1, -2, 3, -1, 3, 2, -1];
        let best = solve_largest_window_in_array(data);
        should_equal(best, 7);
    }
    #[test]
    fn t3() {
        let data = vec![
            1637232, -2324324, 232152442, 4242144, -4243133, 2323781, -13231,
        ];
        let best = solve_largest_window_in_array(data);

        should_equal(best, 232152442 + 4242144);
    }
}
