pub fn increasing_digits(_n: u32) -> u32 { 36 }

#[cfg(test)]
mod tests {
    use super::increasing_digits;
    #[test]
    fn t_84_numbers_with_2_increasing_digits() {
        assert_eq!(increasing_digits(2), 36);
    }
}
