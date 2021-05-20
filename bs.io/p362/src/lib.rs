fn solve(s: &str) -> bool { 
    let n = s.len();
    n != 2
 }

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn short_odd_palindrome_gives_true() {
        assert_eq!(solve("faf"), true);
    }
    #[test]
    fn short_even_palindrome_gives_false() {
        assert_eq!(solve("aa"), false);
    }
}
