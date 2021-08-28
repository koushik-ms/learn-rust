use std::cmp::min;
struct CustomStack {
    the: Vec<(i32,i32)>,
    max_size: usize
}

#[allow(dead_code)]
impl CustomStack {
    fn new(max_size: i32) -> Self { 
        CustomStack{ the: vec![], max_size: max_size as usize } 
    }
    fn push(& mut self, value: i32) { 
        if self.the.len() < self.max_size {
            self.the.push((value, 0)) 
        }
    }
    fn pop(& mut self) -> i32 { 
        if self.the.is_empty() { return -1 }
        let (v, i) = self.the.pop().unwrap(); 
        if !self.the.is_empty() { 
            let a = self.the.len() - 1; 
            self.the[a].1 += i;
        }
        v+i 
    }
    fn inc(& mut self, extent: i32, value: i32) { 
        let extent = min(extent as usize, self.the.len());
        if extent > 0 {
            self.the[extent as usize-1].1 += value; 
        }
    }
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

    #[test]
    fn adjust_extent_to_valid_length() {
        let items_to_increment = [2, 3, 4];
        let size = 3;
        let inc = 42;
        let mut obj = CustomStack::new(size);
        for i in items_to_increment.iter() {
            obj.push(*i);
        }
        obj.inc(5, inc);
        for j in items_to_increment.iter().rev() {
            let ret_2: i32 = obj.pop();
            assert_eq!(*j+inc, ret_2);
        }
    }

    #[test]
    fn ignore_push_after_max_size_is_reached() {
        let size = 3;
        let mut obj = CustomStack::new(size);
        for i in [1, 2].iter() {
            obj.push(*i);
        }
        assert_eq!(2, obj.pop());
        for i in [2, 3, 4].iter() {
            obj.push(*i);
        }
        obj.inc(5, 100);
        obj.inc(2, 100);
        for j in [103, 202, 201, -1].iter() {
            assert_eq!(*j, obj.pop());
        }
    }
    #[test]
    fn ignore_inc_on_empty_stack() {
        let size = 2;
        let mut obj = CustomStack::new(size);
        obj.push(34);
        assert_eq!(34, obj.pop());
        obj.inc(8, 100);
        assert_eq!(-1, obj.pop());
        obj.inc(9, 91);
        obj.push(63);
        assert_eq!(63, obj.pop());
        obj.push(84);
        obj.inc(10, 93);
        obj.inc(6, 45);
        obj.inc(10, 4);
    }
    /*
    ["CustomStack","push","pop","increment","pop","increment","push","pop","push","increment","increment","increment"]
    [[2],[34],[],[8,100],[],[9,91],[63],[],[84],[10,93],[6,45],[10,4]]
    */

}

/*
Test list
✔️ create a stack (push and pop)
✔️ create stack, push apply inc to all and pop
✔️ create stack, push apply inc to some elements and pop
✔️ push, apply inc to full stack, push more, pop
✔️ push, apply inc twice (with diff extents)


✔️ honor maxsize - push after passing maxsize will do nothing. 
✔️ honor maxsize - inc for > maxsize elems will only act on maxsize elements.
✔️ pop on empty stack returhs -1
✔️ inc on empty stack has no effect 
*/