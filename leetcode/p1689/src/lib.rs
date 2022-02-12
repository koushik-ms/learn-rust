
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars()
            .map(|c| c as i32 - '0' as i32)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_3_for_32() {
        let result = Solution::min_partitions(String::from("32"));
        assert_eq!(result, 3);
    }
    #[test]
    fn test_8_for_382() {
        let result = Solution::min_partitions(String::from("382"));
        assert_eq!(result, 8);
    }
    #[test]
    fn test_8_for_82734() {
        let result = Solution::min_partitions(String::from("82734"));
        assert_eq!(result, 8);
    }
    #[test]
    fn test_9_for_27346209830709182346() {
        let result = Solution::min_partitions(String::from("27346209830709182346"));
        assert_eq!(result, 9);
    }
}
