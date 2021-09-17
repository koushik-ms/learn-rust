pub fn is_armstrong_number(num: u32) -> bool {
    let digit_count = (num as f64).log10() as u32 + 1;
    let digit_sum: u32 = num.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap().pow(digit_count))
        .sum();
    digit_sum == num
}
