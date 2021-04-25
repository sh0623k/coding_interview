pub struct Stack<T> {
    arr: Vec<T>,
    top: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            arr: Vec::<T>::new(),
            top: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
    // self.arr.push()である必要はない．topを操作するため．
    // 配列の初期化などでサイズを決めたりしてpushを使わないならtopが必要．
    pub fn push(&mut self, value: T) {
        self.arr.push(value);
        self.top += 1;
    }
    // self.arr.pop()である必要はない．topを操作するため．
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }
        let result = self.arr.pop();
        self.top -= 1;
        result
    }
}

pub struct SetOfStacks<T> {
    stack_set: Vec<Stack<T>>,
    capacity_of_one: usize,
}

impl<T> SetOfStacks<T> {
    pub fn new(capacity_of_one: usize) -> Self {
        SetOfStacks {
            stack_set: Vec::<Stack<T>>::new(),
            capacity_of_one: capacity_of_one,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.stack_set.is_empty()
    }
    pub fn push(&mut self, value: T) {
        if self.stack_set.is_empty() {
            self.stack_set.push(Stack::new());
        }
        if self.stack_set.last().unwrap().top == self.capacity_of_one {
            self.stack_set.push(Stack::new());
        }
        self.stack_set.last_mut().unwrap().push(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }
        let result = self.stack_set.last_mut().unwrap().pop();
        if self.stack_set.last().unwrap().is_empty() {
            self.stack_set.pop();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.is_empty(), true);

        stack.push(0);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.pop().unwrap(), 0);

        stack.push(4);
        stack.push(5);
        stack.push(6);
        stack.push(7);
        stack.push(8);
        stack.push(9);
        assert_eq!(stack.pop().unwrap(), 9);
        assert_eq!(stack.pop().unwrap(), 8);
        assert_eq!(stack.pop().unwrap(), 7);
    }
    #[test]
    fn test_set_of_stacks() {
        let mut set_of_stacks: SetOfStacks<i32> = SetOfStacks::new(10);
        assert_eq!(set_of_stacks.is_empty(), true);
        for i in 0..100 {
            set_of_stacks.push(i);
        }
        for i in (0..100).rev() {
            let res = set_of_stacks.pop().unwrap();
            assert_eq!(i, res);
        }
    }
}