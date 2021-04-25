extern crate coding_interview;
use coding_interview::queue::*;

fn main() {
    let mut my_queue: Queue<i32> = Queue::new(0, 11);
    for i in 0..10 {
        println!("i: {}", i);
        my_queue.enqueue(i);
    }
    my_queue.enqueue(10);
    for _ in 0..10 {
        println!("{}", my_queue.dequeue().unwrap());
    }
}