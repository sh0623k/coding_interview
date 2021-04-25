// https://github.com/brndnmtthws/cracking-the-coding-interview-rust/blob/master/src/bin/c02p01.rs
// Box: p.73
// Option: http://ytyaru.hatenablog.com/entry/2020/08/24/000000 ,p.98
// type: p.164 型エイリアス
// struct: p.187
// generic struct: p.196
// trait object or generic fn: p.236 複数の型が混じるときと、バイナリサイズを節約するときははtrait object。それ以外はgeneric functionを使うのが〇．
// impl: p.192
// self: p.193
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hasher;
use std::rc::Rc;
use std::cmp::Ordering;
// p.98
// アドレスでの比較．
// use std::ptr::eq;

// Rc, Arc: p.87 所有権の共有
// RefCell: p.199 内部可変性
pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

pub struct LinkedList<T> {
    pub head: Option<NodeRef<T>>,
}

pub struct Node<T> {
    pub data: T,
    pub next: Option<NodeRef<T>>,
    pub prev: Option<NodeRef<T>>,
}

pub struct LinkedListIter<T> {
    pub next: Option<NodeRef<T>>,
}

pub struct LinkedListIterBackward<T> {
    pub prev: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    pub fn get_tail(node: &NodeRef<T>) -> Option<NodeRef<T>> {
        // if let: p127
        // OptionやResultからデータを取り出すのに〇．
        // → matchでもできること．
        // Noneかをif let で確認。whileではなく再帰で処理する。
        // https://doc.rust-lang.org/std/convert/trait.AsRef.html
        if let Some(cur) = node.borrow().next.as_ref().cloned() {
            return Node::get_tail(&cur);
        }
        Some(node.clone())
    }

    pub fn get_cnt_to_last(node: &NodeRef<T>, cnt: i32) -> i32 {
        if let Some(cur) = node.borrow().next.as_ref().cloned() {
            return Node::get_cnt_to_last(&cur, cnt);
        }
        cnt
    }

    // 2.3
    pub fn remove(&mut self) {
        if let Some(prev) = &self.prev {
            if let Some(next) = &self.next {
                next.borrow_mut().prev = Some(prev.clone());
                prev.borrow_mut().next = Some(next.clone());
            } else {
                prev.borrow_mut().next = None;
            }
        }
    }
}

