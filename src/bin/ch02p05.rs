/*
   問題: 各ノードの要素が1桁の数である連結リストで表された2つの数がある。一の位がリストの先頭になるように、各位の数は逆順に並んでいる。このとき2つの数の和を求め、それを連結リストで表したものを返す関数を書く。
*/

extern crate coding_interview;
use coding_interview::linked_list::LinkedList;

fn sum_list(num_list1: LinkedList<i32>, num_list2: LinkedList<i32>) -> LinkedList<i32> {
    let mut result = LinkedList::new();
    let num1 = convert_to_i32(num_list1);
    let num2 = convert_to_i32(num_list2);
    let mut num = num1 + num2;
    while num > 0 {
        result.append(num % 10);
        num /= 10;
    }
    result
}

fn convert_to_i32(list: LinkedList<i32>) -> i32 {
    let mut num = 0;
    let base: i32 = 10;
    let mut digit = 0;
    for node in list.iter() {
        let v = node.borrow().data.clone();
        /*
        if v > 9 {
            return 0
        }
        */
        num += v * base.pow(digit);
        digit += 1;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_list() {
        let mut left = LinkedList::<i32>::new();
        left.append(7);
        left.append(1);
        left.append(6);

        let mut right = LinkedList::<i32>::new();
        right.append(5);
        right.append(9);
        right.append(2);

        let result = sum_list(left, right);
        let mut iter = result.iter();
        assert_eq!(iter.next().unwrap().borrow().data, 2);
        assert_eq!(iter.next().unwrap().borrow().data, 1);
        assert_eq!(iter.next().unwrap().borrow().data, 9);
    }
}

fn main() {
    let mut left = LinkedList::<i32>::new();
    left.append(6);

    let mut right = LinkedList::<i32>::new();
    right.append(2);
    sum_list(left, right);
}
