pub fn increasing_digits(n: u32) -> u32 {
    let (mut num, mut den) = (1, 1);
    for i in 0..n {
        num *= 9-i;
        den *= i+1;
    }
    num/den
}

#[cfg(test)]
mod tests {
    use super::increasing_digits;
    #[test]
    fn t_036_numbers_with_2_increasing_digits() {
        assert_eq!(increasing_digits(2), 36);
    }
    #[test]
    fn t_084_numbers_with_3_increasing_digits() {
        assert_eq!(increasing_digits(3), 84);
    }
    #[test]
    fn t_126_numbers_with_4_increasing_digits() {
        assert_eq!(increasing_digits(4), 126);
    }
}
