// 4.2
// derive: p.189
#[derive(Copy, Clone)]
pub struct MinimalBinarySearchTreeNode<T> {
    pub data: T,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl<T> MinimalBinarySearchTreeNode<T> {
    pub fn new(data: T) -> Self {
        MinimalBinarySearchTreeNode {
            data: data,
            left: None,
            right: None,
        }
    }
}

pub struct MinimalBinarySearchTree<T> {
    pub root_idx: usize,
    pub nodes: Vec<Option<MinimalBinarySearchTreeNode<T>>>,
}

impl<T> MinimalBinarySearchTree<T> 
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::marker::Copy,
{
    pub fn new(arr: &Vec<T>) -> Self {
        let mut mbst = MinimalBinarySearchTree {
            root_idx: (arr.len() - 1) / 2,
            nodes: vec![None; arr.len()],
        };
        mbst.create_minimal_bst(arr);
        mbst
    }
    fn create_minimal_bst(&mut self, arr: &Vec<T>) {
        self.create_minimal_bst_with_range(&arr, 0, arr.len() - 1);
    }
    fn create_minimal_bst_with_range(&mut self, arr: &Vec<T>, start: usize, end: usize) -> Option<usize> {
        if end < start {
            return None
        }
        let mid = (start + end) / 2;
        let mut node = MinimalBinarySearchTreeNode::new(arr[mid]);
        if mid >= 1 {
            node.left = self.create_minimal_bst_with_range(&arr, start, mid - 1);
        }
        node.right = self.create_minimal_bst_with_range(&arr, mid + 1, end);
        self.nodes[mid] = Some(node);
        Some(mid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_tree() {
        let arr: Vec<i32> = (0..15).collect();
        let mbst = MinimalBinarySearchTree::<i32>::new(&arr);
        let result: Vec<Vec<Option<usize>>> = vec![
            vec![None, None],
            vec![Some(0), Some(2)],
            vec![None, None],
            vec![Some(1), Some(5)],
            vec![None, None],
            vec![Some(4), Some(6)],
            vec![None, None],
            vec![Some(3), Some(11)],
            vec![None, None],
            vec![Some(8), Some(10)],
            vec![None, None],
            vec![Some(9), Some(13)],
            vec![None, None],
            vec![Some(12), Some(14)],
            vec![None, None],
        ];
        for i in 0..15 {
            assert_eq!(mbst.nodes[i].unwrap().left, result[i][0]);
            assert_eq!(mbst.nodes[i].unwrap().right, result[i][1]);
        }
    }
}