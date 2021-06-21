use std::cmp::max;

#[allow(dead_code)]
fn solve(n: &[i32]) -> i32 { 
    let l = n.len();
    let mut ans = Vec::with_capacity(l+1);
    for i in 0..=l {
        match i {
            0 => ans.push(0),
            1 => ans.push(max(0, n[0])),
            _ => ans.push(max(
                0,
                max(ans[i-2] + n[i-1], ans[i-1])
            )),
        };
    }
    ans[l]
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
