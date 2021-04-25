use std::cmp;

// 隣接リスト表現

pub struct Edge {
    vertex_to: usize,
    weight: usize,
}

impl Edge {
    pub fn new(to: usize, w: usize) -> Self {
        Edge {
            vertex_to: to,
            weight: w,
        }
    }
}

pub struct Graph {
    vertex_list: Vec<Vec<Edge>>,
}

impl Graph {
    // new: p.195
    pub fn new(size: usize) -> Self {
        let mut graph = Graph {
            vertex_list: Vec::new(),
        };
        for _ in 0..size {
            graph.vertex_list.push(Vec::<Edge>::new());
        }
        graph
    }
    pub fn add_edge(&mut self, vertex: usize, edge: Edge) {
        if self.vertex_list.len() <= vertex {
            println!("error: there is no vertex");
            return
        }
        self.vertex_list[vertex].push(edge);
    }
    pub fn has_a_walk(&self, start_vertex: usize, end_vertex: usize) -> bool {
        let graph_size = self.vertex_list.len();
        let mut seen = vec![false; graph_size];
        // queueにするとBFSになる．
        let mut todo = Vec::<usize>::new();

        seen[start_vertex] = true;
        todo.push(start_vertex);

        while !todo.is_empty() {
            let cur_v = todo.pop().unwrap();

            for edge in self.vertex_list[cur_v].iter() {
                if edge.vertex_to == end_vertex {
                    return true
                }
                if seen[edge.vertex_to] == true {
                    continue;
                }

                seen[edge.vertex_to] = true;
                todo.push(edge.vertex_to);
            }
        }
        false
    }
    pub fn dfs(&self, vertex: usize, seen: &mut Vec<bool>) {
        seen[vertex] = true;

        for edge in self.vertex_list[vertex].iter() {
            if seen[edge.vertex_to] == true {
                continue;
            }
            self.dfs(edge.vertex_to, seen);
        }
    }
    // 4.4
    // 4.7
    pub fn topological_sort(&self, vertex: usize) -> Vec<usize> {
        let mut order = Vec::<usize>::new();
        let mut seen = vec![false; self.vertex_list.len()];
        // 指定の頂点から到達できない頂点がある場合は，頂点でループする．
        self.topological_sort_rec(vertex, &mut seen, &mut order);
        order.reverse();
        order
    }
    fn topological_sort_rec(&self, vertex: usize, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
        seen[vertex] = true;
        for edge in self.vertex_list[vertex].iter() {
            let vertex_to = edge.vertex_to;
            if seen[vertex_to] {
                continue
            }
            self.topological_sort_rec(vertex_to, seen, order);
        }
        order.push(vertex);
    }
    // DAG上の最短路長問題を解くためのメソッド
    // アルゴリズムとデータ構造 10.5.2
    // 根とvとを結ぶパスの長さ: 頂点vの深さ
    // 根の深さは0
    // 根付き木の各頂点の深さの最大値が木の高さ
    // 木の深さも，辺の重さ1とした最短路長問題の解法で求められる．
    // 4.9 深さの浅い順に組み合わせを作り，かけていけば出せそう．
    pub fn dp_shortest_path_len(&self, order: Vec<usize>) -> Vec<usize> {
        let mut table = vec![usize::MAX; self.vertex_list.len()];
        table[order[0]] = 0;
        for idx in 0..order.len() {
            for edge in self.vertex_list[order[idx]].iter() {
                println!("idx: {}, order: {}, table: {}", idx, order[idx], table[order[idx]]);
                table[edge.vertex_to] = cmp::min(table[edge.vertex_to], table[order[idx]] + edge.weight);
            }
        }
        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_a_walk(){
        let mut graph = Graph::new(15);
        for i in 1..6 {
            let edge = Edge::new(i, 1);
            graph.add_edge(0, edge);
        }
        let mut vertex = 1;
        for i in 6..11 {
            let edge = Edge::new(i, 1);
            graph.add_edge(vertex, edge);
            vertex += 1;
        }
        let edge = Edge::new(14, 1);
        graph.add_edge(6, edge);

        for i in 1..11 {
            assert_eq!(graph.has_a_walk(0, i), true);
        }
        assert_eq!(graph.has_a_walk(0, 10), true);
        assert_eq!(graph.has_a_walk(1, 14), true);
        assert_eq!(graph.has_a_walk(0, 12), false);
        assert_eq!(graph.has_a_walk(0, 16), false);
        assert_eq!(graph.has_a_walk(6, 10), false);
    }
    #[test]
    fn test_dp_shortest_path_len() {
        let mut graph = Graph::new(16);
        let data = vec![
            vec![8, 4]
            , vec![8, 12]
            , vec![4, 2]
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
        for from_to in data {
            let edge = Edge::new(from_to[1], 1);
            graph.add_edge(from_to[0], edge)
        }
        let left_order = graph.topological_sort(4);
        let right_order = graph.topological_sort(12);

        let left_dp = graph.dp_shortest_path_len(left_order);
        let right_dp = graph.dp_shortest_path_len(right_order);

        let result = vec![0,2,1,2,0,2,1,2,0,2,1,2,0,2,1,2];

        for i in 1..8 {
            assert_eq!(left_dp[i], result[i]);
        }
        for i in 9..16 {
            assert_eq!(right_dp[i], result[i]);
        }
    }
}