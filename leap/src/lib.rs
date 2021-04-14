fn is_leap_year(year: u32) -> bool {
    year%4 == 0 && (year%100 != 0 || year%400 == 0)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn y1990_is_not_a_leap_year() {
        assert_eq!(is_leap_year(1990), false);
    }
    #[test]
    fn y1996_is_a_leap_year() {
        assert_eq!(is_leap_year(1996), true);
    }
    #[test]
    fn y1900_is_atypical_common_year() {
        assert_eq!(is_leap_year(1900), false);
    }
    #[test]
    fn y2000_is_atypical_leap_year() {
        assert_eq!(is_leap_year(2000), true);
    }
}
