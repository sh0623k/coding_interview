// 4.8 辺を追加していく中で初めてルートが一致したらそのルートを返す．
// 対象の子から親に上っていくように辺を追加する．
// 4.10 
pub struct UnionFind {
    parent: Vec<Option<usize>>,
    size: Vec<usize>
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![None; n],
            size: vec![1; n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        if self.parent[x].is_none() {
            return x
        }
        else {
            self.parent[x] = Some(self.root(self.parent[x].unwrap()));
            self.parent[x].unwrap()
        }
    }
    pub fn issame(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn unite(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.root(x);
        y = self.root(y);

        if x == y {
            return false
        }

        // union by size
        if self.size[x] < self.size[y] {
            swap(x, y);
        }

        self.parent[y] = Some(x);
        self.size[x] += self.size[y];

        true
    }
    pub fn get_size(&self, x: usize) -> usize {
        self.size[self.root[x]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let uf = UnionFind::new(16);
        let data_from_to = vec![
            vec![4, 2]
            , vec![4, 6]
            , vec![12, 10]
            , vec![12, 14]
            , vec![2, 1]
            , vec![2, 3]
            , vec![6, 5]
            , vec![6, 7]
            , vec![10, 9]
            , vec![10, 11]
            , vec![14, 13]
            , vec![14, 15]
        ];
        for from_to in data_from_to {
            uf.unite(mut from_to[0], mut from_to[1]);
            if uf.root(3) == uf.root(7) {
                assert_eq!(uf.root(3), 4);
            }
        }
    }
}