// where: p.234, p.233の図のイメージ
// PartialOrd: p.266
impl<T> LinkedList<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::cmp::PartialOrd,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&mut self, new_value: T) {
        if let Some(tail) = self.get_tail() {
            let prev = Some(tail.clone());
            tail.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev,
            })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev: None,
            })));
        }
    }

    pub fn get_tail(&self) -> Option<NodeRef<T>> {
        if let Some(cur) = self.head.as_ref().cloned() {
            if cur.borrow().next.is_none() {
                return Some(cur);
            } else {
                return Node::get_tail(&cur);
            }
        }
        None
    }

    // 2.2
    // n-kにたどり着くためには、kからnと同じだけ0から進めばよい。
    pub fn get_kth_to_last(&self, k: usize) -> Option<NodeRef<T>> {
        // 変更したいOption型の初期化．
        let mut kth_to_last_node: Option<NodeRef<T>> = None;
        for (c, _) in self.iter().enumerate() {
            // match: p.126
            match k.cmp(&c) {
                Ordering::Equal => kth_to_last_node = self.head.as_ref().cloned(),
                Ordering::Less => kth_to_last_node = kth_to_last_node.unwrap().borrow().next.as_ref().cloned(),
                _ => (),
            }
        }
        kth_to_last_node
    }

    // LinkedListからIteratorを使用する。
    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            next: self.head.as_ref().cloned(),
        }
    }

    pub fn iter_backward(&self) -> LinkedListIterBackward<T> {
        LinkedListIterBackward {
            prev: self.get_tail().as_ref().cloned(),
        }
    }

    pub fn remove_duplicates_from_sorted_list(&mut self) {
        let mut node_buf: Option<NodeRef<T>> = None;
        for node in self.iter() {
            if !node_buf.is_none() && node_buf.unwrap().borrow().data == node.borrow().data {
                &node.borrow_mut().remove();
            }
            node_buf = Some(node);
        }
    } 

    // 2.1
    // 有無を判定して、既存の場合はnode単位でremoveが〇
    // removeも定数時間（のはず）なので、先に判定する必要はないかと思った。
    pub fn remove_duplicates(&mut self) {
        // Hash: p.376
        let mut set: HashSet<u64> = HashSet::new();
        for node in self.iter() {
            let mut hasher = DefaultHasher::new();            
            {
                // ブロックに入れて借用を終わらせておく必要がある。
                let data = &node.borrow().data;
                // p.378
                data.hash(&mut hasher);
            }
            let hash = hasher.finish();
            if set.contains(&hash) {
                node.borrow_mut().remove();
            }
            set.insert(hash);
        }
    }

    // テスト用に
    pub fn list_has_duplicates(&self) -> bool {
        let mut set: HashSet<u64> = HashSet::new();
        for node in self.iter() {
            let data = &node.borrow().data;
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            let hash = hasher.finish();
            if set.contains(&hash) {
                return true;
            }
            set.insert(hash);
        }
        false
    }

    /*
    pub fn remove_duplicates(&mut self) {
        let mut set: HashSet<u64> = HashSet::new();
        for node in self.iter() {
            let mut s = DefaultHasher::new();
            {
                let data = &node.borrow().data;
                data.hash(&mut s);
            }
            let hash = s.finish();
            if set.contains(&hash) {
                node.borrow_mut().remove();
            }
            set.insert(hash);
        }
    }
    */
    // 2.4
    pub fn divide_list(&self, partition_value: T) -> LinkedList<T> {
        let mut divided_list = LinkedList::new();
        let mut over_or_eq_p_value_list: Vec<T> = vec![];
        for node in self.iter() {
            let data = node.borrow().data.clone();
            if data < partition_value {
                divided_list.append(data);
            }
            else {
                over_or_eq_p_value_list.push(data);
            }
        }
        for value in over_or_eq_p_value_list.drain(..) {
            divided_list.append(value);
        }
        divided_list
    }
    /*
    型を合わせるためch02p07に移した方がよさそう．
    // 2.7
    // 合流すれば末尾まで同じなので，末尾から同じ長さの分だけ比較すればいい．
    // 共通のノードを返す部分は省く
    pub fn do_lists_intersect(&self, other: &Self) -> bool {
        if let Some(tail_self) = self.get_tail().as_ref().cloned() {
            if let Some(tail_other) = other.get_tail().as_ref().cloned() {
                if !Rc::ptr_eq(&tail_self, &tail_other) {
                    return false
                }
            }
            else {
                return false
            }
        }
        else {
            return false
        }
        if let Some(head_self) = self.head.as_ref().cloned() {
            if let Some(head_other) = other.head.as_ref().cloned() {
                let len_self = Node::get_cnt_to_last(&head_self, 0);
                let len_other = Node::get_cnt_to_last(&head_other, 0);
                if len_self < len_other {
                    let mut iter_other = other.iter();
                    for _ in 0..(len_other - len_self) {
                        iter_other.next();
                    }
                    for node_self in self.iter() {
                        let node_other = iter_other.next().unwrap();
                        if Rc::ptr_eq(&node_self, &node_other) {
                            return true
                        }
                    }
                    
                }
                else {
                    let mut iter_self = self.iter();
                    for _ in 0..(len_self - len_other) {
                        iter_self.next();
                    }
                    for node_other in other.iter() {
                        let node_self = iter_self.next().unwrap();
                        if Rc::ptr_eq(&node_self, &node_other) {
                            return true
                        }
                    }
                    
                }
            }
        }
        false
    }
    */
}

// original Iterator: p.346
// 連続と限界値を設定するためにimpl
// lifetime param: p.102, p.234
// 'a: tick a
impl<'a, T> Iterator for LinkedListIter<T> {
    type Item = NodeRef<T>;

    // Iteratorパターン。次の値を設定して今の値を返す。
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = cur.borrow().next.clone();
            return Some(cur);
        }
        None
    }
}

