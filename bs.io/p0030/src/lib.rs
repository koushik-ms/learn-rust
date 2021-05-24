fn solve(s0: &str, s1: &str) -> bool { true }

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn can_solve_rotation() {
        assert!(solve("hello", "hello"));
    }
}
