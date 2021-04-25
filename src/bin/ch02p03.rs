extern crate coding_interview;
use coding_interview::linked_list;

fn main() {
    let mut list1 = linked_list::LinkedList::<String>::new();
    list1.append(String::from("item1"));
    list1.append(String::from("item2"));
    list1.append(String::from("item3"));
    list1.append(String::from("item4"));
    list1.append(String::from("item5"));
     for (n, node) in list1.iter().enumerate() {
        if n == 3 {
            &node.borrow_mut().remove();
        }
    }
}