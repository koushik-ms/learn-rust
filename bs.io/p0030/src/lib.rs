fn solve(s0: &str, s1: &str) -> bool {
    (s0.to_owned() + s0).contains(s1)
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn hello_is_rotation_of_itself() {
        assert!(solve("hello", "hello"));
    }
    #[test]
    fn llohe_is_rotation_of_hello() {
        assert!(solve("llohe", "hello"));
    }
    #[test]
    fn tap_is_no_rotation_of_pat() {
        assert!(!solve("Pat", "taP"));
    }
}
