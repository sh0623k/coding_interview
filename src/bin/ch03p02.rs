// 最小値の状態を持っておけば，最小値をpopしたときに次の状態を表せる．
pub struct StackWithMin<T> {
    arr: Vec<T>,
    top: usize,
    min_arr: Vec<T>,
}

impl<T> StackWithMin<T> 
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
{
    pub fn new() -> Self {
        StackWithMin {
            arr: Vec::<T>::new(),
            top: 0,
            min_arr: Vec::<T>::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
    pub fn push(&mut self, value: T) {
        self.arr.push(value.clone());
        self.top += 1;
        if self.min_arr.is_empty() || self.min_arr.last().unwrap() >= &value {
            self.min_arr.push(value);
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }
        let result = self.arr.pop();
        self.top -= 1;
        if self.min_arr.last().unwrap() == result.as_ref().unwrap() {
            self.min_arr.pop();
        }
        result
    }
    pub fn min(&self) -> Option<&T> {
        if self.is_empty() {
            return None
        }
        self.min_arr.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_with_min() {
        let mut stack_with_min: StackWithMin<i32> = StackWithMin::new();
        stack_with_min.push(3);
        // Deref: p.281
        assert_eq!(*stack_with_min.min().unwrap(), 3);
        stack_with_min.push(2);
        assert_eq!(*stack_with_min.min().unwrap(), 2);
        stack_with_min.push(1);
        assert_eq!(*stack_with_min.min().unwrap(), 1);
        assert_eq!(stack_with_min.pop().unwrap(), 1);
        assert_eq!(*stack_with_min.min().unwrap(), 2);
        assert_eq!(stack_with_min.pop().unwrap(), 2);
        assert_eq!(*stack_with_min.min().unwrap(), 3);
        assert_eq!(stack_with_min.pop().unwrap(), 3);
        assert_eq!(stack_with_min.min().is_none(), true);
    }
}

fn main() {
    let mut stack_with_min: StackWithMin<i32> = StackWithMin::new();
    stack_with_min.push(1);
    println!("{}", stack_with_min.min().unwrap());
    stack_with_min.pop();
}