impl<'a, T> Iterator for LinkedListIterBackward<T> {
    type Item = NodeRef<T>;

    // Iteratorパターン。次の値を設定して今の値を返す。
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.prev.as_ref().cloned() {
            self.prev = cur.borrow().prev.clone();
            return Some(cur);
        }
        None
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        // ?: p.147
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

// cargo test xxx: p.172
// xxxが名前に含まれているテストのみ実行
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_has_duplicates() {
        let mut list1 = LinkedList::<String>::new();
        list1.append(String::from("item1"));
        list1.append(String::from("item2"));

        assert_eq!(list1.list_has_duplicates(), false);

        let mut list2 = LinkedList::<String>::new();
        list2.append(String::from("item1"));
        list2.append(String::from("item"));
        list2.append(String::from("item2"));
        list2.append(String::from("item"));

        assert_eq!(list2.list_has_duplicates(), true);
    }

    #[test]
    fn test_list_remove_duplicates() {
        let mut list1 = LinkedList::<String>::new();
        list1.append(String::from("item1"));
        list1.append(String::from("item2"));

        list1.remove_duplicates();
        assert_eq!(list1.list_has_duplicates(), false);

        let mut list2 = LinkedList::<String>::new();
        list2.append(String::from("item1"));
        list2.append(String::from("item1"));
        list2.append(String::from("item"));
        list2.append(String::from("item2"));
        list2.append(String::from("item"));
        list2.append(String::from("item3"));
        list2.append(String::from("item"));

        assert_eq!(list2.list_has_duplicates(), true);

        list2.remove_duplicates();

        assert_eq!(list2.list_has_duplicates(), false);
    }
    
    #[test]
    fn test_get_kth_to_last_element() {
        let mut list1 = LinkedList::<String>::new();
        list1.append(String::from("item1"));
        list1.append(String::from("item2"));
        list1.append(String::from("item3"));
        list1.append(String::from("item4"));
        list1.append(String::from("item5"));

        assert_eq!(
            list1.get_kth_to_last(1).unwrap().borrow().data,
            String::from("item4")
        );
        assert_eq!(
            list1.get_kth_to_last(2).unwrap().borrow().data,
            String::from("item3")
        );
    }
    #[test]
    fn test_remove_node() {
        let mut list1 = LinkedList::<String>::new();
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

        for node in list1.iter() {
            assert_ne!(node.borrow().data, "item4");
        }
    }
    #[test]
    fn test_partition() {
        let mut list1 = LinkedList::<i32>::new();
        list1.append(3);
        list1.append(5);
        list1.append(8);
        list1.append(5);
        list1.append(10);
        list1.append(2);
        list1.append(1);

        let list_partitioned = list1.divide_list(5);
        let mut iter = list_partitioned.iter();
        assert_eq!(iter.next().unwrap().borrow().data, 3);
        assert_eq!(iter.next().unwrap().borrow().data, 2);
        assert_eq!(iter.next().unwrap().borrow().data, 1);
        assert_eq!(iter.next().unwrap().borrow().data, 5);
        assert_eq!(iter.next().unwrap().borrow().data, 8);
        assert_eq!(iter.next().unwrap().borrow().data, 5);
        assert_eq!(iter.next().unwrap().borrow().data, 10);
    }
    /*
    #[test]
    fn test_do_lists_intersect() {
        let datavec = vec!['A', 'B', 'C', 'D', 'E'];

        let mut intersecting_first = LinkedList::<char>::new();
        for value in datavec.iter() {
            intersecting_first.append(*value);
        }

        let mut intersecting_second = LinkedList::<char>::new();
        for value in datavec.iter().take(5).skip(2) {
            intersecting_second.append(*value);
        }

        assert_eq!(
            intersecting_first.do_lists_intersect(&intersecting_second),
            true
        );

        let mut nonintersecting_first = LinkedList::<char>::new();
        for value in datavec.iter() {
            nonintersecting_first.append(*value);
        }

        let mut nonintersecting_second = LinkedList::<char>::new();
        for value in datavec.iter().take(2).skip(3) {
            nonintersecting_second.append(*value);
        }

        assert_eq!(
            nonintersecting_first.do_lists_intersect(&nonintersecting_second),
            false
        );
    }
    */
}