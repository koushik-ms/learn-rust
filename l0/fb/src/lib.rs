fn fizzbuzz(x: u32) -> String { x.to_string() }

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
}
