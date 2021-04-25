extern crate coding_interview;
use coding_interview::tree::*;

fn main() {
    let arr: Vec<i32> = (0..15).collect();
    let mut mbst = MinimalBinarySearchTree::<i32>::new(&arr);
    for node in mbst.nodes {
        if node.is_some() {
            let some_node = node.unwrap();
            println!("data: {}, left: {}, right: {}", some_node.data, some_node.left.unwrap_or(&arr.len() + 1), some_node.right.unwrap_or(&arr.len() + 1));
        }
    }
    println!("root: {}", mbst.root_idx);
}