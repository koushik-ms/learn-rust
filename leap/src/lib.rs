fn is_leap_year(year: u32) -> bool {
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_call_is_leap() {
        assert_eq!(is_leap_year(2000), true);
    }
    #[test]
    fn y1990_is_not_a_leap_year() {
        assert_eq!(is_leap_year(1990), false);
    }
}
