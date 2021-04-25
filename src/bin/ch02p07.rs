/*
extern crate coding_interview;
use coding_interview::linked_list::*;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let datavec = vec![
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
        Rc::new(RefCell::new(3)),
        Rc::new(RefCell::new(4)),
        Rc::new(RefCell::new(5)),
    ];


    let mut intersecting_first = LinkedList::<Rc<i32>>::new();
    for value in datavec.iter() {
        intersecting_first.append(value.clone());
    }

    let mut intersecting_second = LinkedList::<Rc<i32>>::new();
    for value in datavec.iter().take(5).skip(2) {
        intersecting_second.append(value.clone());
    }

    if let Some(tail_self) = intersecting_first.get_tail().as_ref().cloned() {
        if let Some(tail_other) = intersecting_second.get_tail().as_ref().cloned() {
            if !Rc::ptr_eq(&tail_self, &tail_other) {
                println!("tail false");
                println!("{}", tail_self.borrow().data);
                println!("{}", tail_other.borrow().data);
            }
        }
        else {
            println!("fail get tail");
        }
    }
    else {
        println!("fail get tail");
    }
    if let Some(head_self) = intersecting_first.head.as_ref().cloned() {
        if let Some(head_other) = intersecting_second.head.as_ref().cloned() {
            let len_self = Node::get_cnt_to_last(&head_self, 0);
            let len_other = Node::get_cnt_to_last(&head_other, 0);
            if len_self < len_other {
                let mut iter_other = intersecting_second.iter();
                for _ in 0..(len_other - len_self) {
                    iter_other.next();
                }
                for node_self in intersecting_first.iter() {
                    let node_other = iter_other.next().unwrap();
                    if Rc::ptr_eq(&node_self, &node_other) {
                        println!("the same node");
                    }
                }
                
            }
            else {
                let mut iter_self = intersecting_first.iter();
                for _ in 0..(len_self - len_other) {
                    iter_self.next();
                }
                for node_other in intersecting_second.iter() {
                    let node_self = iter_self.next().unwrap();
                    if Rc::ptr_eq(&node_self, &node_other) {
                        println!("the same node");
                    }
                }
                
            }
        }
    }
}
*/
fn main() {

}