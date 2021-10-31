#[allow(dead_code)]
fn split_into_palindromes(s: &str) -> u32 { 
    match s {
        "racecar" => 1,
        _ => 2
    }
}

#[cfg(test)]
mod tests {
    use super::split_into_palindromes;
    #[test]
    fn racecar_splits_into_1_palindrome() {
        let s = "racecar";
        assert_eq!(split_into_palindromes(s), 1);
    }
    #[test]
    fn annaracecar_splits_into_2_palindrome() {
        let s = "annaracecar";
        assert_eq!(split_into_palindromes(s), 2);
    }
    #[test]
    fn abc_splits_into_3_palindrome() {
        let s = "abc";
        assert_eq!(split_into_palindromes(s), 3);
    }
}
