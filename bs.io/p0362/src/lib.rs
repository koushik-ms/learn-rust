#[allow(dead_code)]
fn solve(s: &str) -> bool { 
    let b = s.as_bytes();
    !(1..s.len()).any(|i| b[i]==b[i-1])
 }

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn short_odd_palindrome_gives_true() {
        assert_eq!(solve("a"), true);
    }
    #[test]
    fn short_even_palindrome_gives_false() {
        assert_eq!(solve("aa"), false);
    }
    #[test]
    fn long_odd_palindrome_gives_true() {
        assert_eq!(solve("faf"), true);
    }
    #[test]
    fn long_even_palindrome_gives_false() {
        assert_eq!(solve("muum"), false);
    }
    #[test]
    fn odd_length_string_with_even_length_palindromic_substring_gives_false() {
        assert_eq!(solve("faa"), false);
    }
    #[test]
    fn even_length_string_with_odd_palindromic_substrings_gives_true() {
        assert_eq!(solve("abcd"), true);
    }
}
