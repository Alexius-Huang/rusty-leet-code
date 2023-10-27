/**
 *  Problem 155. Min Stack (Medium)
 *  See: https://leetcode.com/problems/min-stack/
 *  Design a stack that supports push, pop, top, and retrieving the
 *  minimum element in constant time.
 *
 *  Implement the MinStack class:
 *  
 *  - MinStack::new() initializes the stack object.
 *  - push(val: i32) pushes the element val onto the stack.
 *  - pop() removes the element on the top of the stack.
 *  - top() -> i32 gets the top element of the stack.
 *  - getMin() -> i32 retrieves the minimum element in the stack.
 *  
 *  You must implement a solution with O(1) time complexity for each function.
 *
 */

pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        if self.stack.len() == 0 {
            self.stack.push((val, val));
            return;
        }

        let (_, min) = self.top_data();

        self.stack.push((val, if val < min { val } else { min }));
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.top_data().0
    }

    fn top_data(&self) -> (i32, i32) {
        self.stack[self.stack.len() - 1]
    }

    pub fn get_min(&self) -> i32 {
        self.top_data().1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_creates_min_stack_and_performs_different_operations() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // return -3
        min_stack.pop();
        assert_eq!(min_stack.top(), 0); // return 0
        assert_eq!(min_stack.get_min(), -2); // return -2
    }
}
