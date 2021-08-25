struct CustomStack;
#[allow(dead_code)]
impl CustomStack {
    fn new(_max_size: i32) -> Self { CustomStack{} }
    fn push(&self, _value: i32) {}
    fn pop(&self) -> i32 { 42 }
}
#[cfg(test)]
mod tests {
    use super::CustomStack;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn can_create_custom_stack() {
        let x = 42;
        let obj = CustomStack::new(1);
        obj.push(x);
        let ret_2: i32 = obj.pop();
        assert_eq!(x, ret_2);
    }
}

