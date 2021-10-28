/*
   問題: ソートされていない連結リストから、重複する要素を削除するコードを書く。発展: もし一次的なバッファが利用できない場合はどうやって解くか。
*/
extern crate coding_interview;
use coding_interview::linked_list;

fn main() {
    let mut list = linked_list::LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
    list.list_has_duplicates();
    list.remove_duplicates();
}
