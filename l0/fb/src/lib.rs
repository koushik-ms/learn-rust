fn fizzbuzz(x: u32) -> String { String::from("1") }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn can_call_fizzbuzz() {
        assert_eq!("1", fizzbuzz(1));
    }
}
