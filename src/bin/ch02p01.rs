extern crate coding_interview;
use coding_interview::linked_list;

fn main() {
    let mut list = linked_list::LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
    list.list_has_duplicates();
    list.remove_duplicates();
}