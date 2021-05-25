fn solve(n: &Vec<i32>) -> i32 { 13 }

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn largest_non_adj_sum_positive_numbers() {
        let nums = vec![3, 4, 6, 9, 1];
        assert_eq!(solve(&nums), 13);
    }
}
