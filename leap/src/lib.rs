fn is_leap_year(year: u32) -> bool {
    year%4 == 0 && (year%100 != 0 || year%400 == 0)
}
#[cfg(test)]
mod tests {
    use super::is_leap_year;
    #[test]
    fn test_typical_common_year() {
        assert_eq!(is_leap_year(1990), false);
    }
    #[test]
    fn test_typical_leap_year() {
        assert_eq!(is_leap_year(1996), true);
    }
    #[test]
    fn test_atypical_common_year() {
        assert_eq!(is_leap_year(1900), false);
    }
    #[test]
    fn test_atypical_leap_year() {
        assert_eq!(is_leap_year(2000), true);
    }
}
