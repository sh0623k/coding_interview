pub struct Queue<T> {
    arr: Vec<T>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> Queue<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
{
    pub fn new(init: T, size: usize) -> Self {
        Queue {
            arr: vec![init; size],
            head: 0,
            tail: 0,
            size: size,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }
    // size - 1個の要素があるときにfullとしている．
    pub fn is_full(&self) -> bool {
        /*
        println!("head: {}", self.head);
        println!("tail: {}", self.tail);
        */
        self.head == (self.tail + 1) % self.size
    }
    pub fn enqueue(&mut self, value: T) -> bool {
        if self.is_full() {
            println!("error: queue is full.");
            return false
        }
        self.arr[self.tail] = value;
        self.tail += 1;
        if self.tail == self.size {
            self.tail = 0;
        }
        println!("tail: {}", self.tail);
        true
    }
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }
        let value = self.arr[self.head].clone();
        self.head += 1;
        if self.head == self.size {
            self.head = 0;
        }
        println!("head: {}", self.head);
        Some(value)
    }
    pub fn get_len(&self) -> usize {
        if self.tail < self.head {
            self.tail + self.size - self.head
        }
        else {
            self.tail - self.head
        }
    }
    pub fn iter(&self) -> QueueIter<T> {
        println!("iter start: {}", self.head);
        let mut idx = self.head;
        let len = self.get_len();
        let mut arr = Vec::<&T>::new();
        for _ in 0..len {
            arr.push(&self.arr[idx]);
            idx += 1;
            if idx == self.size {
                idx = 0;
            }
        };
        let next = 
            if len > 0 {
                Some(&self.arr[self.head])
            }
            else {
                None
            };
        QueueIter {
            arr: arr,
            next: next,
            idx: 0,
        }
    }
}

// iteratorの実装: p.347
// 標準libが，Iteratorを実装するすべての型に対するIntoIteratorを包括実装しているため，Iteratorを実装すればよい．

pub struct QueueIter<'a, T> {
    pub arr: Vec<&'a T>,
    pub next: Option<&'a T>,
    pub idx: usize,
}

impl<'a, T: 'a> Iterator for QueueIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(cur) = self.next {
            self.idx += 1;
            if self.idx < self.arr.len() {
                self.next = Some(self.arr[self.idx]);
            }
            else {
                self.next = None;
            }
            return Some(cur)
        }
        None
    }
}

/*
意味なかった．別に実装する場合．
impl <'a, T: 'a> QueueIter<'a, T> {
    fn get_next_value(&mut self) -> Option<&'a T> {
        self.idx += 1;
        if self.idx < self.arr.len() {
            // 生存期間パラメータを指定するのは，関数や型を定義するときだけ．: p.105
            return Some(self.arr[(self.idx)])
        }
        None
    }
}
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_queue() {
        let mut my_queue: Queue<i32> = Queue::new(0, 11);
        for i in 0..10 {
            my_queue.enqueue(i);
        }
        assert_eq!(my_queue.enqueue(10), false);
        for i in 0..10 {
            assert_eq!(my_queue.dequeue().unwrap(), i);
        }
        assert_eq!(my_queue.dequeue().is_none(), true);
    }
    #[test]
    fn test_iterator() {
        let mut my_queue: Queue<i32> = Queue::new(0, 11);
        for i in 0..10 {
            my_queue.enqueue(i);
        }
        let mut cnt = 0;
        for value in my_queue.iter() {
            assert_eq!(value, &cnt);
            cnt += 1;
        }
        let mut queue2: Queue<i32> = Queue::new(0, 11);
        for i in 0..10 {
            queue2.enqueue(i);
        }
        for _ in 0..3 {
            queue2.dequeue();
        }
        for i in 10..13 {
            queue2.enqueue(i);
        }
        cnt = 3;
        for value in queue2.iter() {
            assert_eq!(value, &cnt);
            cnt += 1;
        }
    }
}