struct CustomStack {
    the: Vec<(i32,i32)>
}

#[allow(dead_code)]
impl CustomStack {
    fn new(_max_size: i32) -> Self { CustomStack{ the: vec![] } }
    fn push(& mut self, value: i32) { self.the.push((value, 0)) }
    fn pop(& mut self) -> i32 { 
        let (v, i) = self.the.pop().unwrap(); 
        if !self.the.is_empty() { 
            let a = self.the.len() - 1; 
            self.the[a].1 += i;
        }
        v+i 
    }
    fn inc(& mut self, extent: i32, value: i32) { self.the[extent as usize-1].1 += value; }
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
        let mut obj = CustomStack::new(3);
        obj.push(x);
        let ret_2: i32 = obj.pop();
        assert_eq!(x, ret_2);
    }
    #[test]
    fn can_push_and_pop_many() {
        let x = vec![2, 3, 4];
        let mut obj = CustomStack::new(3);
        for i in x.iter() {
            obj.push(*i);
        }
        for j in x.iter().rev(){
            let ret_2: i32 = obj.pop();
            assert_eq!(*j, ret_2);
        }
    }
    #[test]
    fn can_apply_inc_to_all_items_pushed() {
        let x = vec![2, 3, 4];
        let size = 3;
        let inc = 42;
        let mut obj = CustomStack::new(size);
        for i in x.iter() {
            obj.push(*i);
        }
        obj.inc(size, inc);
        for j in x.iter().rev(){
            let ret_2: i32 = obj.pop();
            assert_eq!(*j+inc, ret_2);
        }
    }
    #[test]
    fn can_apply_inc_to_some_items() {
        let items_to_increment = vec![2, 3, 4];
        let incd_size = 3;
        let inc = 42;
        let additional_items = vec![5, 6];
        let addl_size = 2;
        let mut obj = CustomStack::new(incd_size + addl_size);
        for i in items_to_increment.iter() {
            obj.push(*i);
        }
        obj.inc(incd_size, inc);
        for i in additional_items.iter() {
            obj.push(*i);
        }
        for j in additional_items.iter().rev() {
            let ret_2: i32 = obj.pop();
            assert_eq!(*j, ret_2);
        }
        for j in items_to_increment.iter().rev() {
            let ret_2: i32 = obj.pop();
            assert_eq!(*j+inc, ret_2);
        }
    }
}

/*
Test list
✔️ create a stack (push and pop)
✔️ create stack, push apply inc to all and pop
create stack, push apply inc to some elements and pop
push, apply inc to full stack, push more, pop
push, apply inc twice (with diff extents)


honor maxsize - push after passing maxsize will do nothing. 
honor maxsize - inc for > maxsize elems will only act on maxsize elements.
pop on empty stack returhs -1
inc on empty stack has no effect 
*/