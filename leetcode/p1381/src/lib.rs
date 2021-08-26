struct CustomStack {
    the: Vec<i32>
}

#[allow(dead_code)]
impl CustomStack {
    fn new(_max_size: i32) -> Self { CustomStack{ the: vec![] } }
    fn push(& mut self, value: i32) { self.the.push(value) }
    fn pop(& mut self) -> i32 { self.the.pop().unwrap() }
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
        let mut obj = CustomStack::new(1);
        obj.push(x);
        let ret_2: i32 = obj.pop();
        assert_eq!(x, ret_2);
    }
    #[test]
    fn can_push_and_pop_many() {
        let x = vec![2, 3, 4];
        let mut obj = CustomStack::new(1);
        for i in x.iter() {
            obj.push(*i);
        }
        for j in x.iter().rev(){
            let ret_2: i32 = obj.pop();
            assert_eq!(*j, ret_2);
        }
    }
}

/*
Test list
create a stack (push and pop)
create stack, push apply inc to all and pop
create stack, push apply inc to some elements and pop
push, apply inc to full stack, push more, pop
push, apply inc twice (with diff extents)


pop on empty stack returhs -1
inc on empty stack has no effect 
*/