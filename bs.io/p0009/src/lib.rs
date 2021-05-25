use std::cmp::max;

fn solve(n: &[i32]) -> i32 { 
    match n.len() {
        0 => 0,
        1 => max(0, n[0]),
        _ => max(
            0,
            max(n[0]+solve(&n[2..]), solve(&n[1..]))
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn largest_non_adj_sum_positive_numbers() {
        let nums = vec![3, 4, 6, 9, 1];
        assert_eq!(solve(&nums), 13);
    }
    #[test]
    fn largest_non_adj_sum_positive_numbers_skip_two() {
        let nums = vec![6, 0, 2, 5];
        assert_eq!(solve(&nums), 11);
    }
    #[test]
    fn largest_non_adj_sum_negative_numbers() {
        let nums = vec![-3, -4, -6, -9, -1];
        assert_eq!(solve(&nums), 0);
    }
    #[test]
    fn largest_non_adj_sum_positive_and_negative_numbers() {
        let nums = vec![3, 4, 6, -9, 1];
        assert_eq!(solve(&nums), 10);
    }
}
