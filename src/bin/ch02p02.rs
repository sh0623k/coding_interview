/*
   問題: 単方向連結リストにおいて、末尾から数えてk番目の要素を見つけるアルゴリズムを実装する。→ src/linked_list.rsのget_kth_to_last()が回答。
*/

extern crate coding_interview;
use coding_interview::linked_list;

fn main() {
    let mut list = linked_list::LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
    list.append(String::from("item3"));
    list.append(String::from("item4"));
    list.append(String::from("item5"));

}
