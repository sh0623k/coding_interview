// binary search
// midが次の範囲の内側にある場合は，midまでを範囲として調べればよい．→ mid - 1, mid + 1の部分を変えればよい．
// 異なる整数でない場合は，左右どちらも調べる必要がある．
fn find_magic_index(sorted_nums: &[isize]) -> Option<usize> {
    if sorted_nums.len() < 1 {
        return None
    }
    find_magic_index_with_range(sorted_nums, 0, sorted_nums.len() - 1)
}

fn find_magic_index_with_range(sorted_nums: &[isize], start: usize, end: usize) -> Option<usize> {
    if start > end {
        return None
    }
    let mid = (start + end) / 2;
    //println!("num: {}, mid: {}", sorted_nums[mid], mid);
    // usizeをisizeと比較したい場合は，isizeにキャストする．
    if sorted_nums[mid] > mid as isize {
        if mid < 1 {
            None
        }
        else {
            find_magic_index_with_range(sorted_nums, start, mid - 1)
        }
    }
    else if sorted_nums[mid] < mid as isize {
        find_magic_index_with_range(sorted_nums, mid + 1, end)
    }
    else {
        Some(mid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_index() {
        assert_eq!(find_magic_index(&[0, 2, 3, 5]), Some(0));
        assert_eq!(find_magic_index(&[-10, 2, 3, 5]), None);
    }
}

fn main() {
    find_magic_index(&[0, 2, 3, 5]);
}