fn fizzbuzz(x: u32) -> String {
    let mut result = String::new();
    if x%3 == 0 { result += "Fizz"; }
    if x%5 == 0 { result += "Buzz"; }
    if result.is_empty() { result = x.to_string(); }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn fizzbuzz_1_is_1() {
        assert_eq!("1", fizzbuzz(1));
    }
    #[test]
    fn fizzbuzz_2_is_2() {
        assert_eq!("2", fizzbuzz(2));
    }
    #[test]
    fn fizzbuzz_3_is_Fizz() {
        assert_eq!("Fizz", fizzbuzz(3));
    }
    #[test]
    fn fizzbuzz_5_is_Buzz() {
        assert_eq!("Buzz", fizzbuzz(5));
    }
    #[test]
    fn fizzbuzz_6_is_Fizz() {
        assert_eq!("Fizz", fizzbuzz(6));
    }
    #[test]
    fn fizzbuzz_10_is_Buzz() {
        assert_eq!("Buzz", fizzbuzz(10));
    }
    #[test]
    fn fizzbuzz_15_is_FizzBuzz() {
        assert_eq!("FizzBuzz", fizzbuzz(15));
    }
}
