/*
   問題: ある数xが与えられたとき、連結リストの要素を並び替え、xより小さいものが前に来るようにするコードを書く。xがリストに含まれる場合、xの値はxより小さい要素の後にある必要がある。区切り要素のxは右半分のどこに現れてもかまわない。
   回答→ src/linked_list.rsのdivide_list()
*/

extern crate coding_interview;
use coding_interview::linked_list;

fn main() {
    let mut list = linked_list::LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
    let _list_partitioned = list.divide_list("hi".to_string());
}
