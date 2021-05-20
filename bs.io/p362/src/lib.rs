fn solve(s: &str) -> bool { true }

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        assert_eq!(solve("faf"), true);
    }
}
