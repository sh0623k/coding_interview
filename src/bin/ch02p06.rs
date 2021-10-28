/*
   問題: 連結リストが回文かどうかを調べる関数を実装する。
*/

extern crate coding_interview;
use coding_interview::linked_list::*;

fn is_palindrome(list: LinkedList<i32>) -> bool {
    let mut node_backward;
    let mut iterator_back = list.iter_backward();
    for node_forward in list.iter() {
        node_backward = iterator_back.next();
        if node_forward.borrow().data != node_backward.unwrap().borrow().data {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let mut list1 = LinkedList::<i32>::new();
        list1.append(1);
        list1.append(2);
        list1.append(3);
        list1.append(2);
        list1.append(1);
        assert_eq!(is_palindrome(list1), true);

        let mut list2 = LinkedList::<i32>::new();
        list2.append(1);
        list2.append(2);
        list2.append(3);
        list2.append(2);
        list2.append(1);
        list2.append(1);
        assert_eq!(is_palindrome(list2), false);

        let mut list3 = LinkedList::<i32>::new();
        list3.append(1);
        list3.append(2);
        list3.append(2);
        list3.append(1);
        assert_eq!(is_palindrome(list3), true);

        let mut list4 = LinkedList::<i32>::new();
        list4.append(1);
        list4.append(1);
        list4.append(2);
        list4.append(2);
        assert_eq!(is_palindrome(list4), false);
    }
}

fn main() {
    let mut list = LinkedList::<i32>::new();
    list.append(6);
    is_palindrome(list);
}
