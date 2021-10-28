/*
   問題: 2つのソートされた配列A, Bがあります。Aの配列には十分に空きがあり、後ろにBを追加できる。このとき、BをAにソートされた状態でマージするメソッドを書く。
*/

// 後ろから見ていくと，Aの末尾から設定していけばシフトがいらなくなる．
// これはマージソートのマージ部分．
fn merge_sorted_lists(list1: &mut Vec<usize>, list2: &mut Vec<usize>) -> Vec<usize> {
    list2.reverse();
    list1.append(list2);
    let mut result = vec![];
    let mut idx_left = 0;
    let mut idx_right = list1.len() - 1;
    while idx_left < idx_right {
        if list1[idx_left] < list1[idx_right] {
            result.push(list1[idx_left]);
            idx_left += 1;
        }
        else {
            result.push(list1[idx_right]);
            idx_right -= 1;
        }
    }
    result.push(list1[idx_left]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sorted_lists() {
        let mut list1 = vec![1,3,6,8,9,11];
        let mut list2 = vec![2,4,5,7,9,10];
        let merged_list = merge_sorted_lists(&mut list1, &mut list2);
        assert_eq!(merged_list, vec![1,2,3,4,5,6,7,8,9,9,10,11]);
    }
}

fn main() {
    let mut list1 = vec![1,3,6,8,9,11];
    let mut list2 = vec![2,4,5,7,9,10];
    merge_sorted_lists(&mut list1, &mut list2);
}
