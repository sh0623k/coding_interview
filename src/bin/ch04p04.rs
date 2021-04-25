extern crate coding_interview;
use coding_interview::graph::*;

fn main() {
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
    for i in left_dp {
        println!("depth: {}", i);
    }

    let right_dp = graph.dp_shortest_path_len(right_order);
    for i in right_dp {
        println!("depth: {}", i);
    }
}